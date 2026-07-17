import math

try:
    import numpy as np
except ImportError:
    np = None


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


def duck_cols(m):
    """Foreign clone of a matrix decoded via its to_cols_array() method."""
    cols = [float(v) for v in m.to_cols_array()]
    return type(
        type(m).__name__, (), {"to_cols_array": lambda self, _c=cols: list(_c)}
    )()


def duck_affine(a):
    """Foreign clone of an Affine3 decoded via .matrix3 / .translation attrs."""
    foreign = type(type(a).__name__, (), {})()
    foreign.matrix3 = duck_cols(a.matrix3)
    foreign.translation = duck(a.translation)
    return foreign


def approx(a, b, tol=1e-9):
    return abs(a - b) <= tol


def cols_equal(a, b, tol=1e-9):
    a = [float(v) for v in a]
    b = [float(v) for v in b]
    assert len(a) == len(b), f"length mismatch {len(a)} vs {len(b)}"
    return all(abs(x - y) <= tol for x, y in zip(a, b))


def veq(v, expected, tol=1e-9):
    return cols_equal(v.to_array(), expected, tol)


def assert_raises(exc, fn, *args, **kwargs):
    try:
        fn(*args, **kwargs)
    except exc:
        return
    except Exception as e:
        raise AssertionError(
            f"expected {exc.__name__}, got {type(e).__name__}: {e}"
        ) from e
    raise AssertionError(f"expected {exc.__name__}, nothing raised")


def check_vec3():
    box = Cuboid.from_aabb(Vec3(-1.0, -1.0, -1.0), Vec3(1.0, 1.0, 1.0))
    inside = Vec3(0.25, -0.5, 0.75)
    outside = Vec3(2.0, 0.0, 0.0)

    for native in (inside, outside):
        d = duck(native)
        listform = [native.x, native.y, native.z]
        assert box.contains_point(native) == box.contains_point(d)
        assert box.contains_point(native) == box.contains_point(listform)
        assert approx(box.point_dist_sq(native), box.point_dist_sq(d))
        assert approx(box.point_dist_sq(native), box.point_dist_sq(listform))

    native = Vec3(3.0, -4.0, 5.0)
    s_native = Sphere(center=native, radius=2.0)
    s_duck = Sphere(center=duck(native), radius=2.0)
    s_list = Sphere(center=[3.0, -4.0, 5.0], radius=2.0)
    assert s_native.center == s_duck.center
    assert s_native.center == s_list.center


def check_vec2():
    v = Vec2(2.5, -1.5)
    assert approx(v.x, 2.5) and approx(v.y, -1.5)
    assert cols_equal(v.to_list(), [2.5, -1.5])

    native = Vec2(0.8, 1.3)
    m_native = Mat3.from_scale(native)
    m_duck = Mat3.from_scale(duck(native))
    assert cols_equal(m_native.to_cols_array(), m_duck.to_cols_array())

    t_native = Mat3.from_translation(native)
    t_duck = Mat3.from_translation(duck(native))
    assert cols_equal(t_native.to_cols_array(), t_duck.to_cols_array())


def check_vec4():
    v = Vec4(1.0, 2.0, 3.0, 4.0)
    assert approx(v.x, 1.0) and approx(v.w, 4.0)
    assert cols_equal(v.to_list(), [1.0, 2.0, 3.0, 4.0])

    native = Vec4(2.0, -3.0, 4.0, 2.0)
    assert Vec3.from_homogeneous(native) == Vec3.from_homogeneous(duck(native))
    assert Vec3.from_homogeneous(native) == Vec3.from_homogeneous([2.0, -3.0, 4.0, 2.0])

    diag_native = Mat4.from_diagonal(native)
    diag_duck = Mat4.from_diagonal(duck(native))
    diag_list = Mat4.from_diagonal([2.0, -3.0, 4.0, 2.0])
    assert cols_equal(diag_native.to_cols_array(), diag_duck.to_cols_array())
    assert cols_equal(diag_native.to_cols_array(), diag_list.to_cols_array())


def check_quat():
    native = Quat.from_axis_angle(Vec3(0.0, 0.0, 1.0), 0.6)
    d = duck(native)
    listform = [native.x, native.y, native.z, native.w]

    assert cols_equal(
        Mat3.from_quat(native).to_cols_array(), Mat3.from_quat(d).to_cols_array()
    )
    assert cols_equal(
        Mat3.from_quat(native).to_cols_array(), Mat3.from_quat(listform).to_cols_array()
    )
    assert cols_equal(
        Mat4.from_quat(native).to_cols_array(), Mat4.from_quat(d).to_cols_array()
    )


def check_mat3():
    orientation = Mat3.from_rotation_z(0.5)
    center = Vec3(1.0, 2.0, 3.0)
    size = (2.0, 4.0, 6.0)

    c_native = Cuboid.from_center_size_orientation(center, size, orientation)
    c_duck = Cuboid.from_center_size_orientation(center, size, duck_cols(orientation))
    assert c_native.center == c_duck.center
    assert cols_equal(
        c_native.orientation.to_cols_array(), c_duck.orientation.to_cols_array()
    )
    assert cols_equal(c_native.half_extents, c_duck.half_extents)

    assert cols_equal(
        Mat4.from_mat3(orientation).to_cols_array(),
        Mat4.from_mat3(duck_cols(orientation)).to_cols_array(),
    )


def check_mat4():
    native = Mat4.from_rotation_y(0.35)
    d = duck_cols(native)
    assert cols_equal(
        Mat3.from_mat4(native).to_cols_array(), Mat3.from_mat4(d).to_cols_array()
    )
    assert cols_equal(
        Affine3.from_mat4(native).to_cols_array(), Affine3.from_mat4(d).to_cols_array()
    )


def check_affine3():
    rot = Mat3.from_rotation_x(0.4)
    trans = Vec3(5.0, -2.0, 1.0)
    native = Affine3.from_mat3_translation(rot, trans)
    d = duck_affine(native)
    assert cols_equal(
        Quat.from_affine3(native).to_array(), Quat.from_affine3(d).to_array()
    )


def check_euler_rot():
    a = Mat3.from_euler(EulerRot.XYZ, 0.1, 0.2, 0.3)
    b = Mat3.from_euler(EulerRot.XYZ, 0.1, 0.2, 0.3)
    assert cols_equal(a.to_cols_array(), b.to_cols_array())
    assert approx(a.determinant(), 1.0)

    q = Quat.from_euler(EulerRot.ZYX, 0.1, 0.2, 0.3)
    assert approx(q.length(), 1.0)


VEC_CLASSES = None


def vec_classes():
    return ((Vec2, 2), (Vec3, 3), (Vec4, 4))


def check_vec_constructor_forms():
    for cls, n in vec_classes():
        axes = "xyzw"[:n]
        assert veq(cls(), [0.0] * n), f"{cls.__name__}() must default all zeros"
        assert veq(cls(1.5), [1.5] + [0.0] * (n - 1))
        for k in range(2, n + 1):
            vals = [float(i + 1) for i in range(k)]
            expected = vals + [0.0] * (n - k)
            assert veq(cls(*vals), expected)
        full = [float(i + 1) for i in range(n)]
        kw = {ax: val for ax, val in zip(axes, full)}
        assert cls(**kw) == cls(*full), (
            f"{cls.__name__} keyword ctor must match positional"
        )
        assert veq(cls(y=2.0), [0.0, 2.0] + [0.0] * (n - 2))
        assert cls(1.0, **{axes[-1]: 9.0}) == cls(*([1.0] + [0.0] * (n - 2) + [9.0]))
        assert_raises(TypeError, cls, *([1.0] * (n + 1)))
        assert_raises(TypeError, cls, no_such_arg=1.0)
        assert_raises(TypeError, cls, 1.0, x=2.0)

    q = Quat()
    assert q.x == 0.0 and q.y == 0.0 and q.z == 0.0 and q.w == 1.0
    assert Quat(0.1, 0.2, 0.3) == Quat(0.1, 0.2, 0.3, 1.0)
    assert Quat(x=0.1, y=0.2, z=0.3, w=0.4) == Quat(0.1, 0.2, 0.3, 0.4)
    assert Quat(z=0.5).w == 1.0
    assert_raises(TypeError, Quat, 1.0, 2.0, 3.0, 4.0, 5.0)
    assert_raises(TypeError, Quat, bogus=1.0)


