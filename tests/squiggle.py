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


def main() -> None:
    check_interval()
    check_nearest()
    check_quadratic_bezier()
    check_cubic_bezier()
    check_polyline()
    check_spline()
    print(
        "ok squiggle duck-decode: Interval, Nearest, QuadraticBezier, CubicBezier, Polyline, Spline"
    )
