import math

EPS = 1e-4


def _field_names(obj):
    df = getattr(type(obj), "__dataclass_fields__", None)
    return list(df.keys()) if df else []


def duck(obj):
    """Foreign structural clone: same __name__ + same field attrs, different class."""
    names = _field_names(obj)
    assert names, f"{type(obj).__name__} exposes no __dataclass_fields__ to clone"
    foreign = type(type(obj).__name__, (), {})()
    for n in names:
        setattr(foreign, n, getattr(obj, n))
    return foreign


def _v(a):
    return [a.x, a.y, a.z]


def _approx(a, b, eps=EPS):
    return abs(a - b) <= eps


def _vapprox(a, b, eps=EPS):
    return _approx(a.x, b.x, eps) and _approx(a.y, b.y, eps) and _approx(a.z, b.z, eps)


def _vis(v, x, y, z):
    assert v.x == x and v.y == y and v.z == z, (
        f"got ({v.x}, {v.y}, {v.z}), want ({x}, {y}, {z})"
    )


def _vnear(v, x, y, z, eps=EPS):
    assert _approx(v.x, x, eps) and _approx(v.y, y, eps) and _approx(v.z, z, eps), (
        f"got ({v.x}, {v.y}, {v.z}), want ({x}, {y}, {z}) within {eps}"
    )


def _cross(a, b):
    return (
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    )


def expect(exc, fn, what):
    try:
        fn()
    except exc:
        return
    except Exception as e:  # noqa: BLE001
        raise AssertionError(
            f"{what}: expected {exc.__name__}, got {type(e).__name__}: {e}"
        )
    raise AssertionError(f"{what}: expected {exc.__name__}, nothing raised")


def check_getnewargs_roundtrip(obj, same):
    """__getnewargs_ex__ must return ((), {'__pickle_state__': bytes}) and rebuild."""
    args, kwargs = obj.__getnewargs_ex__()
    name = type(obj).__name__
    assert args == (), f"{name}: __getnewargs_ex__ args not empty: {args!r}"
    assert set(kwargs) == {"__pickle_state__"}, (
        f"{name}: __getnewargs_ex__ kwargs keys {set(kwargs)}"
    )
    assert isinstance(kwargs["__pickle_state__"], bytes)
    rebuilt = type(obj)(*args, **kwargs)
    assert same(rebuilt), f"{name}: pickle-state rebuild differs from original"


def decode_invariance(native_a, native_b):
    """abs_diff_eq must give the same verdict for a native arg and its duck clone.

    Equal pair -> True for both native and duck; distinct pair -> False for both.
    A wrong FromPyObject decode of the duck makes the duck verdict diverge.
    """
    da = duck(native_a)
    db = duck(native_b)

    self_native = native_a.abs_diff_eq(native_a, EPS)
    self_duck = native_a.abs_diff_eq(da, EPS)
    assert self_native is True, f"{type(native_a).__name__}: native self-eq false"
    assert self_duck == self_native, (
        f"{type(native_a).__name__}: duck self-eq diverged ({self_duck} != {self_native})"
    )

    diff_native = native_a.abs_diff_eq(native_b, EPS)
    diff_duck = native_a.abs_diff_eq(db, EPS)
    assert diff_native is False, (
        f"{type(native_a).__name__}: distinct pair not distinct"
    )
    assert diff_duck == diff_native, (
        f"{type(native_a).__name__}: duck distinct-eq diverged ({diff_duck} != {diff_native})"
    )


def check_interval():
    a = Interval(0.0, 1.0)
    b = Interval(0.5, 3.0)
    assert _approx(a.min, 0.0) and _approx(a.max, 1.0)
    decode_invariance(a, b)

    d = duck(a)
    rebuilt = Interval(d.min, d.max)
    assert _approx(rebuilt.min, a.min) and _approx(rebuilt.max, a.max)
    assert a.abs_diff_eq(rebuilt, EPS) is True


def check_nearest():
    curve = Polyline([[0, 0, 0], [4, 0, 0], [4, 4, 0]])
    na = curve.nearest([1.0, 2.0, 0.0])
    nb = curve.nearest([4.0, 1.0, 1.0])

    d = duck(na)
    assert _approx(d.t, na.t)
    assert _vapprox(d.point, na.point)
    assert _approx(d.dist_sq, na.dist_sq)

    rebuilt = Nearest(na.t, _v(na.point), na.dist_sq)
    assert _approx(rebuilt.t, na.t)
    assert _vapprox(rebuilt.point, na.point)
    assert _approx(rebuilt.dist_sq, na.dist_sq)
    assert na.abs_diff_eq(rebuilt, EPS) is True

    decode_invariance(na, nb)


