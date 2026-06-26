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
    return type(type(m).__name__, (), {"to_cols_array": lambda self, _c=cols: list(_c)})()


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

    assert cols_equal(Mat3.from_quat(native).to_cols_array(), Mat3.from_quat(d).to_cols_array())
    assert cols_equal(
        Mat3.from_quat(native).to_cols_array(), Mat3.from_quat(listform).to_cols_array()
    )
    assert cols_equal(Mat4.from_quat(native).to_cols_array(), Mat4.from_quat(d).to_cols_array())


def check_mat3():
    orientation = Mat3.from_rotation_z(0.5)
    center = Vec3(1.0, 2.0, 3.0)
    size = (2.0, 4.0, 6.0)

    c_native = Cuboid.from_center_size_orientation(center, size, orientation)
    c_duck = Cuboid.from_center_size_orientation(center, size, duck_cols(orientation))
    assert c_native.center == c_duck.center
    assert cols_equal(c_native.orientation.to_cols_array(), c_duck.orientation.to_cols_array())
    assert cols_equal(c_native.half_extents, c_duck.half_extents)

    assert cols_equal(
        Mat4.from_mat3(orientation).to_cols_array(),
        Mat4.from_mat3(duck_cols(orientation)).to_cols_array(),
    )


def check_mat4():
    native = Mat4.from_rotation_y(0.35)
    d = duck_cols(native)
    assert cols_equal(Mat3.from_mat4(native).to_cols_array(), Mat3.from_mat4(d).to_cols_array())
    assert cols_equal(
        Affine3.from_mat4(native).to_cols_array(), Affine3.from_mat4(d).to_cols_array()
    )


def check_affine3():
    rot = Mat3.from_rotation_x(0.4)
    trans = Vec3(5.0, -2.0, 1.0)
    native = Affine3.from_mat3_translation(rot, trans)
    d = duck_affine(native)
    assert cols_equal(Quat.from_affine3(native).to_array(), Quat.from_affine3(d).to_array())


def check_euler_rot():
    a = Mat3.from_euler(EulerRot.XYZ, 0.1, 0.2, 0.3)
    b = Mat3.from_euler(EulerRot.XYZ, 0.1, 0.2, 0.3)
    assert cols_equal(a.to_cols_array(), b.to_cols_array())
    assert approx(a.determinant(), 1.0)

    q = Quat.from_euler(EulerRot.ZYX, 0.1, 0.2, 0.3)
    assert approx(q.length(), 1.0)


def main() -> None:
    check_vec3()
    check_vec2()
    check_vec4()
    check_quat()
    check_mat3()
    check_mat4()
    check_affine3()
    check_euler_rot()
    print("ok glam FromPyObject decode-invariance: vec2/3/4, quat, mat3/4, affine3, eulerrot")