def check_matrix_constructor_forms():
    c0, c1, c2 = Vec3(1, 2, 3), Vec3(4, 5, 6), Vec3(7, 8, 10)
    m_pos = Mat3(c0, c1, c2)
    m_kw = Mat3(x_axis=c0, y_axis=c1, z_axis=c2)
    m_cols = Mat3.from_cols(c0, c1, c2)
    assert m_pos == m_cols
    assert m_kw == m_cols
    assert Mat3(c0, z_axis=c2, y_axis=c1) == m_cols
    assert_raises(ValueError, Mat3)
    assert_raises(ValueError, Mat3, c0, c1)
    assert_raises(ValueError, Mat3, x_axis=c0, z_axis=c2)
    assert_raises(TypeError, Mat3, c0, c1, c2, c0)
    assert_raises(TypeError, Mat3, c0, c1, c2, w_axis=c0)

    d0, d1 = Vec4(1, 2, 3, 4), Vec4(5, 6, 7, 8)
    d2, d3 = Vec4(9, 10, 11, 12), Vec4(13, 14, 15, 16)
    m4_pos = Mat4(d0, d1, d2, d3)
    m4_kw = Mat4(x_axis=d0, y_axis=d1, z_axis=d2, w_axis=d3)
    m4_cols = Mat4.from_cols(d0, d1, d2, d3)
    assert m4_pos == m4_cols
    assert m4_kw == m4_cols
    assert_raises(ValueError, Mat4)
    assert_raises(ValueError, Mat4, d0, d1, d2)
    assert_raises(ValueError, Mat4, x_axis=d0, y_axis=d1, w_axis=d3)
    assert_raises(TypeError, Mat4, d0, d1, d2, d3, d0)
    assert_raises(TypeError, Mat4, d0, d1, d2, d3, extra=d0)