def check_quadratic_bezier():
    a = QuadraticBezier([0, 0, 0], [1, 2, 0], [2, 0, 0])
    b = QuadraticBezier([0, 0, 1], [1, -2, 0], [3, 1, 0])
    assert len(a.points) == 3
    decode_invariance(a, b)

    d = duck(a)
    rebuilt = QuadraticBezier(*[_v(p) for p in d.points])
    assert a.abs_diff_eq(rebuilt, EPS) is True


def check_cubic_bezier():
    a = CubicBezier([0, 0, 0], [1, 2, 0], [2, 2, 0], [3, 0, 0])
    b = CubicBezier([0, 1, 0], [1, -2, 0], [2, 3, 1], [4, 0, 0])
    assert len(a.points) == 4
    decode_invariance(a, b)

    d = duck(a)
    rebuilt = CubicBezier(*[_v(p) for p in d.points])
    assert a.abs_diff_eq(rebuilt, EPS) is True


def check_polyline():
    a = Polyline([[0, 0, 0], [1, 0, 0], [1, 1, 0], [2, 1, 0]])
    b = Polyline([[0, 0, 0], [2, 0, 0], [2, 2, 0]])
    assert len(a.points) == 4
    decode_invariance(a, b)

    d = duck(a)
    rebuilt = Polyline([_v(p) for p in d.points])
    assert a.abs_diff_eq(rebuilt, EPS) is True
    assert len(rebuilt.points) == len(a.points)


def check_spline():
    a = Spline([[0, 0, 0], [1, 1, 0], [2, 0, 0], [3, 1, 0], [4, 0, 0]])
    b = Spline([[0, 0, 0], [1, -1, 0], [2, 1, 0], [3, -1, 0], [4, 0, 0]])
    assert len(a.points) == 5
    decode_invariance(a, b)

    d = duck(a)
    rebuilt = Spline([_v(p) for p in d.points])
    assert a.abs_diff_eq(rebuilt, EPS) is True
    assert len(rebuilt.points) == len(a.points)


def check_curve_common(curve):
    """Exercise the shared curve surface with relation-based assertions."""
    name = type(curve).__name__

    d = curve.domain()
    assert d.min == 0.0 and d.max == 1.0, f"{name}: domain not [0, 1]"

    start, end = curve.endpoints()
    assert _vapprox(start, curve.point(0.0), 1e-6), f"{name}: endpoints[0] != point(0)"
    assert _vapprox(end, curve.point(1.0), 1e-6), f"{name}: endpoints[1] != point(1)"
    assert _vapprox(curve.point_clamped(-2.0), start, 1e-6), f"{name}: point_clamped low"
    assert _vapprox(curve.point_clamped(3.0), end, 1e-6), f"{name}: point_clamped high"

    v = curve.velocity(0.3)
    speed = math.sqrt(v.x * v.x + v.y * v.y + v.z * v.z)
    assert speed > 1e-6, f"{name}: velocity(0.3) vanished"
    tg = curve.tangent(0.3)
    tlen = math.sqrt(tg.x * tg.x + tg.y * tg.y + tg.z * tg.z)
    assert _approx(tlen, 1.0, 1e-5), f"{name}: tangent not unit ({tlen})"
    _vnear(tg, v.x / speed, v.y / speed, v.z / speed, 1e-4)

    n = curve.normal(0.3)
    b = curve.binormal(0.3)
    cx, cy, cz = _cross((tg.x, tg.y, tg.z), (n.x, n.y, n.z))
    _vnear(b, cx, cy, cz, 1e-4)

    acc = curve.acceleration(0.3)
    kx, ky, kz = _cross((v.x, v.y, v.z), (acc.x, acc.y, acc.z))
    k_expected = math.sqrt(kx * kx + ky * ky + kz * kz) / speed**3
    assert _approx(curve.curvature(0.3), k_expected, 1e-3), f"{name}: curvature(0.3)"

    ts = (0.0, 0.35, 0.7, 1.0)
    off = Vec3(1.0, -2.0, 3.0)
    rot = Mat3.from_rotation_z(math.pi / 2)
    moved = curve.translated(off)
    doubled = curve.scaled(2.0)
    rot_m = curve.rotated_mat(rot)
    rot_q = curve.rotated_quat(Quat.from_rotation_z(math.pi / 2))
    placed = curve.transformed(Affine3(translation=off, rotation=rot))
    for t in ts:
        p = curve.point(t)
        _vnear(moved.point(t), p.x + 1.0, p.y - 2.0, p.z + 3.0, 1e-5)
        _vnear(doubled.point(t), 2.0 * p.x, 2.0 * p.y, 2.0 * p.z, 1e-5)
        _vnear(rot_m.point(t), -p.y, p.x, p.z, 1e-4)
        _vnear(rot_q.point(t), -p.y, p.x, p.z, 1e-4)
        _vnear(placed.point(t), -p.y + 1.0, p.x - 2.0, p.z + 3.0, 1e-4)

    mid = curve.point(0.5)
    sub = curve.subcurve(0.25, 0.75)
    assert _vapprox(sub.point(0.0), curve.point(0.25), 1e-4), f"{name}: subcurve start"
    assert _vapprox(sub.point(1.0), curve.point(0.75), 1e-4), f"{name}: subcurve end"
    rev = curve.reversed()
    assert _vapprox(rev.point(0.0), end, 1e-5), f"{name}: reversed start"
    assert _vapprox(rev.point(1.0), start, 1e-5), f"{name}: reversed end"
    tail = curve.truncate_start(0.5)
    assert _vapprox(tail.point(0.0), mid, 1e-4), f"{name}: truncate_start start"
    assert _vapprox(tail.point(1.0), end, 1e-4), f"{name}: truncate_start end"
    head = curve.truncate_end(0.5)
    assert _vapprox(head.point(0.0), start, 1e-4), f"{name}: truncate_end start"
    assert _vapprox(head.point(1.0), mid, 1e-4), f"{name}: truncate_end end"
    first, second = curve.split_at(0.5)
    assert _vapprox(first.point(0.0), start, 1e-4), f"{name}: split_at first start"
    assert _vapprox(first.point(1.0), mid, 1e-4), f"{name}: split_at first end"
    assert _vapprox(second.point(0.0), mid, 1e-4), f"{name}: split_at second start"
    assert _vapprox(second.point(1.0), end, 1e-4), f"{name}: split_at second end"

    assert curve.arc_length_to(0.0) == 0.0, f"{name}: arc_length_to(0) != 0"
    total = curve.length()
    assert total > 0.0, f"{name}: zero length"
    assert _approx(curve.arc_length_to(1.0), total, 1e-4 * max(total, 1.0)), (
        f"{name}: arc_length_to(1) != length()"
    )
    assert curve.t_at_distance(0.0) == 0.0, f"{name}: t_at_distance(0) != 0"
    assert _approx(curve.t_at_distance(total), 1.0, 1e-3), f"{name}: t_at_distance(L)"
    assert _vapprox(curve.point_at_distance(0.0), start, 1e-5), (
        f"{name}: point_at_distance(0)"
    )
    assert _vapprox(curve.point_at_distance(total), end, 1e-3), (
        f"{name}: point_at_distance(L)"
    )

    q = [0.1, -0.7, 0.4]
    nr = curve.nearest(q)
    assert _vapprox(nr.point, curve.point(nr.t), 1e-4), f"{name}: nearest.point/t"
    dx, dy, dz = q[0] - nr.point.x, q[1] - nr.point.y, q[2] - nr.point.z
    assert _approx(dx * dx + dy * dy + dz * dz, nr.dist_sq, 1e-4), (
        f"{name}: nearest.dist_sq inconsistent"
    )
    assert _approx(curve.distance_sq(q), nr.dist_sq, 1e-6), f"{name}: distance_sq"
    assert _approx(curve.distance(q), math.sqrt(nr.dist_sq), 1e-6), f"{name}: distance"
    assert _approx(nr.distance(), math.sqrt(nr.dist_sq), 1e-9)

    pts = curve.points
    cps = curve.control_points()
    assert len(pts) == len(cps), f"{name}: control_points()/points length"
    for a, b2 in zip(pts, cps):
        _vis(a, b2.x, b2.y, b2.z)

    bb = curve.aabb()
    c = bb.center
    he = bb.half_extents
    for p in pts:
        assert abs(p.x - c.x) <= he[0] + 1e-4, f"{name}: aabb misses x of {p!r}"
        assert abs(p.y - c.y) <= he[1] + 1e-4, f"{name}: aabb misses y of {p!r}"
        assert abs(p.z - c.z) <= he[2] + 1e-4, f"{name}: aabb misses z of {p!r}"

    check_getnewargs_roundtrip(curve, lambda rb: curve.abs_diff_eq(rb, 1e-6) is True)