def check_vec_common_methods():
    for cls, n in vec_classes():
        axes = "xyzw"[:n]
        a = [3.0, 4.0] + [0.0] * (n - 2)
        v = cls(*a)

        arr = v.to_array()
        assert type(arr) is list, f"{cls.__name__}.to_array() must return a list"
        assert arr == a
        lst = v.to_list()
        assert type(lst) is list and lst == a
        assert cls.from_array(a) == v
        assert cls.splat(2.5) == cls(*([2.5] * n))

        for i, ax in enumerate(axes):
            assert getattr(v, ax) == a[i]
            w = getattr(v, f"with_{ax}")(9.0)
            expected = list(a)
            expected[i] = 9.0
            assert veq(w, expected)

        b = [1.0, -2.0, 2.0, 0.5][:n]
        vb = cls(*b)
        assert approx(v.dot(vb), sum(x * y for x, y in zip(a, b)))
        assert approx(v.length(), 5.0)
        assert approx(v.length_squared(), 25.0)
        assert approx(v.length_recip(), 0.2)
        assert approx(v.distance(cls()), 5.0)
        assert approx(v.distance_squared(cls()), 25.0)
        diff = [x - y for x, y in zip(a, b)]
        assert approx(v.distance(vb), math.sqrt(sum(d * d for d in diff)))
        assert approx(v.distance_squared(vb), sum(d * d for d in diff))

        unit = [x / 5.0 for x in a]
        assert veq(v.normalize(), unit)
        assert not v.is_normalized()
        assert v.normalize().is_normalized()
        nv, ln = v.normalize_and_length()
        assert veq(nv, unit) and approx(ln, 5.0)
        assert cls().try_normalize() is None
        assert veq(v.try_normalize(), unit)
        fb = cls(*([1.0] + [0.0] * (n - 1)))
        assert cls().normalize_or(fb) == fb
        assert veq(v.normalize_or(fb), unit)

        ux = cls(*([1.0] + [0.0] * (n - 1)))
        assert veq(v.project_onto(ux), [a[0]] + [0.0] * (n - 1))
        assert veq(v.reject_from(ux), [0.0] + a[1:])
        assert veq(v.project_onto_normalized(ux), [a[0]] + [0.0] * (n - 1))
        assert veq(v.reject_from_normalized(ux), [0.0] + a[1:])

        uy = cls(*([0.0, 1.0] + [0.0] * (n - 2)))
        inc = cls(*([1.0, -1.0] + [0.0] * (n - 2)))
        assert veq(inc.reflect(uy), [1.0, 1.0] + [0.0] * (n - 2))
        inc2 = cls(*([0.6, -0.8] + [0.0] * (n - 2)))
        assert veq(inc2.refract(uy, 1.0), inc2.to_array())

        assert veq(v.lerp(vb, 0.25), [x + (y - x) * 0.25 for x, y in zip(a, b)])
        assert veq(v.midpoint(vb), [(x + y) / 2.0 for x, y in zip(a, b)])
        tgt = list(a)
        tgt[1] += 6.0
        moved = v.move_towards(cls(*tgt), 2.0)
        expected = list(a)
        expected[1] += 2.0
        assert veq(moved, expected)

        assert veq(v.min(vb), [min(x, y) for x, y in zip(a, b)])
        assert veq(v.max(vb), [max(x, y) for x, y in zip(a, b)])
        lo = cls(*([0.0] * n))
        hi = cls(*([2.0, 3.0] + [1.0] * (n - 2)))
        assert veq(
            v.clamp(lo, hi),
            [
                min(max(x, low), high)
                for x, low, high in zip(a, lo.to_array(), hi.to_array())
            ],
        )

        c = [2.0, -1.0, 5.0, 0.5][:n]
        vc = cls(*c)
        assert vc.min_element() == min(c)
        assert vc.max_element() == max(c)
        assert vc.min_position() == c.index(min(c))
        assert vc.max_position() == c.index(max(c))
        assert approx(vc.element_sum(), sum(c))
        prod = 1.0
        for x in c:
            prod *= x
        assert approx(vc.element_product(), prod)

        assert veq(v.clamp_length(0.0, 1.0), unit)
        assert veq(v.clamp_length_max(5.0), a)
        assert veq(v.clamp_length_max(1.0), unit)
        assert veq(v.clamp_length_min(10.0), [2.0 * x for x in a])
        assert veq(v.clamp_length_min(1.0), a)

        signs = [-1.0, 1.0, -2.0, 3.0][:n]
        assert veq(
            vc.copysign(cls(*signs)), [math.copysign(x, s) for x, s in zip(c, signs)]
        )
        assert veq(vc.powf(2.0), [x * x for x in c], 1e-9)
        assert veq(vc.mul_add(vb, v), [x * y + z for x, y, z in zip(c, b, a)])

        s, co = vc.sin_cos()
        assert veq(s, [math.sin(x) for x in c], 1e-12)
        assert veq(co, [math.cos(x) for x in c], 1e-12)

        edge = [1.0, 3.0, 0.0, 2.0][:n]
        x = [2.0] * n
        assert veq(
            cls(*edge).step(cls(*x)),
            [1.0 if xv >= e else 0.0 for e, xv in zip(edge, x)],
        )

        p = [7.0, -7.0, 8.0, -3.0][:n]
        q = [2.0, 2.0, 3.0, 2.0][:n]
        assert veq(cls(*p).div_euclid(cls(*q)), [pv // qv for pv, qv in zip(p, q)])
        assert veq(cls(*p).rem_euclid(cls(*q)), [pv % qv for pv, qv in zip(p, q)])

        assert v.abs_diff_eq(cls(*[x + 1e-6 for x in a]), 1e-3)
        assert not v.abs_diff_eq(cls(*[x + 1e-6 for x in a]), 1e-9)

        assert v.is_finite()
        assert not v.is_nan()
        assert not cls.INFINITY.is_finite()
        assert not cls.NEG_INFINITY.is_finite()
        assert cls.NAN.is_nan()

        assert cls.ZERO == cls()
        assert veq(cls.ONE, [1.0] * n)
        assert veq(cls.NEG_ONE, [-1.0] * n)
        for i, ax in enumerate(axes):
            e = [0.0] * n
            e[i] = 1.0
            assert veq(getattr(cls, ax.upper()), e)
            e[i] = -1.0
            assert veq(getattr(cls, "NEG_" + ax.upper()), e)


def check_vec_unary_methods():
    for cls, n in vec_classes():
        vals = [-1.5, 0.25, 4.0, -0.75][:n]
        v = cls(*vals)
        assert veq(v.abs(), [abs(x) for x in vals])
        assert veq(v.signum(), [math.copysign(1.0, x) for x in vals])
        assert veq(v.floor(), [math.floor(x) for x in vals])
        assert veq(v.ceil(), [math.ceil(x) for x in vals])
        assert veq(v.round(), [-2.0, 0.0, 4.0, -1.0][:n])
        assert veq(v.trunc(), [math.trunc(x) for x in vals])
        assert veq(v.fract(), [x - math.trunc(x) for x in vals])
        assert veq(v.fract_gl(), [x - math.floor(x) for x in vals])
        assert veq(v.saturate(), [min(max(x, 0.0), 1.0) for x in vals])

        pos = [0.25, 4.0, 16.0, 1.0][:n]
        pv = cls(*pos)
        assert veq(pv.sqrt(), [math.sqrt(x) for x in pos], 1e-12)
        assert veq(pv.recip(), [1.0 / x for x in pos], 1e-12)
        assert veq(pv.ln(), [math.log(x) for x in pos], 1e-12)
        assert veq(pv.log2(), [math.log2(x) for x in pos], 1e-12)

        ex = [0.0, 1.0, 2.0, -1.0][:n]
        evec = cls(*ex)
        assert veq(evec.exp(), [math.exp(x) for x in ex], 1e-12)
        assert veq(evec.exp2(), [2.0**x for x in ex], 1e-12)
        assert veq(evec.cos(), [math.cos(x) for x in ex], 1e-12)
        assert veq(evec.sin(), [math.sin(x) for x in ex], 1e-12)

        assert cls().normalize_or_zero() == cls.ZERO
        three_four = cls(*([3.0, 4.0] + [0.0] * (n - 2)))
        assert veq(three_four.normalize_or_zero(), [0.6, 0.8] + [0.0] * (n - 2))


def check_vec_operator_protocol():
    for cls, n in vec_classes():
        a = [3.0, 4.0] + [1.0] * (n - 2)
        c = [2.0, -1.0, 5.0, 0.5][:n]
        v = cls(*a)
        u = cls(*c)

        assert veq(v + u, [x + y for x, y in zip(a, c)])
        assert veq(v + 2.0, [x + 2.0 for x in a])
        assert veq(2.0 + v, [x + 2.0 for x in a])
        assert veq(v - u, [x - y for x, y in zip(a, c)])
        assert veq(v - 1.0, [x - 1.0 for x in a])
        assert veq(10.0 - v, [10.0 - x for x in a])
        assert veq(v * u, [x * y for x, y in zip(a, c)])
        assert veq(v * 2.0, [x * 2.0 for x in a])
        assert veq(3.0 * v, [3.0 * x for x in a])
        assert veq(v / 2.0, [x / 2.0 for x in a])
        assert veq(v / u, [x / y for x, y in zip(a, c)])
        assert veq(12.0 / u, [12.0 / y for y in c])
        assert veq(v % 2.0, [math.fmod(x, 2.0) for x in a])
        assert veq(v % u, [math.fmod(x, y) for x, y in zip(a, c)])
        assert veq(7.0 % u, [math.fmod(7.0, y) for y in c]), (
            f"scalar % {cls.__name__} must splat the scalar and delegate"
        )

        assert veq(-v, [-x for x in a])
        assert (+v) == v

        assert v == cls(*a)
        assert v == list(a)
        assert not (v == u)
        assert v != u
        assert not (v == "x")
        assert v != "x"

        sentinel = object()
        radd_cls = type("Radd", (), {"__radd__": lambda self, other: sentinel})
        assert (v + radd_cls()) is sentinel, (
            f"{cls.__name__}.__add__ must return NotImplemented so foreign __radd__ runs"
        )
        rmod_cls = type("Rmod", (), {"__rmod__": lambda self, other: sentinel})
        assert (v % rmod_cls()) is sentinel
        assert_raises(TypeError, lambda: v + "x")
        assert_raises(TypeError, lambda: v * "x")
        assert_raises(TypeError, lambda: "x" - v)

        assert hash(cls(*a)) == hash(cls(*a))
        neg_zero = [-0.0] + [0.0] * (n - 1)
        pos_zero = [0.0] * n
        assert cls(*neg_zero) == cls(*pos_zero)
        assert hash(cls(*neg_zero)) == hash(cls(*pos_zero)), (
            f"hash({cls.__name__}(-0.0, ...)) must equal hash of +0.0 form"
        )

        assert len(v) == n
        for i in range(n):
            assert v[i] == a[i]
            assert v[-1 - i] == a[n - 1 - i]
        assert_raises(IndexError, lambda: v[n])
        assert_raises(IndexError, lambda: v[-n - 1])

        assert repr(v).startswith(cls.__name__ + "(")
        assert str(v).startswith("[") and str(v).endswith("]")


def check_vec_duck_operands():
    for cls, n in vec_classes():
        a = [1.5, -2.0, 3.25, 0.5][:n]
        b = [4.0, 0.5, -1.0, 2.0][:n]
        v = cls(*a)
        w = cls(*b)
        fw = duck(w)

        assert v == list(a)
        assert v == tuple(a)
        assert v == duck(v)
        assert not (v == list(a) + [0.0])
        assert not (v == list(a)[:-1])
        assert not (v == list(b))

        summed = [x + y for x, y in zip(a, b)]
        assert veq(v + tuple(b), summed)
        assert veq(v + list(b), summed)
        assert veq(list(b) + v, summed)
        assert veq(v + fw, summed)
        assert veq(v - tuple(b), [x - y for x, y in zip(a, b)])
        assert veq(v * list(b), [x * y for x, y in zip(a, b)])
        assert_raises(TypeError, lambda: v + tuple(list(b) + [1.0]))
        assert_raises(TypeError, lambda: v + list(b)[:-1])

        dotted = sum(x * y for x, y in zip(a, b))
        assert approx(v.dot(list(b)), dotted)
        assert approx(v.dot(tuple(b)), dotted)
        assert approx(v.dot(fw), dotted)
        assert veq(v.min(tuple(b)), [min(x, y) for x, y in zip(a, b)])
        assert veq(v.max(list(b)), [max(x, y) for x, y in zip(a, b)])
        assert veq(v.lerp(fw, 0.5), [(x + y) / 2.0 for x, y in zip(a, b)])
        assert veq(v.lerp(list(b), 1.0), b)

    q = Quat.from_rotation_z(0.0)
    assert q == [0.0, 0.0, 0.0, 1.0]
    assert q == (0.0, 0.0, 0.0, 1.0)
    assert q == duck(q)
    assert not (q == [0.0, 0.0, 0.0])


def check_vec2_specific():
    assert Vec2(3.0, 4.0).perp() == Vec2(-4.0, 3.0)
    assert approx(Vec2(3.0, 4.0).perp_dot(Vec2(1.0, 2.0)), 2.0)

    assert veq(Vec2(0.0, 1.0).rotate(Vec2(1.0, 0.0)), [0.0, 1.0])
    assert veq(Vec2(0.0, 1.0).rotate(Vec2(2.0, 0.0)), [0.0, 2.0])
    assert veq(Vec2(0.0, 2.0).rotate(Vec2(0.0, 1.0)), [-2.0, 0.0])

    assert approx(Vec2(1.0, 0.0).angle_to(Vec2(0.0, 1.0)), math.pi / 2)
    assert approx(Vec2(0.0, 1.0).angle_to(Vec2(1.0, 0.0)), -math.pi / 2)

    h = math.sqrt(2.0) / 2.0
    assert veq(Vec2(1.0, 0.0).rotate_towards(Vec2(0.0, 1.0), math.pi / 4), [h, h])

    assert Vec2(1.0, 2.0).extend(3.0) == Vec3(1.0, 2.0, 3.0)


def check_vec3_specific():
    assert Vec3(1, 0, 0).cross(Vec3(0, 1, 0)) == Vec3(0, 0, 1)
    assert Vec3(0, 1, 0).cross(Vec3(1, 0, 0)) == Vec3(0, 0, -1)

    assert Vec3(1, 2, 3).extend(4.0) == Vec4(1, 2, 3, 4)
    assert Vec3(1, 2, 3).truncate() == Vec2(1, 2)
    assert Vec3(1, 2, 3).to_homogeneous() == Vec4(1, 2, 3, 1)
    assert Vec3.from_homogeneous(Vec4(2, -3, 4, 2)) == Vec3(1, -1.5, 2)

    assert veq(Vec3.from_spherical(Mat3.from_rotation_z(math.pi / 2), 2.0), [0, 2, 0])
    assert veq(Vec3.from_spherical(Mat3.IDENTITY, 3.0), [3, 0, 0])

    m = Vec3(0.0, 0.0, 2.0).angle()
    assert veq(m.mul_vec3(Vec3(1.0, 0.0, 0.0)), [0.0, 0.0, 1.0])
    ident = Vec3(0.0, 3.0, 0.0).angle(Vec3(0.0, 1.0, 0.0))
    assert ident.abs_diff_eq(Mat3.IDENTITY, 1e-9)
    assert_raises(ValueError, Vec3().angle)
    assert_raises(ValueError, Vec3(1, 0, 0).angle, Vec3())

    h = math.sqrt(2.0) / 2.0
    assert veq(Vec3(1, 0, 0).slerp(Vec3(0, 1, 0), 0.5), [h, h, 0.0])
    assert approx(Vec3(1, 0, 0).angle_between(Vec3(0, 1, 0)), math.pi / 2)

    assert veq(Vec3(0, 1, 0).rotate_x(math.pi / 2), [0, 0, 1])
    assert veq(Vec3(0, 0, 1).rotate_y(math.pi / 2), [1, 0, 0])
    assert veq(Vec3(1, 0, 0).rotate_z(math.pi / 2), [0, 1, 0])
    assert veq(Vec3(1, 0, 0).rotate_axis(Vec3(0, 0, 1), math.pi / 2), [0, 1, 0])
    assert veq(Vec3(1, 0, 0).rotate_towards(Vec3(0, 1, 0), math.pi / 4), [h, h, 0.0])

    v = Vec3(1.0, 2.0, -0.5)
    ortho = v.any_orthogonal_vector()
    assert approx(v.dot(ortho), 0.0, 1e-12) and ortho.length() > 0.0
    onorm = v.normalize().any_orthonormal_vector()
    assert approx(v.normalize().dot(onorm), 0.0, 1e-9)
    assert approx(onorm.length(), 1.0)
    p1, p2 = v.normalize().any_orthonormal_pair()
    assert approx(p1.length(), 1.0) and approx(p2.length(), 1.0)
    assert approx(p1.dot(p2), 0.0, 1e-9)
    assert approx(p1.dot(v.normalize()), 0.0, 1e-9)
    assert approx(p2.dot(v.normalize()), 0.0, 1e-9)


def check_vec4_specific():
    assert Vec4(1, 2, 3, 4).truncate() == Vec3(1, 2, 3)


def check_quat_full():
    q = Quat.from_xyzw(0.1, 0.2, 0.3, 0.4)
    assert q.x == 0.1 and q.y == 0.2 and q.z == 0.3 and q.w == 0.4
    assert Quat.from_array([0.1, 0.2, 0.3, 0.4]) == q
    arr = q.to_array()
    assert type(arr) is list and arr == [0.1, 0.2, 0.3, 0.4]
    assert q.xyz() == Vec3(0.1, 0.2, 0.3)

    half = math.pi / 4
    qz90 = Quat.from_axis_angle(Vec3(0, 0, 1), math.pi / 2)
    assert approx(qz90.x, 0.0, 1e-15) and approx(qz90.y, 0.0, 1e-15)
    assert approx(qz90.z, math.sin(half), 1e-15)
    assert approx(qz90.w, math.cos(half), 1e-15)
    assert veq(qz90.mul_vec3(Vec3(1, 0, 0)), [0, 1, 0])

    assert Quat.from_scaled_axis(Vec3(0, 0, math.pi / 2)).abs_diff_eq(qz90, 1e-12)
    assert Quat.from_rotation_x(0.6).abs_diff_eq(
        Quat.from_axis_angle(Vec3(1, 0, 0), 0.6), 1e-12
    )
    assert Quat.from_rotation_y(0.6).abs_diff_eq(
        Quat.from_axis_angle(Vec3(0, 1, 0), 0.6), 1e-12
    )
    assert Quat.from_rotation_z(0.6).abs_diff_eq(
        Quat.from_axis_angle(Vec3(0, 0, 1), 0.6), 1e-12
    )

    a, b, c = 0.1, 0.2, 0.3
    qe = Quat.from_euler(EulerRot.XYZ, a, b, c)
    manual = (
        Quat.from_rotation_x(a)
        .mul_quat(Quat.from_rotation_y(b))
        .mul_quat(Quat.from_rotation_z(c))
    )
    assert qe.abs_diff_eq(manual, 1e-12), "from_euler XYZ must compose Rx*Ry*Rz"
    assert Quat.from_euler(EulerRot.XYZ, a, 0.0, 0.0).abs_diff_eq(
        Quat.from_rotation_x(a), 1e-12
    )
    ea, eb, ec = qe.to_euler(EulerRot.XYZ)
    assert approx(ea, a) and approx(eb, b) and approx(ec, c)

    rot = Mat3.from_rotation_z(0.7)
    q07 = Quat.from_rotation_z(0.7)
    assert Quat.from_mat3(rot).abs_diff_eq(q07, 1e-12)
    assert Quat.from_mat4(Mat4.from_rotation_z(0.7)).abs_diff_eq(q07, 1e-12)
    assert Quat.from_affine3(Affine3.from_rotation_z(0.7)).abs_diff_eq(q07, 1e-12)
    assert Quat.from_rotation_axes(rot.x_axis, rot.y_axis, rot.z_axis).abs_diff_eq(
        q07, 1e-12
    )

    arc = Quat.from_rotation_arc(Vec3(1, 0, 0), Vec3(0, 1, 0))
    assert veq(arc.mul_vec3(Vec3(1, 0, 0)), [0, 1, 0])
    colinear = Quat.from_rotation_arc_colinear(Vec3(1, 0, 0), Vec3(-1, 0, 0))
    assert colinear.abs_diff_eq(Quat.IDENTITY, 1e-12)
    diag = Vec3(1, 1, 0).normalize()
    colinear2 = Quat.from_rotation_arc_colinear(Vec3(1, 0, 0), diag)
    assert approx(abs(diag.dot(colinear2.mul_vec3(Vec3(1, 0, 0)))), 1.0)
    arc2d = Quat.from_rotation_arc_2d(Vec2(1, 0), Vec2(0, 1))
    assert veq(arc2d.mul_vec3(Vec3(1, 0, 0)), [0, 1, 0])

    assert Quat.look_to_rh(Vec3(0, 0, -1), Vec3(0, 1, 0)).abs_diff_eq(
        Quat.IDENTITY, 1e-12
    )
    assert Quat.look_to_lh(Vec3(0, 0, 1), Vec3(0, 1, 0)).abs_diff_eq(
        Quat.IDENTITY, 1e-12
    )
    assert Quat.look_at_rh(Vec3(0, 0, 5), Vec3(0, 0, 0), Vec3(0, 1, 0)).abs_diff_eq(
        Quat.IDENTITY, 1e-12
    )
    assert Quat.look_at_lh(Vec3(0, 0, -5), Vec3(0, 0, 0), Vec3(0, 1, 0)).abs_diff_eq(
        Quat.IDENTITY, 1e-12
    )

    assert Quat.from_xyzw(1, 2, 3, 4).conjugate() == Quat.from_xyzw(-1, -2, -3, 4)
    assert (
        Quat.from_rotation_z(0.6)
        .inverse()
        .abs_diff_eq(Quat.from_rotation_z(-0.6), 1e-12)
    )
    assert approx(Quat.from_xyzw(1, 2, 3, 4).dot(Quat.from_xyzw(5, 6, 7, 8)), 70.0)
    assert approx(Quat.from_xyzw(1, 2, 3, 4).length(), math.sqrt(30.0))
    assert approx(Quat.from_xyzw(1, 2, 3, 4).length_squared(), 30.0)
    assert approx(Quat.from_xyzw(1, 2, 3, 4).length_recip(), 1.0 / math.sqrt(30.0))
    assert (
        Quat.from_xyzw(0, 0, 3, 4)
        .normalize()
        .abs_diff_eq(Quat.from_xyzw(0, 0, 0.6, 0.8), 1e-12)
    )

    assert (
        Quat.from_rotation_z(0.3)
        .mul_quat(Quat.from_rotation_z(0.4))
        .abs_diff_eq(Quat.from_rotation_z(0.7), 1e-12)
    )
    assert Quat.IDENTITY.slerp(qz90, 0.5).abs_diff_eq(
        Quat.from_rotation_z(math.pi / 4), 1e-9
    )
    assert Quat.IDENTITY.lerp(qz90, 0.0).abs_diff_eq(Quat.IDENTITY, 1e-12)
    assert Quat.IDENTITY.lerp(qz90, 1.0).abs_diff_eq(qz90, 1e-12)
    assert approx(
        Quat.from_rotation_z(0.3).angle_between(Quat.from_rotation_z(0.9)), 0.6
    )
    assert Quat.IDENTITY.rotate_towards(Quat.from_rotation_z(1.0), 0.4).abs_diff_eq(
        Quat.from_rotation_z(0.4), 1e-9
    )

    q08 = Quat.from_axis_angle(Vec3(0, 0, 1), 0.8)
    axis, angle = q08.to_axis_angle()
    assert veq(axis, [0, 0, 1]) and approx(angle, 0.8)
    assert veq(q08.to_scaled_axis(), [0, 0, 0.8])

    assert Quat().is_finite() and not Quat().is_nan()
    assert Quat.NAN.is_nan() and not Quat.NAN.is_finite()
    assert Quat.from_rotation_z(0.3).is_normalized()
    assert not Quat.from_xyzw(1, 2, 3, 4).is_normalized()
    assert Quat.from_rotation_z(1e-8).is_near_identity()
    assert not Quat.from_rotation_z(0.5).is_near_identity()
    assert Quat.IDENTITY == Quat()

    q3 = Quat.from_rotation_z(0.3)
    q4 = Quat.from_rotation_z(0.4)
    assert (q3 * q4).abs_diff_eq(Quat.from_rotation_z(0.7), 1e-12)
    assert veq(qz90 * Vec3(1, 0, 0), [0, 1, 0])
    doubled = q3 * 2.0
    assert veq(doubled, [2 * x for x in q3.to_array()])
    assert (2.0 * q3) == doubled
    added = q3 + q3
    assert veq(added, [2 * x for x in q3.to_array()])
    assert veq(q3 - q3, [0, 0, 0, 0])
    assert veq(q3 / 2.0, [x / 2 for x in q3.to_array()])
    assert veq(-q3, [-x for x in q3.to_array()])
    assert q3 == Quat.from_rotation_z(0.3)
    assert q3 != q4
    assert hash(q3) == hash(Quat.from_rotation_z(0.3))
    assert_raises(TypeError, lambda: q3 * "x")
    assert repr(q3).startswith("Quat(")
    assert str(q3).startswith("[")


def check_mat3_full():
    c0, c1, c2 = Vec3(1, 2, 3), Vec3(4, 5, 6), Vec3(7, 8, 10)
    m = Mat3.from_cols(c0, c1, c2)

    flat = m.to_cols_array()
    assert type(flat) is list
    assert flat == [1, 2, 3, 4, 5, 6, 7, 8, 10], "to_cols_array must be column-major"
    two_d = m.to_cols_array_2d()
    assert type(two_d) is list and all(type(col) is list for col in two_d)
    assert two_d == [[1, 2, 3], [4, 5, 6], [7, 8, 10]]
    assert Mat3.from_cols_array(flat) == m
    assert Mat3.from_cols_array_2d(two_d) == m

    assert m.x_axis == c0 and m.y_axis == c1 and m.z_axis == c2
    assert m.col(0) == c0 and m.col(1) == c1 and m.col(2) == c2
    assert m.row(0) == Vec3(1, 4, 7)
    assert m.row(1) == Vec3(2, 5, 8)
    assert m.row(2) == Vec3(3, 6, 10)
    assert_raises(IndexError, m.col, 3)
    assert_raises(IndexError, m.row, 3)
    assert m.diagonal() == Vec3(1, 5, 10)

    assert approx(m.determinant(), -3.0, 1e-12)
    assert m.transpose().to_cols_array() == [1, 4, 7, 2, 5, 8, 3, 6, 10]
    assert m.mul_mat3(m.inverse()).abs_diff_eq(Mat3.IDENTITY, 1e-9)
    diag = Mat3.from_diagonal(Vec3(2, 4, 5))
    assert diag.inverse().abs_diff_eq(Mat3.from_diagonal(Vec3(0.5, 0.25, 0.2)), 1e-12)
    assert Mat3.ZERO.try_inverse() is None
    assert m.try_inverse().abs_diff_eq(m.inverse(), 1e-12)
    assert Mat3.ZERO.inverse_or_zero() == Mat3.ZERO
    assert m.inverse_or_zero().abs_diff_eq(m.inverse(), 1e-12)

    assert veq(m.mul_vec3(Vec3(1, 1, 1)), [12, 15, 19])
    assert veq(m.mul_vec3(Vec3(1, 0, 0)), [1, 2, 3])
    assert veq(m.mul_transpose_vec3(Vec3(1, 1, 1)), [6, 15, 25])

    assert Mat3.from_diagonal(Vec3(2, 3, 4)).mul_vec3(Vec3(1, 1, 1)) == Vec3(2, 3, 4)
    assert (
        Mat3.from_quat(Quat.from_rotation_z(math.pi / 2))
        .mul_vec3(Vec3(1, 0, 0))
        .abs_diff_eq(Vec3(0, 1, 0), 1e-9)
    )
    assert (
        Mat3.from_axis_angle(Vec3(0, 0, 1), math.pi / 2)
        .mul_vec3(Vec3(1, 0, 0))
        .abs_diff_eq(Vec3(0, 1, 0), 1e-9)
    )
    assert Mat3.from_scaled_axis(Vec3(0, 0, math.pi / 2)).abs_diff_eq(
        Mat3.from_rotation_z(math.pi / 2), 1e-12
    )
    assert (
        Mat3.from_rotation_x(math.pi / 2)
        .mul_vec3(Vec3(0, 1, 0))
        .abs_diff_eq(Vec3(0, 0, 1), 1e-9)
    )
    assert (
        Mat3.from_rotation_y(math.pi / 2)
        .mul_vec3(Vec3(0, 0, 1))
        .abs_diff_eq(Vec3(1, 0, 0), 1e-9)
    )
    assert (
        Mat3.from_rotation_z(math.pi / 2)
        .mul_vec3(Vec3(1, 0, 0))
        .abs_diff_eq(Vec3(0, 1, 0), 1e-9)
    )

    a, b, c = 0.1, 0.2, 0.3
    me = Mat3.from_euler(EulerRot.XYZ, a, b, c)
    manual = Mat3.from_rotation_x(a).mul_mat3(
        Mat3.from_rotation_y(b).mul_mat3(Mat3.from_rotation_z(c))
    )
    assert me.abs_diff_eq(manual, 1e-12)
    ea, eb, ec = me.to_euler(EulerRot.XYZ)
    assert approx(ea, a) and approx(eb, b) and approx(ec, c)

    ang = Mat3.from_angle(math.pi / 2)
    assert ang.transform_point2(Vec2(1, 0)).abs_diff_eq(Vec2(0, 1), 1e-9)
    tr = Mat3.from_translation(Vec2(3, 4))
    assert tr.transform_point2(Vec2(1, 1)) == Vec2(4, 5)
    assert tr.transform_vector2(Vec2(1, 1)) == Vec2(1, 1)
    sc = Mat3.from_scale(Vec2(2, 3))
    assert sc.transform_point2(Vec2(1, 1)) == Vec2(2, 3)
    sat = Mat3.from_scale_angle_translation(Vec2(2, 3), 0.0, Vec2(5, 6))
    assert sat.transform_point2(Vec2(1, 1)).abs_diff_eq(Vec2(7, 9), 1e-12)

    m4 = Mat4.from_rotation_y(0.35)
    assert Mat3.from_mat4(m4).abs_diff_eq(Mat3.from_rotation_y(0.35), 1e-12)
    big = Mat4.from_cols(
        Vec4(1, 2, 3, 4), Vec4(5, 6, 7, 8), Vec4(9, 10, 11, 12), Vec4(13, 14, 15, 16)
    )
    minor = Mat3.from_mat4_minor(big, 3, 3)
    assert minor.to_cols_array() == [1, 2, 3, 5, 6, 7, 9, 10, 11]

    assert Mat3.look_to_rh(Vec3(0, 0, -1), Vec3(0, 1, 0)).abs_diff_eq(
        Mat3.IDENTITY, 1e-12
    )
    assert Mat3.look_to_lh(Vec3(0, 0, 1), Vec3(0, 1, 0)).abs_diff_eq(
        Mat3.IDENTITY, 1e-12
    )

    assert m.add_mat3(m) == m.mul_scalar(2.0)
    assert m.sub_mat3(m) == Mat3.ZERO
    assert m.mul_scalar(2.0).to_cols_array() == [2 * x for x in flat]
    assert m.div_scalar(2.0).to_cols_array() == [x / 2 for x in flat]
    scaled = m.mul_diagonal_scale(Vec3(2, 3, 4))
    assert scaled.col(0) == Vec3(2, 4, 6)
    assert scaled.col(1) == Vec3(12, 15, 18)
    assert scaled.col(2) == Vec3(28, 32, 40)

    assert (-m).abs() == m
    nz = Mat3.from_cols_array([1, 2, 4, 5, 8, 10, 16, 20, 25])
    assert cols_equal(
        nz.recip().to_cols_array(), [1 / x for x in nz.to_cols_array()], 1e-12
    )

    assert m.is_finite() and not m.is_nan()
    assert Mat3.NAN.is_nan() and not Mat3.NAN.is_finite()
    assert m.abs_diff_eq(Mat3.from_cols(c0, c1, c2), 1e-12)
    assert not m.abs_diff_eq(Mat3.IDENTITY, 1e-3)
    assert Mat3.IDENTITY.determinant() == 1.0
    assert Mat3.ZERO.to_cols_array() == [0.0] * 9

    assert (m + m) == m.add_mat3(m)
    assert (m - m) == Mat3.ZERO
    assert (m * 2.0) == m.mul_scalar(2.0)
    assert (2.0 * m) == m.mul_scalar(2.0)
    assert (m / 2.0) == m.div_scalar(2.0)
    assert (m * m) == m.mul_mat3(m)
    assert (m * Vec3(1, 1, 1)) == Vec3(12, 15, 19)
    assert m == Mat3.from_cols(c0, c1, c2)
    assert m != Mat3.IDENTITY
    assert hash(m) == hash(Mat3.from_cols(c0, c1, c2))
    assert repr(m).startswith("Mat3(")


def check_mat4_full():
    cols = [
        Vec4(2, 0, 0, 0),
        Vec4(0, 3, 0, 0),
        Vec4(0, 0, 4, 0),
        Vec4(1, 2, 3, 1),
    ]
    m = Mat4.from_cols(*cols)
    flat = m.to_cols_array()
    assert type(flat) is list
    assert flat == [2, 0, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 1, 2, 3, 1]
    two_d = m.to_cols_array_2d()
    assert type(two_d) is list and all(type(col) is list for col in two_d)
    assert two_d == [[2, 0, 0, 0], [0, 3, 0, 0], [0, 0, 4, 0], [1, 2, 3, 1]]
    assert Mat4.from_cols_array(flat) == m
    assert Mat4.from_cols_array_2d(two_d) == m

    assert m.x_axis == cols[0] and m.y_axis == cols[1]
    assert m.z_axis == cols[2] and m.w_axis == cols[3]
    for i in range(4):
        assert m.col(i) == cols[i]
    assert m.row(0) == Vec4(2, 0, 0, 1)
    assert m.row(3) == Vec4(0, 0, 0, 1)
    assert_raises(IndexError, m.col, 4)
    assert_raises(IndexError, m.row, 4)
    assert m.diagonal() == Vec4(2, 3, 4, 1)

    assert approx(m.determinant(), 24.0, 1e-12)
    assert m.transpose().row(0) == cols[0]
    assert m.transpose().col(0) == Vec4(2, 0, 0, 1)
    inv = m.inverse()
    assert m.mul_mat4(inv).abs_diff_eq(Mat4.IDENTITY, 1e-12)
    assert inv.transform_point3(Vec3(3, 5, 7)).abs_diff_eq(Vec3(1, 1, 1), 1e-12)
    assert Mat4.ZERO.try_inverse() is None
    assert m.try_inverse().abs_diff_eq(inv, 1e-12)
    assert Mat4.ZERO.inverse_or_zero() == Mat4.ZERO
    assert m.inverse_or_zero().abs_diff_eq(inv, 1e-12)

    assert veq(m.mul_vec4(Vec4(1, 1, 1, 1)), [3, 5, 7, 1])
    assert veq(m.mul_transpose_vec4(Vec4(1, 1, 1, 1)), [2, 3, 4, 7])
    assert m.transform_point3(Vec3(1, 1, 1)) == Vec3(3, 5, 7)
    assert m.transform_vector3(Vec3(1, 1, 1)) == Vec3(2, 3, 4)
    assert m.project_point3(Vec3(1, 1, 1)) == Vec3(3, 5, 7)

    assert Mat4.from_diagonal(Vec4(2, 3, 4, 1)).mul_vec4(Vec4(1, 1, 1, 1)) == Vec4(
        2, 3, 4, 1
    )
    q = Quat.from_rotation_z(math.pi / 2)
    t = Vec3(10, 20, 30)
    rt = Mat4.from_rotation_translation(q, t)
    assert rt.transform_point3(Vec3(1, 0, 0)).abs_diff_eq(Vec3(10, 21, 30), 1e-9)
    assert (
        Mat4.from_quat(q)
        .transform_vector3(Vec3(1, 0, 0))
        .abs_diff_eq(Vec3(0, 1, 0), 1e-9)
    )
    rot3 = Mat3.from_rotation_z(math.pi / 2)
    m3t = Mat4.from_mat3_translation(rot3, t)
    assert m3t.abs_diff_eq(rt, 1e-12)
    assert Mat4.from_mat3(rot3).abs_diff_eq(Mat4.from_quat(q), 1e-12)
    assert Mat4.from_translation(t).transform_point3(Vec3(1, 1, 1)) == Vec3(11, 21, 31)
    assert Mat4.from_translation(t).transform_vector3(Vec3(1, 1, 1)) == Vec3(1, 1, 1)
    assert (
        Mat4.from_axis_angle(Vec3(0, 0, 1), math.pi / 2)
        .transform_vector3(Vec3(1, 0, 0))
        .abs_diff_eq(Vec3(0, 1, 0), 1e-9)
    )
    assert Mat4.from_scale(Vec3(2, 3, 4)).transform_point3(Vec3(1, 1, 1)) == Vec3(
        2, 3, 4
    )
    assert (
        Mat4.from_rotation_x(math.pi / 2)
        .transform_vector3(Vec3(0, 1, 0))
        .abs_diff_eq(Vec3(0, 0, 1), 1e-9)
    )
    assert (
        Mat4.from_rotation_y(math.pi / 2)
        .transform_vector3(Vec3(0, 0, 1))
        .abs_diff_eq(Vec3(1, 0, 0), 1e-9)
    )
    assert (
        Mat4.from_rotation_z(math.pi / 2)
        .transform_vector3(Vec3(1, 0, 0))
        .abs_diff_eq(Vec3(0, 1, 0), 1e-9)
    )

    a, b, c = 0.1, 0.2, 0.3
    me = Mat4.from_euler(EulerRot.XYZ, a, b, c)
    manual = Mat4.from_rotation_x(a).mul_mat4(
        Mat4.from_rotation_y(b).mul_mat4(Mat4.from_rotation_z(c))
    )
    assert me.abs_diff_eq(manual, 1e-12)
    ea, eb, ec = me.to_euler(EulerRot.XYZ)
    assert approx(ea, a) and approx(eb, b) and approx(ec, c)

    scale, rot, trans = rt.to_scale_rotation_translation()
    assert scale.abs_diff_eq(Vec3(1, 1, 1), 1e-9)
    assert rot.abs_diff_eq(q, 1e-9)
    assert trans == t

    persp_rh = Mat4.perspective_rh(math.pi / 2, 2.0, 1.0, 5.0)
    r = 5.0 / (1.0 - 5.0)
    assert cols_equal(
        persp_rh.to_cols_array(),
        [0.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, r, -1, 0, 0, r * 1.0, 0],
        1e-12,
    )
    assert approx(persp_rh.project_point3(Vec3(0, 0, -1)).z, 0.0, 1e-12)
    assert approx(persp_rh.project_point3(Vec3(0, 0, -5)).z, 1.0, 1e-12)
    persp_lh = Mat4.perspective_lh(math.pi / 2, 2.0, 1.0, 5.0)
    assert approx(persp_lh.project_point3(Vec3(0, 0, 1)).z, 0.0, 1e-12)
    assert approx(persp_lh.project_point3(Vec3(0, 0, 5)).z, 1.0, 1e-12)
    inf_lh = Mat4.perspective_infinite_lh(math.pi / 2, 2.0, 1.0)
    assert approx(inf_lh.project_point3(Vec3(0, 0, 1)).z, 0.0, 1e-12)
    inf_rh = Mat4.perspective_infinite_rh(math.pi / 2, 2.0, 1.0)
    assert approx(inf_rh.project_point3(Vec3(0, 0, -1)).z, 0.0, 1e-12)

    eye, center, up = Vec3(0, 0, 5), Vec3(0, 0, 0), Vec3(0, 1, 0)
    view = Mat4.look_at_rh(eye, center, up)
    assert view.transform_point3(eye).abs_diff_eq(Vec3(0, 0, 0), 1e-12)
    assert view.transform_point3(center).abs_diff_eq(Vec3(0, 0, -5), 1e-12)
    view_lh = Mat4.look_at_lh(eye, center, up)
    assert view_lh.transform_point3(center).abs_diff_eq(Vec3(0, 0, 5), 1e-12)
    assert Mat4.look_to_rh(eye, Vec3(0, 0, -1), up).abs_diff_eq(view, 1e-12)
    assert Mat4.look_to_lh(eye, Vec3(0, 0, -1), up).abs_diff_eq(view_lh, 1e-12)

    assert m.add_mat4(m) == m.mul_scalar(2.0)
    assert m.sub_mat4(m) == Mat4.ZERO
    assert m.mul_scalar(2.0).to_cols_array() == [2 * x for x in flat]
    assert m.div_scalar(2.0).to_cols_array() == [x / 2 for x in flat]
    scaled = m.mul_diagonal_scale(Vec4(2, 3, 4, 5))
    for i, s in enumerate([2.0, 3.0, 4.0, 5.0]):
        assert scaled.col(i) == cols[i] * s

    assert (-m).abs() == m
    nz = Mat4.from_cols_array([2.0**k for k in range(16)])
    assert cols_equal(
        nz.recip().to_cols_array(), [1 / x for x in nz.to_cols_array()], 1e-12
    )

    assert m.is_finite() and not m.is_nan()
    assert Mat4.NAN.is_nan() and not Mat4.NAN.is_finite()
    assert m.abs_diff_eq(Mat4.from_cols(*cols), 1e-12)
    assert Mat4.IDENTITY.determinant() == 1.0
    assert Mat4.ZERO.to_cols_array() == [0.0] * 16

    assert (m + m) == m.add_mat4(m)
    assert (m - m) == Mat4.ZERO
    assert (m * 2.0) == m.mul_scalar(2.0)
    assert (2.0 * m) == m.mul_scalar(2.0)
    assert (m / 2.0) == m.div_scalar(2.0)
    assert (m * m) == m.mul_mat4(m)
    assert (m * Vec4(1, 1, 1, 1)) == Vec4(3, 5, 7, 1)
    assert m == Mat4.from_cols(*cols)
    assert m != Mat4.IDENTITY
    assert hash(m) == hash(Mat4.from_cols(*cols))
    assert repr(m).startswith("Mat4(")


def check_affine3_full():
    rot = Mat3.from_rotation_z(math.pi / 2)
    t = Vec3(10, 20, 30)

    a = Affine3(t, rot)
    assert a.translation == t, "Affine3 first positional argument is translation"
    assert a.matrix3.abs_diff_eq(rot, 1e-12)
    assert a.transform_point3(Vec3(1, 0, 0)).abs_diff_eq(Vec3(10, 21, 30), 1e-9)
    assert a.transform_vector3(Vec3(1, 0, 0)).abs_diff_eq(Vec3(0, 1, 0), 1e-9)

    assert Affine3(translation=t, rotation=rot) == a
    assert Affine3(rotation=rot, translation=t) == a
    assert Affine3(t, rotation=rot) == a
    assert Affine3.from_mat3_translation(rot, t) == a
    assert_raises(ValueError, Affine3)
    assert_raises(ValueError, Affine3, t)
    assert_raises(ValueError, Affine3, rotation=rot)
    assert_raises(TypeError, Affine3, t, rot, t)
    assert_raises(TypeError, Affine3, t, rot, bogus=1)

    x, y, z = Vec3(1, 2, 3), Vec3(4, 5, 6), Vec3(7, 8, 10)
    w = Vec3(11, 12, 13)
    fc = Affine3.from_cols(x, y, z, w)
    flat = fc.to_cols_array()
    assert type(flat) is list
    assert flat == [1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13]
    two_d = fc.to_cols_array_2d()
    assert type(two_d) is list and all(type(col) is list for col in two_d)
    assert two_d == [[1, 2, 3], [4, 5, 6], [7, 8, 10], [11, 12, 13]]
    assert Affine3.from_cols_array(flat) == fc
    assert Affine3.from_cols_array_2d(two_d) == fc
    assert fc.matrix3.to_cols_array() == flat[:9]
    assert fc.translation == w

    assert Affine3.from_scale(Vec3(2, 3, 4)).transform_point3(Vec3(1, 1, 1)) == Vec3(
        2, 3, 4
    )
    q = Quat.from_rotation_z(math.pi / 2)
    assert (
        Affine3.from_quat(q)
        .transform_vector3(Vec3(1, 0, 0))
        .abs_diff_eq(Vec3(0, 1, 0), 1e-9)
    )
    assert (
        Affine3.from_axis_angle(Vec3(0, 0, 1), math.pi / 2)
        .transform_vector3(Vec3(1, 0, 0))
        .abs_diff_eq(Vec3(0, 1, 0), 1e-9)
    )
    assert (
        Affine3.from_rotation_x(math.pi / 2)
        .transform_vector3(Vec3(0, 1, 0))
        .abs_diff_eq(Vec3(0, 0, 1), 1e-9)
    )
    assert (
        Affine3.from_rotation_y(math.pi / 2)
        .transform_vector3(Vec3(0, 0, 1))
        .abs_diff_eq(Vec3(1, 0, 0), 1e-9)
    )
    assert (
        Affine3.from_rotation_z(math.pi / 2)
        .transform_vector3(Vec3(1, 0, 0))
        .abs_diff_eq(Vec3(0, 1, 0), 1e-9)
    )
    ft = Affine3.from_translation(t)
    assert ft.transform_point3(Vec3(1, 1, 1)) == Vec3(11, 21, 31)
    assert ft.transform_vector3(Vec3(1, 1, 1)) == Vec3(1, 1, 1)
    fm = Affine3.from_mat3(rot)
    assert fm.matrix3.abs_diff_eq(rot, 1e-12)
    assert fm.translation == Vec3.ZERO

    jt = Affine3.just(Vec3(1, 2, 3))
    assert jt.translation == Vec3(1, 2, 3)
    assert jt.matrix3 == Mat3.IDENTITY
    jl = Affine3.just([1.0, 2.0, 3.0])
    assert jl == jt
    jr = Affine3.just(rot)
    assert jr.matrix3.abs_diff_eq(rot, 1e-12)
    assert jr.translation == Vec3.ZERO

    frt = Affine3.from_rotation_translation(q, t)
    assert frt.abs_diff_eq(a, 1e-12)
    assert Affine3.from_mat4(Mat4.from_rotation_translation(q, t)).abs_diff_eq(a, 1e-12)

    eye, center, up = Vec3(0, 0, 5), Vec3(0, 0, 0), Vec3(0, 1, 0)
    view = Affine3.look_at_rh(eye, center, up)
    assert view.transform_point3(eye).abs_diff_eq(Vec3(0, 0, 0), 1e-12)
    assert view.transform_point3(center).abs_diff_eq(Vec3(0, 0, -5), 1e-12)
    view_lh = Affine3.look_at_lh(eye, center, up)
    assert view_lh.transform_point3(center).abs_diff_eq(Vec3(0, 0, 5), 1e-12)
    assert Affine3.look_to_rh(eye, Vec3(0, 0, -1), up).abs_diff_eq(view, 1e-12)
    assert Affine3.look_to_lh(eye, Vec3(0, 0, -1), up).abs_diff_eq(view_lh, 1e-12)

    p = Vec3(0.5, -1.25, 2.0)
    assert a.inverse().transform_point3(a.transform_point3(p)).abs_diff_eq(p, 1e-9)
    assert (a * a.inverse()).abs_diff_eq(Affine3.IDENTITY, 1e-9)

    scale, rq, rt = a.to_scale_rotation_translation()
    assert scale.abs_diff_eq(Vec3(1, 1, 1), 1e-9)
    assert rq.abs_diff_eq(q, 1e-9)
    assert rt == t

    shift = Affine3.from_translation(Vec3(1, 0, 0))
    composed = a * shift
    assert composed.transform_point3(Vec3(0, 0, 0)).abs_diff_eq(
        a.transform_point3(Vec3(1, 0, 0)), 1e-12
    )

    assert a.is_finite() and not a.is_nan()
    assert Affine3.NAN.is_nan() and not Affine3.NAN.is_finite()
    assert Affine3.ZERO.to_cols_array() == [0.0] * 12
    assert Affine3.IDENTITY.transform_point3(p) == p
    assert a == Affine3(t, rot)
    assert a != Affine3.IDENTITY
    assert hash(a) == hash(Affine3(t, rot))
    assert repr(a).startswith("Affine3(")


def check_euler_rot_full():
    order = [
        "ZYX",
        "ZXY",
        "YXZ",
        "YZX",
        "XYZ",
        "XZY",
        "ZYZ",
        "ZXZ",
        "YXY",
        "YZY",
        "XYX",
        "XZX",
        "ZYXEx",
        "ZXYEx",
        "YXZEx",
        "YZXEx",
        "XYZEx",
        "XZYEx",
        "ZYZEx",
        "ZXZEx",
        "YXYEx",
        "YZYEx",
        "XYXEx",
        "XZXEx",
    ]
    for i, name in enumerate(order):
        variant = getattr(EulerRot, name)
        assert int(variant) == i, f"EulerRot.{name} must have discriminant {i}"
        assert variant == i
        assert variant == getattr(EulerRot, name)
    assert EulerRot.XYZ != EulerRot.ZYX
    assert EulerRot.XYZ != 0
    assert hash(EulerRot.XYZ) == hash(EulerRot.XYZ)
    assert EulerRot.XYZ in {EulerRot.XYZ, EulerRot.ZYX}

    expected = Quat.from_euler(EulerRot.XYZ, 0.1, 0.2, 0.3)
    foreign_int = type("EulerRot", (), {"__int__": lambda self: 4})()
    assert Quat.from_euler(foreign_int, 0.1, 0.2, 0.3) == expected
    foreign_name = type("EulerRot", (), {"name": "XYZ"})()
    assert Quat.from_euler(foreign_name, 0.1, 0.2, 0.3) == expected
    assert Mat3.from_euler(foreign_int, 0.1, 0.2, 0.3) == Mat3.from_euler(
        EulerRot.XYZ, 0.1, 0.2, 0.3
    )

    wrong_class = type("NotEuler", (), {"__int__": lambda self: 4})()
    assert_raises(TypeError, Quat.from_euler, wrong_class, 0.1, 0.2, 0.3)
    bad_name = type("EulerRot", (), {"name": "QQQ"})()
    assert_raises((ValueError, TypeError), Quat.from_euler, bad_name, 0.1, 0.2, 0.3)

    extrinsic = Quat.from_euler(EulerRot.XYZEx, 0.1, 0.2, 0.3)
    manual = (
        Quat.from_rotation_z(0.3)
        .mul_quat(Quat.from_rotation_y(0.2))
        .mul_quat(Quat.from_rotation_x(0.1))
    )
    assert extrinsic.abs_diff_eq(manual, 1e-12)


def check_serde_roundtrip():
    objects = [
        Vec3(1.5, -2.25, 3.125),
        Quat.from_rotation_z(0.3),
        Mat3.from_rotation_z(0.3),
        Affine3(Vec3(1, 2, 3), Mat3.from_rotation_x(0.4)),
    ]
    for obj in objects:
        cls = type(obj)
        assert cls.from_json(obj.to_json()).abs_diff_eq(obj, 1e-12)
        assert cls.try_from_json(obj.to_json()).abs_diff_eq(obj, 1e-12)
        assert cls.try_from_json("not valid json") is None
        assert cls.from_dict(obj.to_dict()) == obj
        assert cls.try_from_dict(obj.to_dict()) == obj
        assert cls.try_from_dict("bogus") is None
        assert_raises(ValueError, cls.from_json, "not valid json")


def check_getnewargs_roundtrip():
    objects = [
        Vec2(1.5, -2.25),
        Vec3(1.5, -2.25, 3.125),
        Vec4(1.5, -2.25, 3.125, -0.5),
        Quat.from_rotation_z(0.3),
        Mat3.from_rotation_z(0.3),
        Mat4.from_rotation_y(0.2),
        Affine3(Vec3(1, 2, 3), Mat3.from_rotation_x(0.4)),
    ]
    for obj in objects:
        args, kwargs = obj.__getnewargs_ex__()
        assert args == ()
        assert set(kwargs) == {"__pickle_state__"}
        clone = type(obj)(*args, **kwargs)
        assert clone == obj


def check_numpy_interop():
    assert np is not None
    assert Vec2.from_numpy(np.array([1.5, 2.5])) == Vec2(1.5, 2.5)
    assert Vec2.from_numpy(np.array([1, 2])) == Vec2(1.0, 2.0)
    assert Vec3.from_numpy(np.array([1.5, 2.5, 3.5])) == Vec3(1.5, 2.5, 3.5)
    assert Vec4.from_numpy(np.array([1.0, 2.0, 3.0, 4.0])) == Vec4(1, 2, 3, 4)
    assert Quat.from_numpy(np.array([0.0, 0.0, 0.6, 0.8])) == Quat.from_xyzw(
        0.0, 0.0, 0.6, 0.8
    )
    assert_raises(ValueError, Vec3.from_numpy, np.zeros(4))
    assert_raises(ValueError, Vec2.from_numpy, np.zeros(3))

    rows = np.array([[1.0, 4.0, 7.0], [2.0, 5.0, 8.0], [3.0, 6.0, 10.0]])
    m3 = Mat3.from_numpy(rows)
    assert m3.row(0) == Vec3(1, 4, 7), "Mat3.from_numpy must treat input as row-major"
    assert m3.col(0) == Vec3(1, 2, 3)
    assert m3.to_cols_array() == [1, 2, 3, 4, 5, 6, 7, 8, 10]
    assert_raises(ValueError, Mat3.from_numpy, np.zeros((4, 4)))

    rows4 = np.array(
        [
            [2.0, 0.0, 0.0, 1.0],
            [0.0, 3.0, 0.0, 2.0],
            [0.0, 0.0, 4.0, 3.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    )
    m4 = Mat4.from_numpy(rows4)
    assert m4.row(0) == Vec4(2, 0, 0, 1)
    assert m4.col(3) == Vec4(1, 2, 3, 1)
    assert_raises(ValueError, Mat4.from_numpy, np.zeros((3, 3)))

    aff_rows = np.array(
        [[1.0, 0.0, 0.0, 5.0], [0.0, 1.0, 0.0, 6.0], [0.0, 0.0, 1.0, 7.0]]
    )
    aff = Affine3.from_numpy(aff_rows)
    assert aff.translation == Vec3(5, 6, 7)
    assert aff.matrix3 == Mat3.IDENTITY
    assert_raises(ValueError, Affine3.from_numpy, np.zeros((4, 4)))

    v = Vec3(1.5, 2.5, 3.5)
    out = v.to_numpy()
    assert list(out) == [1.5, 2.5, 3.5]
    assert list(np.asarray(v)) == [1.5, 2.5, 3.5]
    q = Quat.from_xyzw(0.1, 0.2, 0.3, 0.4)
    assert list(np.asarray(q)) == [0.1, 0.2, 0.3, 0.4]


def main() -> None:
    check_vec3()
    check_vec2()
    check_vec4()
    check_quat()
    check_mat3()
    check_mat4()
    check_affine3()
    check_euler_rot()
    check_vec_constructor_forms()
    check_matrix_constructor_forms()
    check_vec_common_methods()
    check_vec_unary_methods()
    check_vec_operator_protocol()
    check_vec_duck_operands()
    check_vec2_specific()
    check_vec3_specific()
    check_vec4_specific()
    check_quat_full()
    check_mat3_full()
    check_mat4_full()
    check_affine3_full()
    check_euler_rot_full()
    check_serde_roundtrip()
    check_getnewargs_roundtrip()
    if np is not None:
        check_numpy_interop()
    print(
        "ok glam FromPyObject decode-invariance: vec2/3/4, quat, mat3/4, affine3, eulerrot"
    )
    print(
        "ok glam full-surface: constructors, methods, operators, serde, "
        + ("numpy" if np is not None else "numpy SKIPPED (not importable)")
    )