def check_interval_values():
    u = Interval.unit()
    assert u.min == 0.0 and u.max == 1.0
    assert u.span() == 1.0
    assert u.is_finite() is True

    al = Interval.all()
    assert al.min == float("-inf") and al.max == float("inf")
    assert al.is_finite() is False
    assert al.contains(1e30) is True

    iv = Interval(2.0, 6.0)
    assert iv.min == 2.0 and iv.max == 6.0
    assert iv.span() == 4.0
    assert iv.clamp(1.0) == 2.0
    assert iv.clamp(7.5) == 6.0
    assert iv.clamp(3.25) == 3.25
    assert iv.lerp(0.0) == 2.0
    assert iv.lerp(0.5) == 4.0
    assert iv.lerp(1.0) == 6.0
    assert iv.contains(2.0) is True and iv.contains(6.0) is True
    assert iv.contains(1.75) is False and iv.contains(6.25) is False
    assert repr(iv) == "Interval(min=2, max=6)"

    kwd = Interval(min=2.0, max=6.0)
    assert kwd.min == 2.0 and kwd.max == 6.0
    assert iv.abs_diff_eq(kwd, 1e-9) is True
    assert iv.abs_diff_eq(Interval(2.0005, 6.0), 1e-3) is True
    assert iv.abs_diff_eq(Interval(2.01, 6.0), 1e-3) is False

    check_getnewargs_roundtrip(iv, lambda rb: rb.min == 2.0 and rb.max == 6.0)

    expect(ValueError, lambda: Interval(), "Interval()")
    expect(ValueError, lambda: Interval(1.0), "Interval(min only)")
    expect(ValueError, lambda: Interval(max=3.0), "Interval(max only)")
    expect(TypeError, lambda: Interval(0.0, 1.0, 2.0), "Interval 3 positionals")
    expect(TypeError, lambda: Interval(0.0, 1.0, lo=2.0), "Interval unknown kwarg")


def check_nearest_values():
    curve = Polyline([[0, 0, 0], [4, 0, 0], [4, 4, 0]])
    n = curve.nearest([1.0, 2.0, 0.0])
    assert n.t == 0.125, f"nearest t {n.t} != 0.125"
    _vis(n.point, 1.0, 0.0, 0.0)
    assert n.dist_sq == 4.0
    assert n.distance() == 2.0
    assert repr(n).startswith("Nearest(t=")

    kwd = Nearest(t=0.5, point=[1.0, 2.0, 3.0], dist_sq=0.25)
    assert kwd.t == 0.5
    _vis(kwd.point, 1.0, 2.0, 3.0)
    assert kwd.dist_sq == 0.25
    assert kwd.distance() == 0.5
    assert kwd.abs_diff_eq(Nearest(0.5, [1.0, 2.0, 3.0], 0.25), 1e-9) is True

    check_getnewargs_roundtrip(
        kwd, lambda rb: rb.t == 0.5 and rb.dist_sq == 0.25 and rb.point.z == 3.0
    )

    expect(ValueError, lambda: Nearest(), "Nearest()")
    expect(ValueError, lambda: Nearest(0.5), "Nearest(t only)")
    expect(ValueError, lambda: Nearest(0.5, [1, 2, 3]), "Nearest missing dist_sq")
    expect(
        TypeError,
        lambda: Nearest(0.5, [1, 2, 3], 0.25, foo=1),
        "Nearest unknown kwarg",
    )


def check_quadratic_bezier_values():
    q = QuadraticBezier([0, 0, 0], [1, 2, 0], [2, 0, 0])
    _vis(q.point(0.0), 0.0, 0.0, 0.0)
    _vis(q.point(1.0), 2.0, 0.0, 0.0)
    _vis(q.point(0.5), 1.0, 1.0, 0.0)
    _vis(q.velocity(0.0), 2.0, 4.0, 0.0)
    _vis(q.velocity(1.0), 2.0, -4.0, 0.0)
    _vis(q.velocity(0.5), 2.0, 0.0, 0.0)
    _vis(q.acceleration(0.25), 0.0, -8.0, 0.0)
    _vis(q.tangent(0.5), 1.0, 0.0, 0.0)
    _vis(q.normal(0.5), 0.0, -1.0, 0.0)
    _vis(q.binormal(0.5), 0.0, 0.0, -1.0)
    assert q.curvature(0.5) == 2.0

    pts = q.points
    assert len(pts) == 3
    _vis(pts[0], 0.0, 0.0, 0.0)
    _vis(pts[1], 1.0, 2.0, 0.0)
    _vis(pts[2], 2.0, 0.0, 0.0)

    left, right = q.split(0.5)
    _vis(left.points[0], 0.0, 0.0, 0.0)
    _vis(left.points[1], 0.5, 1.0, 0.0)
    _vis(left.points[2], 1.0, 1.0, 0.0)
    _vis(right.points[0], 1.0, 1.0, 0.0)
    _vis(right.points[1], 1.5, 1.0, 0.0)
    _vis(right.points[2], 2.0, 0.0, 0.0)

    bb = q.aabb()
    _vis(bb.center, 1.0, 1.0, 0.0)
    assert bb.half_extents == [1.0, 1.0, 0.0]

    kwd = QuadraticBezier(p0=[0, 0, 0], p1=[1, 2, 0], p2=[2, 0, 0])
    assert q.abs_diff_eq(kwd, 1e-9) is True

    apex = q.nearest([1.0, 3.0, 0.0])
    assert _approx(apex.t, 0.5, 1e-3)
    _vnear(apex.point, 1.0, 1.0, 0.0, 1e-3)
    assert _approx(apex.dist_sq, 4.0, 1e-3)

    straight = QuadraticBezier([0, 0, 0], [1, 0, 0], [2, 0, 0])
    assert _approx(straight.length(), 2.0, 1e-4)
    assert _approx(straight.arc_length_to(0.5), 1.0, 1e-4)
    assert _approx(straight.t_at_distance(1.0), 0.5, 1e-4)
    _vnear(straight.point_at_distance(0.5), 0.5, 0.0, 0.0, 1e-4)

    assert repr(q) == "QuadraticBezier(3 points)"

    expect(ValueError, lambda: QuadraticBezier(), "QuadraticBezier()")
    expect(
        ValueError,
        lambda: QuadraticBezier([0, 0, 0], [1, 1, 0]),
        "QuadraticBezier missing p2",
    )
    expect(
        TypeError,
        lambda: QuadraticBezier([0, 0, 0], [1, 1, 0], [2, 0, 0], p2=[9, 9, 9]),
        "QuadraticBezier duplicate p2",
    )
    expect(
        TypeError,
        lambda: QuadraticBezier([0, 0, 0], [1, 1, 0], [2, 0, 0], foo=1),
        "QuadraticBezier unknown kwarg",
    )


def check_cubic_bezier_values():
    c = CubicBezier([0, 0, 0], [1, 2, 0], [2, 2, 0], [3, 0, 0])
    _vis(c.point(0.0), 0.0, 0.0, 0.0)
    _vis(c.point(1.0), 3.0, 0.0, 0.0)
    _vis(c.point(0.5), 1.5, 1.5, 0.0)
    _vis(c.velocity(0.0), 3.0, 6.0, 0.0)
    _vis(c.velocity(1.0), 3.0, -6.0, 0.0)
    _vis(c.velocity(0.5), 3.0, 0.0, 0.0)
    _vis(c.acceleration(0.0), 0.0, -12.0, 0.0)

    pts = c.points
    assert len(pts) == 4
    _vis(pts[3], 3.0, 0.0, 0.0)

    left, right = c.split(0.5)
    _vis(left.points[0], 0.0, 0.0, 0.0)
    _vis(left.points[1], 0.5, 1.0, 0.0)
    _vis(left.points[2], 1.0, 1.5, 0.0)
    _vis(left.points[3], 1.5, 1.5, 0.0)
    _vis(right.points[0], 1.5, 1.5, 0.0)
    _vis(right.points[1], 2.0, 1.5, 0.0)
    _vis(right.points[2], 2.5, 1.0, 0.0)
    _vis(right.points[3], 3.0, 0.0, 0.0)

    bb = c.aabb()
    _vis(bb.center, 1.5, 1.0, 0.0)
    assert bb.half_extents == [1.5, 1.0, 0.0]

    kwd = CubicBezier(p0=[0, 0, 0], p1=[1, 2, 0], p2=[2, 2, 0], p3=[3, 0, 0])
    assert c.abs_diff_eq(kwd, 1e-9) is True

    straight = CubicBezier([0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0])
    assert _approx(straight.length(), 3.0, 1e-4)
    assert _approx(straight.arc_length_to(0.5), 1.5, 1e-4)
    _vnear(straight.point_at_distance(1.5), 1.5, 0.0, 0.0, 1e-4)

    assert repr(c) == "CubicBezier(4 points)"

    expect(ValueError, lambda: CubicBezier(), "CubicBezier()")
    expect(
        ValueError,
        lambda: CubicBezier([0, 0, 0], [1, 1, 0], [2, 2, 0]),
        "CubicBezier missing p3",
    )
    expect(
        TypeError,
        lambda: CubicBezier([0, 0, 0], [1, 1, 0], [2, 2, 0], [3, 0, 0], foo=1),
        "CubicBezier unknown kwarg",
    )


def check_polyline_values():
    pl = Polyline([[0, 0, 0], [3, 0, 0], [4, 0, 0]])
    _vis(pl.point(0.0), 0.0, 0.0, 0.0)
    _vis(pl.point(0.25), 1.5, 0.0, 0.0)
    _vis(pl.point(0.5), 3.0, 0.0, 0.0)
    _vis(pl.point(0.75), 3.5, 0.0, 0.0)
    _vis(pl.point(1.0), 4.0, 0.0, 0.0)
    _vis(pl.velocity(0.25), 6.0, 0.0, 0.0)
    _vis(pl.velocity(0.75), 2.0, 0.0, 0.0)
    _vis(pl.acceleration(0.3), 0.0, 0.0, 0.0)
    _vis(pl.tangent(0.1), 1.0, 0.0, 0.0)
    assert pl.curvature(0.3) == 0.0
    assert pl.length() == 4.0
    assert pl.arc_length_to(0.25) == 1.5
    assert pl.arc_length_to(0.5) == 3.0
    assert pl.arc_length_to(1.0) == 4.0
    assert pl.t_at_distance(3.0) == 0.5
    assert pl.t_at_distance(3.5) == 0.75
    _vis(pl.point_at_distance(2.0), 2.0, 0.0, 0.0)
    assert pl.distance([2.0, 2.0, 0.0]) == 2.0
    assert pl.distance_sq([2.0, 2.0, 0.0]) == 4.0

    segs = pl.segments()
    assert len(segs) == 2
    _vis(segs[0].p1, 0.0, 0.0, 0.0)
    _vis(segs[0].p2, 3.0, 0.0, 0.0)
    _vis(segs[1].p1, 3.0, 0.0, 0.0)
    _vis(segs[1].p2, 4.0, 0.0, 0.0)

    bb = pl.aabb()
    _vis(bb.center, 2.0, 0.0, 0.0)
    assert bb.half_extents == [2.0, 0.0, 0.0]

    sub = pl.subcurve(0.25, 0.75)
    assert len(sub.points) == 3
    _vis(sub.points[0], 1.5, 0.0, 0.0)
    _vis(sub.points[1], 3.0, 0.0, 0.0)
    _vis(sub.points[2], 3.5, 0.0, 0.0)
    assert sub.length() == 2.0

    first, second = pl.split_at(0.5)
    assert len(first.points) == 2 and len(second.points) == 2
    _vis(first.points[1], 3.0, 0.0, 0.0)
    _vis(second.points[1], 4.0, 0.0, 0.0)
    tail = pl.truncate_start(0.5)
    _vis(tail.points[0], 3.0, 0.0, 0.0)
    _vis(tail.points[-1], 4.0, 0.0, 0.0)
    head = pl.truncate_end(0.25)
    _vis(head.points[0], 0.0, 0.0, 0.0)
    _vis(head.points[-1], 1.5, 0.0, 0.0)
    rev = pl.reversed()
    _vis(rev.points[0], 4.0, 0.0, 0.0)
    _vis(rev.points[2], 0.0, 0.0, 0.0)

    assert repr(pl) == "Polyline(points=3)"

    kwd = Polyline(points=[[0, 0, 0], [3, 0, 0], [4, 0, 0]])
    assert len(kwd.points) == 3
    assert pl.abs_diff_eq(kwd, 1e-9) is True

    expect(
        TypeError, lambda: Polyline([[0, 0, 0]], [1, 2, 3]), "Polyline 2 positionals"
    )
    expect(TypeError, lambda: Polyline(pts=[[0, 0, 0]]), "Polyline unknown kwarg")


def check_spline_values():
    sp = Spline([[0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0], [4, 0, 0]])
    _vis(sp.point(0.0), 0.0, 0.0, 0.0)
    _vis(sp.point(1.0), 4.0, 0.0, 0.0)
    _vis(sp.point(0.5), 2.0, 0.0, 0.0)
    _vis(sp.point(0.25), 1.0, 0.0, 0.0)
    _vis(sp.point(0.375), 1.5, 0.0, 0.0)
    _vis(sp.point(0.125), 0.4375, 0.0, 0.0)
    _vis(sp.velocity(0.375), 4.0, 0.0, 0.0)
    _vis(sp.acceleration(0.375), 0.0, 0.0, 0.0)
    _vis(sp.tangent(0.375), 1.0, 0.0, 0.0)
    assert sp.curvature(0.375) == 0.0
    assert _approx(sp.length(), 4.0, 1e-3)
    assert _approx(sp.arc_length_to(0.5), sp.length() / 2.0, 1e-4)
    _vnear(sp.point_at_distance(2.0), 2.0, 0.0, 0.0, 1e-4)

    bb = sp.aabb()
    _vis(bb.center, 2.0, 0.0, 0.0)
    assert bb.half_extents == [2.0, 0.0, 0.0]

    near = sp.nearest([2.0, 1.0, 0.0])
    assert _approx(near.t, 0.5, 1e-3)
    _vnear(near.point, 2.0, 0.0, 0.0, 1e-3)
    assert _approx(near.dist_sq, 1.0, 1e-3)

    interp = Spline([[0, 0, 0], [1, 1, 0], [2, 0, 0]])
    _vis(interp.point(0.0), 0.0, 0.0, 0.0)
    _vis(interp.point(0.5), 1.0, 1.0, 0.0)
    _vis(interp.point(1.0), 2.0, 0.0, 0.0)

    assert repr(sp) == "Spline(points=5)"

    kwd = Spline(points=[[0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0], [4, 0, 0]])
    assert len(kwd.points) == 5
    assert sp.abs_diff_eq(kwd, 1e-9) is True

    expect(TypeError, lambda: Spline([[0, 0, 0]], [1, 2, 3]), "Spline 2 positionals")
    expect(TypeError, lambda: Spline(pts=[[0, 0, 0]]), "Spline unknown kwarg")


def check_empty_curves():
    for empty in (Polyline(), Polyline(points=None), Spline(), Spline(points=None)):
        name = type(empty).__name__
        assert len(empty.points) == 0, f"{name}: empty constructor kept points"
        expect(ValueError, empty.aabb, f"empty {name}.aabb()")
    assert Polyline().length() == 0.0
    assert repr(Polyline()) == "Polyline(points=0)"
    assert repr(Spline()) == "Spline(points=0)"


def main() -> None:
    check_interval()
    check_nearest()
    check_quadratic_bezier()
    check_cubic_bezier()
    check_polyline()
    check_spline()
    check_interval_values()
    check_nearest_values()
    check_quadratic_bezier_values()
    check_cubic_bezier_values()
    check_polyline_values()
    check_spline_values()
    check_empty_curves()
    check_curve_common(QuadraticBezier([0, 0, 0], [1, 2, 0], [2, 0, 0]))
    check_curve_common(CubicBezier([0, 0, 0], [1, 2, 0], [2, 2, 0], [3, 0, 0]))
    check_curve_common(Polyline([[0, 0, 0], [3, 0, 0], [4, 0, 0]]))
    check_curve_common(Spline([[0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0], [4, 0, 0]]))
    print(
        "ok squiggle: duck-decode, numeric ground truth, constructor errors, "
        "empty-curve aabb, pickle round-trips"
    )
