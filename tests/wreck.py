import math

TOL = 1e-5


def _field_names(obj):
    df = getattr(type(obj), "__dataclass_fields__", None)
    return list(df.keys()) if df else []


def expect(exc, fn, *args, **kwargs):
    names = (
        "/".join(e.__name__ for e in exc) if isinstance(exc, tuple) else exc.__name__
    )
    try:
        fn(*args, **kwargs)
    except exc:
        return
    except Exception as e:
        raise AssertionError(
            f"expected {names}, got {type(e).__name__}: {e}"
        ) from e
    raise AssertionError(f"expected {names}, but no exception was raised")


def close(a, b, tol=TOL, msg=""):
    assert abs(a - b) <= tol, f"{msg}: {a} != {b} (tol {tol})"


def assert_vec(v, xyz, tol=TOL, msg=""):
    got = (v.x, v.y, v.z)
    for g, e, ax in zip(got, xyz, "xyz"):
        assert abs(g - e) <= tol, f"{msg}: {ax}={g} expected {e} (full {got} vs {xyz})"


def duck(obj):
    """Foreign structural clone: same __name__ + same field attrs, different class."""
    names = _field_names(obj)
    assert names, f"{type(obj).__name__} exposes no __dataclass_fields__ to clone"
    foreign = type(type(obj).__name__, (), {})()
    for n in names:
        setattr(foreign, n, getattr(obj, n))
    return foreign


def _cube_polytope(center, half):
    cx, cy, cz = center
    h = half
    verts = [
        [cx + sx * h, cy + sy * h, cz + sz * h]
        for sx in (-1.0, 1.0)
        for sy in (-1.0, 1.0)
        for sz in (-1.0, 1.0)
    ]
    planes = [
        ([1.0, 0.0, 0.0], cx + h),
        ([-1.0, 0.0, 0.0], h - cx),
        ([0.0, 1.0, 0.0], cy + h),
        ([0.0, -1.0, 0.0], h - cy),
        ([0.0, 0.0, 1.0], cz + h),
        ([0.0, 0.0, -1.0], h - cz),
    ]
    return ConvexPolytope(planes, verts)


def _build_shapes():
    shapes = {}
    shapes["Sphere"] = Sphere([0.0, 0.0, 0.0], 1.0)
    shapes["Cuboid"] = Cuboid(
        [0.5, 0.0, 0.0],
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        [1.0, 1.0, 1.0],
    )
    shapes["Cylinder"] = Cylinder([0.0, -1.0, 0.0], [0.0, 1.0, 0.0], 0.5)
    shapes["Capsule"] = Capsule([0.0, 0.0, -1.0], [0.0, 0.0, 1.0], 0.5)
    shapes["Plane"] = Plane([0.0, 0.0, 1.0], 0.0)
    shapes["Line"] = Line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0])
    shapes["Ray"] = Ray([0.0, 0.0, 0.0], [1.0, 0.0, 0.0])
    shapes["LineSegment"] = LineSegment([-1.0, 0.0, 0.0], [1.0, 0.0, 0.0])
    shapes["ConvexPolytope"] = _cube_polytope([0.0, 0.0, 0.0], 2.0)
    shapes["ConvexPolygon"] = ConvexPolygon(
        [50.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]],
    )
    shapes["Pointcloud"] = Pointcloud.from_list(
        [[50.0, 50.0, 50.0], [51.0, 50.0, 50.0]], 0.1
    )
    return shapes


def test_required_constructor_args():
    expect(ValueError, Sphere)
    expect(ValueError, Sphere, [0.0, 0.0, 0.0])
    expect(ValueError, Sphere, center=[0.0, 0.0, 0.0])
    expect(ValueError, Sphere, radius=1.0)
    expect(ValueError, Cylinder, [0.0, 0.0, 0.0], [0.0, 1.0, 0.0])
    expect(ValueError, Cylinder, p1=[0.0, 0.0, 0.0], p2=[0.0, 1.0, 0.0])
    expect(ValueError, Capsule, [0.0, 0.0, 0.0], [0.0, 1.0, 0.0])
    expect(ValueError, Capsule, p1=[0.0, 0.0, 0.0], p2=[0.0, 1.0, 0.0])
    expect(ValueError, Cuboid)
    expect(ValueError, Cuboid, [0.0, 0.0, 0.0])
    expect(ValueError, Plane)
    expect(ValueError, Line, [0.0, 0.0, 0.0])
    expect(ValueError, Ray)
    expect(ValueError, LineSegment, [0.0, 0.0, 0.0])
    expect(ValueError, ConvexPolytope)
    expect(ValueError, ConvexPolytope, planes=[([1.0, 0.0, 0.0], 1.0)])
    expect(ValueError, ConvexPolygon)
    expect(ValueError, Pointcloud)

    s = Sphere(radius=2.0, center=[1.0, 2.0, 3.0])
    assert_vec(s.center, (1.0, 2.0, 3.0), msg="Sphere keyword center")
    assert s.radius == 2.0, "Sphere keyword radius"
    cyl = Cylinder(radius=0.5, p1=[0.0, 0.0, 0.0], p2=[0.0, 2.0, 0.0])
    assert cyl.radius == 0.5, "Cylinder keyword radius"
    assert_vec(cyl.p2, (0.0, 2.0, 0.0), msg="Cylinder keyword p2")
    cap = Capsule(radius=0.5, p2=[0.0, 2.0, 0.0], p1=[0.0, 0.0, 0.0])
    assert cap.radius == 0.5, "Capsule keyword radius"
    assert_vec(cap.p1, (0.0, 0.0, 0.0), msg="Capsule keyword p1")

    expect(TypeError, Sphere, [0.0, 0.0, 0.0], 1.0, 5)
    expect(TypeError, Sphere, center=[0.0, 0.0, 0.0], radius=1.0, bogus=1)
    expect(TypeError, Sphere, [0.0, 0.0, 0.0], radius="not a number")


def test_from_center_orientation():
    for cls in (Cylinder, Capsule):
        shp = cls.from_center_orientation(
            [1.0, 2.0, 3.0], Mat3.IDENTITY, length=10.0, radius=0.5
        )
        close(shp.length(), 10.0, tol=1e-4, msg=f"{cls.__name__} length")
        assert shp.radius == 0.5, f"{cls.__name__} radius: {shp.radius}"
        assert_vec(shp.p1, (1.0, -3.0, 3.0), msg=f"{cls.__name__} identity p1")
        assert_vec(shp.p2, (1.0, 7.0, 3.0), msg=f"{cls.__name__} identity p2")

        rot = Mat3.from_rotation_x(math.pi / 2.0)
        shp = cls.from_center_orientation([1.0, 2.0, 3.0], rot, 10.0, 0.5)
        assert_vec(shp.p1, (1.0, 2.0, -2.0), tol=1e-4, msg=f"{cls.__name__} rot p1")
        assert_vec(shp.p2, (1.0, 2.0, 8.0), tol=1e-4, msg=f"{cls.__name__} rot p2")
        y = rot.y_axis
        axis = (
            (shp.p2.x - shp.p1.x) / 10.0,
            (shp.p2.y - shp.p1.y) / 10.0,
            (shp.p2.z - shp.p1.z) / 10.0,
        )
        for a, b, ax in zip(axis, (y.x, y.y, y.z), "xyz"):
            close(a, b, tol=1e-4, msg=f"{cls.__name__} axis {ax} != orientation Y col")

    cyl = Cylinder.from_center_orientation(
        [1.0, 2.0, 3.0], Mat3.from_rotation_x(math.pi / 2.0), length=10.0, radius=0.5
    )
    center, orient = cyl.center_orientation()
    assert_vec(center, (1.0, 2.0, 3.0), tol=1e-4, msg="center_orientation center")
    assert_vec(orient.y_axis, (0.0, 0.0, 1.0), tol=1e-4, msg="center_orientation Y col")
    rebuilt = Cylinder.from_center_orientation(
        center, orient, length=cyl.length(), radius=cyl.radius
    )
    assert rebuilt.abs_diff_eq(cyl, 1e-4), "center_orientation round-trip"


def test_sphere_api():
    s = Sphere([1.0, 2.0, 3.0], 2.0)
    assert_vec(s.center, (1.0, 2.0, 3.0), msg="Sphere center")
    assert s.radius == 2.0
    assert isinstance(repr(s), str) and repr(s)

    bp = s.broadphase()
    assert_vec(bp.center, (1.0, 2.0, 3.0), msg="Sphere broadphase center")
    close(bp.radius, 2.0, msg="Sphere broadphase radius")
    box = s.aabb()
    assert box.axis_aligned is True
    assert_vec(box.center, (1.0, 2.0, 3.0), msg="Sphere aabb center")
    for he, ax in zip(box.half_extents, "xyz"):
        close(he, 2.0, msg=f"Sphere aabb half extent {ax}")
    assert isinstance(s.obb(), Cuboid)

    assert Sphere([0.0, 0.0, 0.0], 1.0).collides(Sphere([2.0, 0.0, 0.0], 1.0)) is True
    assert Sphere([0.0, 0.0, 0.0], 1.0).collides(Sphere([2.5, 0.0, 0.0], 1.0)) is False

    parts = Sphere([0.0, 0.0, 0.0], 1.0).stretch([4.0, 0.0, 0.0])
    assert isinstance(parts, list) and len(parts) == 1
    assert isinstance(parts[0], Capsule)
    assert_vec(parts[0].p1, (0.0, 0.0, 0.0), msg="Sphere stretch p1")
    assert_vec(parts[0].p2, (4.0, 0.0, 0.0), msg="Sphere stretch p2")
    assert parts[0].radius == 1.0
    still = Sphere([0.0, 0.0, 0.0], 1.0).stretch([0.0, 0.0, 0.0])
    assert len(still) == 1 and isinstance(still[0], Sphere)

    assert s.abs_diff_eq(Sphere([1.0, 2.0, 3.0], 2.0), 0.0) is True
    assert s.abs_diff_eq(Sphere([1.2, 2.0, 3.0], 2.0), 0.5) is True
    assert s.abs_diff_eq(Sphere([1.2, 2.0, 3.0], 2.0), 0.1) is False


def test_transforms():
    s = Sphere([1.0, 0.0, 0.0], 1.0)
    assert_vec(s.translated([1.0, 2.0, 3.0]).center, (2.0, 2.0, 3.0), msg="translated")
    assert Sphere([0.0, 0.0, 0.0], 1.0).scaled(2.0).radius == 2.0
    assert_vec(
        s.rotated_mat(Mat3.from_rotation_z(math.pi / 2.0)).center,
        (1.0, 0.0, 0.0),
        msg="Sphere rotation is body-frame: center stays put",
    )
    moved = s.transformed(
        Affine3(translation=Vec3(1.0, 2.0, 3.0), rotation=Mat3.IDENTITY)
    )
    assert_vec(moved.center, (2.0, 2.0, 3.0), msg="transformed")

    cyl = Cylinder([1.0, 0.0, 0.0], [2.0, 0.0, 0.0], 0.5)
    turned = cyl.rotated_mat(Mat3.from_rotation_z(math.pi / 2.0))
    assert_vec(turned.p1, (0.0, 1.0, 0.0), msg="Cylinder rotated_mat p1")
    assert_vec(turned.p2, (0.0, 2.0, 0.0), msg="Cylinder rotated_mat p2")
    half = math.pi / 4.0
    spun = cyl.rotated_quat(Quat(0.0, 0.0, math.sin(half), math.cos(half)))
    assert_vec(spun.p1, (0.0, 1.0, 0.0), msg="Cylinder rotated_quat p1")
    assert_vec(spun.p2, (0.0, 2.0, 0.0), msg="Cylinder rotated_quat p2")


def test_cylinder_api():
    cyl = Cylinder([0.0, -1.0, 0.0], [0.0, 1.0, 0.0], 0.5)
    assert_vec(cyl.p1, (0.0, -1.0, 0.0), msg="Cylinder p1")
    assert_vec(cyl.p2, (0.0, 1.0, 0.0), msg="Cylinder p2")
    assert cyl.radius == 0.5
    close(cyl.length(), 2.0, msg="Cylinder length")

    assert cyl.contains_point([0.0, 0.0, 0.0]) is True
    assert cyl.contains_point([0.5, 0.0, 0.0]) is True
    assert cyl.contains_point([0.6, 0.0, 0.0]) is False
    assert cyl.contains_point([0.0, 1.1, 0.0]) is False
    close(cyl.point_dist_sq([2.0, 0.0, 0.0]), 2.25, msg="Cylinder radial dist_sq")
    close(cyl.point_dist_sq([0.0, 3.0, 0.0]), 4.0, msg="Cylinder axial dist_sq")
    close(cyl.point_dist_sq([0.0, 0.0, 0.0]), 0.0, msg="Cylinder inside dist_sq")

    bc, br = cyl.bounding_sphere()
    assert_vec(bc, (0.0, 0.0, 0.0), msg="Cylinder bounding_sphere center")
    close(br, 1.5, msg="Cylinder bounding_sphere radius")

    aligned = Cylinder([0.0, 0.0, 0.0], [0.0, 4.0, 0.0], 1.0).stretch([0.0, 3.0, 0.0])
    assert len(aligned) == 1 and isinstance(aligned[0], Cylinder)
    assert_vec(aligned[0].p1, (0.0, 0.0, 0.0), msg="aligned stretch p1")
    assert_vec(aligned[0].p2, (0.0, 7.0, 0.0), msg="aligned stretch p2")
    unaligned = Cylinder([0.0, 0.0, 0.0], [0.0, 4.0, 0.0], 1.0).stretch([3.0, 0.0, 0.0])
    assert len(unaligned) == 5, f"unaligned stretch parts: {len(unaligned)}"
    assert all(isinstance(p, Capsule) for p in unaligned[:4])
    assert isinstance(unaligned[4], ConvexPolytope)

    assert cyl.collides(Sphere([1.5, 0.0, 0.0], 1.0)) is True
    assert cyl.collides(Sphere([1.6, 0.0, 0.0], 1.0)) is False
    assert isinstance(cyl.aabb(), Cuboid)
    assert isinstance(cyl.obb(), Cuboid)
    assert isinstance(cyl.broadphase(), Sphere)


def test_capsule_api():
    cap = Capsule([0.0, 0.0, -1.0], [0.0, 0.0, 1.0], 0.5)
    assert_vec(cap.p1, (0.0, 0.0, -1.0), msg="Capsule p1")
    assert_vec(cap.p2, (0.0, 0.0, 1.0), msg="Capsule p2")
    assert cap.radius == 0.5
    close(cap.length(), 2.0, msg="Capsule length")

    cp = cap.closest_point_to([3.0, 0.0, 5.0])
    assert_vec(cp, (0.0, 0.0, 1.0), msg="Capsule closest_point_to clamps to p2")
    cp = cap.closest_point_to([3.0, 4.0, 0.0])
    assert_vec(cp, (0.0, 0.0, 0.0), msg="Capsule closest_point_to mid segment")

    bc, br = cap.bounding_sphere()
    assert_vec(bc, (0.0, 0.0, 0.0), msg="Capsule bounding_sphere center")
    close(br, 1.5, msg="Capsule bounding_sphere radius")

    box = cap.aabb()
    assert_vec(box.center, (0.0, 0.0, 0.0), msg="Capsule aabb center")
    hx, hy, hz = box.half_extents
    close(hx, 0.5, msg="Capsule aabb hx")
    close(hy, 0.5, msg="Capsule aabb hy")
    close(hz, 1.5, msg="Capsule aabb hz")

    assert cap.collides(Sphere([0.0, 0.0, 2.5], 1.0)) is True
    assert cap.collides(Sphere([0.0, 0.0, 2.6], 1.0)) is False
    parts = cap.stretch([0.0, 0.0, 3.0])
    assert len(parts) == 1 and isinstance(parts[0], Capsule)
    assert cap.abs_diff_eq(Capsule([0.0, 0.0, -1.0], [0.0, 0.0, 1.0], 0.5), 0.0) is True


def test_cuboid_api():
    ident = Cuboid.from_center_size_orientation(
        [1.0, 2.0, 3.0], (2.0, 4.0, 6.0), Mat3.IDENTITY
    )
    assert ident.axis_aligned is True, "identity orientation must be axis aligned"
    assert ident.axes == [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
    assert ident.half_extents == [1.0, 2.0, 3.0]
    assert ident.full_extents == [2.0, 4.0, 6.0]
    assert_vec(ident.center, (1.0, 2.0, 3.0), msg="Cuboid center")

    direct = Cuboid(
        [1.0, 2.0, 3.0],
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        [1.0, 2.0, 3.0],
    )
    assert ident.abs_diff_eq(direct, 0.0) is True
    assert ident.aabb().abs_diff_eq(
        Cuboid.from_aabb([0.0, 0.0, 0.0], [2.0, 4.0, 6.0]), 0.0
    ) is True
    assert ident.obb().abs_diff_eq(ident, 0.0) is True
    assert direct.corners() == ident.corners()

    assert ident.contains_point([2.0, 4.0, 6.0]) is True
    assert ident.contains_point([2.2, 2.0, 3.0]) is False
    close(ident.point_dist_sq([4.0, 2.0, 3.0]), 4.0, msg="Cuboid point_dist_sq")
    close(ident.point_dist_sq([1.0, 2.0, 3.0]), 0.0, msg="Cuboid inside dist_sq")
    close(
        ident.bounding_sphere_radius(),
        math.sqrt(14.0),
        msg="Cuboid bounding_sphere_radius",
    )
    bp = ident.broadphase()
    assert_vec(bp.center, (1.0, 2.0, 3.0), msg="Cuboid broadphase center")
    close(bp.radius, math.sqrt(14.0), msg="Cuboid broadphase radius")

    rot = Mat3.from_cols(Vec3(0.0, 1.0, 0.0), Vec3(-1.0, 0.0, 0.0), Vec3(0.0, 0.0, 1.0))
    turned = Cuboid.from_center_size_orientation([1.0, 2.0, 3.0], (2.0, 4.0, 6.0), rot)
    assert turned.axis_aligned is False
    assert turned.axes == [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]
    assert_vec(turned.orientation.x_axis, (0.0, 1.0, 0.0), msg="Cuboid orientation col")

    signs = [
        (-1.0, -1.0, -1.0),
        (-1.0, -1.0, 1.0),
        (-1.0, 1.0, -1.0),
        (-1.0, 1.0, 1.0),
        (1.0, -1.0, -1.0),
        (1.0, -1.0, 1.0),
        (1.0, 1.0, -1.0),
        (1.0, 1.0, 1.0),
    ]
    corners = turned.corners()
    assert len(corners) == 8
    for corner, (sx, sy, sz) in zip(corners, signs):
        expected = (1.0 - 2.0 * sy, 2.0 + sx, 3.0 + 3.0 * sz)
        assert_vec(corner, expected, tol=1e-9, msg=f"corner for signs {(sx, sy, sz)}")

    box = turned.aabb()
    assert box.axis_aligned is True
    assert_vec(box.center, (1.0, 2.0, 3.0), msg="rotated aabb center")
    hx, hy, hz = box.half_extents
    close(hx, 2.0, msg="rotated aabb hx")
    close(hy, 1.0, msg="rotated aabb hy")
    close(hz, 3.0, msg="rotated aabb hz")

    fa = Cuboid.from_aabb([0.0, 0.0, 0.0], [2.0, 4.0, 6.0])
    assert fa.axis_aligned is True
    assert_vec(fa.center, (1.0, 2.0, 3.0), msg="from_aabb center")
    assert fa.half_extents == [1.0, 2.0, 3.0]

    parts = ident.stretch([1.0, 0.0, 0.0])
    assert parts and all(isinstance(p, (Cuboid, ConvexPolytope)) for p in parts)


def test_plane_api():
    assert Plane([0.0, 0.0, 1.0]).d == 0.0, "Plane d must default to 0.0"
    p = Plane(normal=[0.0, 0.0, 1.0], d=2.0)
    assert_vec(p.normal, (0.0, 0.0, 1.0), msg="Plane normal")
    assert p.d == 2.0

    fp = Plane.from_point_normal([0.0, 0.0, 5.0], [0.0, 0.0, 1.0])
    close(fp.d, 5.0, msg="from_point_normal d = normal.dot(point)")

    assert p.collides(Sphere([0.0, 0.0, 4.0], 1.0)) is False
    assert p.collides(Sphere([0.0, 0.0, 2.5], 1.0)) is True
    assert p.collides(Sphere([0.0, 0.0, -10.0], 1.0)) is True

    parts = p.stretch([0.0, 0.0, 3.0])
    assert len(parts) == 1 and isinstance(parts[0], Plane)
    close(parts[0].d, 5.0, msg="Plane stretch along normal grows d")
    parts = p.stretch([0.0, 0.0, -3.0])
    close(parts[0].d, 2.0, msg="Plane stretch against normal keeps d")


def test_line_ray_segment_api():
    ln = Line.from_points([1.0, 0.0, 0.0], [3.0, 0.0, 0.0])
    assert_vec(ln.origin, (1.0, 0.0, 0.0), msg="Line origin")
    assert_vec(ln.dir, (2.0, 0.0, 0.0), msg="Line dir = b - a, unnormalized")

    assert Line([0.0, 0.0, 5.0], [1.0, 0.0, 0.0]).collides(
        Sphere([0.0, 0.0, 0.0], 1.0)
    ) is False
    assert Line([0.0, 0.0, 0.5], [1.0, 0.0, 0.0]).collides(
        Sphere([0.0, 0.0, 0.0], 1.0)
    ) is True
    parts = Line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).stretch([2.0, 0.0, 0.0])
    assert len(parts) == 1 and isinstance(parts[0], Line)
    parts = Line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).stretch([0.0, 2.0, 0.0])
    assert len(parts) == 1 and isinstance(parts[0], ConvexPolygon)

    ray = Ray([2.0, 0.0, 0.0], [1.0, 0.0, 0.0])
    assert_vec(ray.origin, (2.0, 0.0, 0.0), msg="Ray origin")
    assert_vec(ray.dir, (1.0, 0.0, 0.0), msg="Ray dir")
    assert ray.collides(Sphere([0.0, 0.0, 0.0], 1.0)) is False
    assert Ray([2.0, 0.0, 0.0], [-1.0, 0.0, 0.0]).collides(
        Sphere([0.0, 0.0, 0.0], 1.0)
    ) is True

    seg = LineSegment([-1.0, 0.0, 0.0], [1.0, 0.0, 0.0])
    assert_vec(seg.p1, (-1.0, 0.0, 0.0), msg="LineSegment p1")
    assert_vec(seg.p2, (1.0, 0.0, 0.0), msg="LineSegment p2")
    bc, br = seg.bounding_sphere()
    assert_vec(bc, (0.0, 0.0, 0.0), msg="LineSegment bounding_sphere center")
    close(br, 1.0, msg="LineSegment bounding_sphere radius")
    box = seg.aabb()
    assert_vec(box.center, (0.0, 0.0, 0.0), msg="LineSegment aabb center")
    hx, hy, hz = box.half_extents
    close(hx, 1.0, msg="LineSegment aabb hx")
    close(hy, 0.0, msg="LineSegment aabb hy")
    close(hz, 0.0, msg="LineSegment aabb hz")
    assert isinstance(seg.obb(), Cuboid)
    assert isinstance(seg.broadphase(), Sphere)
    assert seg.collides(Sphere([1.5, 0.0, 0.0], 0.6)) is True
    assert seg.collides(Sphere([1.5, 0.0, 0.0], 0.4)) is False
    parts = seg.stretch([2.0, 0.0, 0.0])
    assert len(parts) == 1 and isinstance(parts[0], LineSegment)
    parts = seg.stretch([0.0, 2.0, 0.0])
    assert len(parts) == 1 and isinstance(parts[0], ConvexPolygon)


def test_convex_polytope_api():
    cube = _cube_polytope([0.0, 0.0, 0.0], 2.0)

    planes = cube.planes
    assert isinstance(planes, list) and len(planes) == 6
    assert isinstance(planes[0], tuple) and isinstance(planes[0][0], list)
    assert ([1.0, 0.0, 0.0], 2.0) in [(n, d) for n, d in planes]
    verts = cube.vertices
    assert isinstance(verts, list) and len(verts) == 8
    assert all(isinstance(v, list) and len(v) == 3 for v in verts)
    assert [-2.0, -2.0, -2.0] in verts

    assert callable(cube.obb), "obb must be a method, not a property value"
    obb = cube.obb()
    assert isinstance(obb, Cuboid)
    assert_vec(obb.center, (0.0, 0.0, 0.0), tol=1e-4, msg="polytope obb center")
    for v in verts:
        assert obb.point_dist_sq(v) <= 1e-6, f"obb must contain vertex {v}"
    box = cube.aabb()
    assert_vec(box.center, (0.0, 0.0, 0.0), tol=1e-4, msg="polytope aabb center")
    for he, ax in zip(box.half_extents, "xyz"):
        close(he, 2.0, tol=1e-4, msg=f"polytope aabb half extent {ax}")

    assert cube.collides(Sphere([1.9, 0.0, 0.0], 0.01)) is True
    assert cube.collides(Sphere([2.5, 0.0, 0.0], 0.01)) is False
    assert cube.collides(Sphere([3.0, 0.0, 0.0], 1.0)) is True

    given = Cuboid.from_aabb([-2.0, -2.0, -2.0], [2.0, 2.0, 2.0])
    p2 = ConvexPolytope.with_obb(planes, verts, given)
    assert p2.obb().abs_diff_eq(given, 1e-6) is True
    assert p2.collides(Sphere([0.0, 0.0, 0.0], 0.5)) is True

    def plane_d(poly, normal):
        for n, d in poly.planes:
            if all(abs(a - b) <= 1e-6 for a, b in zip(n, normal)):
                return d
        raise AssertionError(f"no plane with normal {normal}")

    swept = cube.swept([1.0, 0.0, 0.0])
    close(plane_d(swept, [1.0, 0.0, 0.0]), 3.0, msg="swept +x plane d")
    close(plane_d(swept, [-1.0, 0.0, 0.0]), 2.0, msg="swept -x plane d")
    offset = cube.swept([1.0, 0.0, 0.0], 0.5)
    close(plane_d(offset, [1.0, 0.0, 0.0]), 2.5, msg="swept plane_offset d")

    parts = cube.stretch([1.0, 0.0, 0.0])
    assert len(parts) == 1 and isinstance(parts[0], ConvexPolytope)
    assert cube.abs_diff_eq(_cube_polytope([0.0, 0.0, 0.0], 2.0), 0.0) is True


def test_convex_polygon_api():
    square = [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]]
    poly = ConvexPolygon([5.0, 5.0, 0.0], [0.0, 0.0, 1.0], square)
    assert_vec(poly.center, (5.0, 5.0, 0.0), msg="ConvexPolygon center")
    assert_vec(poly.normal, (0.0, 0.0, 1.0), msg="ConvexPolygon normal")
    v2 = poly.vertices_2d
    assert isinstance(v2, list) and len(v2) == 4
    assert all(isinstance(v, list) and len(v) == 2 for v in v2)
    assert v2[0] == [-1.0, -1.0]

    axed = ConvexPolygon.with_axes(
        [5.0, 5.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        square,
    )
    assert_vec(axed.u_axis, (1.0, 0.0, 0.0), msg="with_axes u_axis")
    assert_vec(axed.v_axis, (0.0, 1.0, 0.0), msg="with_axes v_axis")
    assert_vec(axed.center, (5.0, 5.0, 0.0), msg="with_axes center")

    assert axed.collides(Sphere([5.0, 5.0, 0.5], 0.6)) is True
    assert axed.collides(Sphere([5.0, 5.0, 0.5], 0.4)) is False
    assert axed.collides(Sphere([7.0, 5.0, 0.0], 0.5)) is False
    assert axed.collides(Sphere([6.3, 5.0, 0.0], 0.5)) is True

    box = axed.aabb()
    assert_vec(box.center, (5.0, 5.0, 0.0), tol=1e-4, msg="polygon aabb center")
    hx, hy, hz = box.half_extents
    close(hx, 1.0, tol=1e-4, msg="polygon aabb hx")
    close(hy, 1.0, tol=1e-4, msg="polygon aabb hy")
    close(hz, 0.0, tol=1e-4, msg="polygon aabb hz")
    assert isinstance(axed.obb(), Cuboid)
    assert isinstance(axed.broadphase(), Sphere)
    assert axed.abs_diff_eq(axed, 0.0) is True

    parts = axed.stretch([0.0, 0.0, 2.0])
    assert len(parts) == 1 and isinstance(parts[0], ConvexPolytope)
    parts = axed.stretch([1.0, 0.0, 0.0])
    assert len(parts) == 1 and isinstance(parts[0], ConvexPolygon)


def test_pointcloud_api():
    pc = Pointcloud.from_list([[0.0, 0.0, 0.0]])
    assert pc.collides(Sphere([0.05, 0.0, 0.0], 0.02)) is True, (
        "default point_radius must be 0.033 (0.05 <= 0.033 + 0.02)"
    )
    assert pc.collides(Sphere([0.06, 0.0, 0.0], 0.02)) is False, (
        "default point_radius must be 0.033 (0.06 > 0.033 + 0.02)"
    )

    wide = Pointcloud.from_list([[0.0, 0.0, 0.0]], point_radius=0.5)
    assert wide.collides(Sphere([0.9, 0.0, 0.0], 0.45)) is True
    assert wide.collides(Sphere([1.0, 0.0, 0.0], 0.45)) is False

    far = Pointcloud.from_list([[50.0, 50.0, 50.0], [51.0, 50.0, 50.0]], 0.1)
    assert far.collides(Sphere([50.5, 50.0, 50.0], 0.45)) is True, (
        "cloud far from the origin must still collide with a touching sphere"
    )
    assert far.collides(Sphere([50.0, 50.0, 48.0], 0.5)) is False
    assert Sphere([50.5, 50.0, 50.0], 0.45).collides(far) is True
    assert Sphere([45.0, 50.0, 50.0], 0.45).collides(far) is False

    ctor = Pointcloud([[50.0, 50.0, 50.0]], point_radius=0.1)
    assert ctor.collides(Sphere([50.4, 50.0, 50.0], 0.35)) is True
    assert ctor.collides(Sphere([50.6, 50.0, 50.0], 0.35)) is False

    other = Pointcloud.from_list([[50.05, 50.0, 50.0]], 0.1)
    assert far.collides(other) is True, "overlapping pointclouds must collide"
    distant = Pointcloud.from_list([[0.0, 0.0, 0.0]], 0.1)
    assert far.collides(distant) is False, "distant pointclouds must not collide"

    box = far.aabb()
    assert_vec(box.center, (50.5, 50.0, 50.0), tol=1e-4, msg="pointcloud aabb center")
    hx, hy, hz = box.half_extents
    close(hx, 0.6, tol=1e-4, msg="pointcloud aabb hx")
    close(hy, 0.1, tol=1e-4, msg="pointcloud aabb hy")
    close(hz, 0.1, tol=1e-4, msg="pointcloud aabb hz")
    bp = far.broadphase()
    assert_vec(bp.center, (50.5, 50.0, 50.0), tol=1e-4, msg="pointcloud broadphase")
    close(bp.radius, 0.6, tol=1e-4, msg="pointcloud broadphase radius")
    assert isinstance(far.obb(), Cuboid)

    moved = far.translated([1.0, 0.0, 0.0])
    assert moved.collides(Sphere([51.5, 50.0, 50.0], 0.45)) is True
    assert far.abs_diff_eq(far, 0.0) is True
    assert repr(far).startswith("Pointcloud")

    try:
        import numpy
    except ImportError:
        numpy = None
    if numpy is not None:
        pcn = Pointcloud.from_numpy(
            numpy.array([[50.0, 50.0, 50.0], [51.0, 50.0, 50.0]]), point_radius=0.1
        )
        assert pcn.collides(Sphere([50.5, 50.0, 50.0], 0.45)) is True
        assert pcn.collides(Sphere([50.0, 50.0, 48.0], 0.5)) is False


def test_sphere_collection_api():
    sc = SphereCollection()
    assert sc.is_empty() is True
    assert sc.len() == 0 and len(sc) == 0
    expect(IndexError, sc.get, 0)
    expect(TypeError, SphereCollection, 1)

    spheres = [
        Sphere([0.0, 0.0, 0.0], 1.0),
        Sphere([5.0, 0.0, 0.0], 1.0),
        Sphere([10.0, 0.0, 0.0], 1.0),
    ]
    for s in spheres:
        sc.push(s)
    assert sc.len() == 3 and len(sc) == 3
    assert sc.is_empty() is False
    assert_vec(sc.get(1).center, (5.0, 0.0, 0.0), msg="get(1)")
    assert sc.get(0).radius == 1.0
    expect(IndexError, sc.get, 3)
    expect(IndexError, sc.get, 100)
    expect((IndexError, OverflowError), sc.get, -1)

    assert_vec(sc[0].center, (0.0, 0.0, 0.0), msg="sc[0]")
    assert_vec(sc[-1].center, (10.0, 0.0, 0.0), msg="sc[-1]")
    assert_vec(sc[-3].center, (0.0, 0.0, 0.0), msg="sc[-3]")
    expect(IndexError, sc.__getitem__, 3)
    expect(IndexError, sc.__getitem__, -4)

    assert sc.any_collides_sphere(Sphere([4.0, 0.0, 0.0], 0.5)) is True
    assert sc.any_collides_sphere(Sphere([0.0, 50.0, 0.0], 1.0)) is False

    built = SphereCollection.from_slice(spheres)
    assert len(built) == 3
    assert built.abs_diff_eq(built, 0.0) is True
    assert_vec(built.get(2).center, (10.0, 0.0, 0.0), msg="from_slice get(2)")

    cap = SphereCollection.with_capacity(8)
    assert len(cap) == 0

    sc.clear()
    assert sc.len() == 0 and sc.is_empty() is True


def main() -> None:
    test_required_constructor_args()
    test_from_center_orientation()
    test_sphere_api()
    test_transforms()
    test_cylinder_api()
    test_capsule_api()
    test_cuboid_api()
    test_plane_api()
    test_line_ray_segment_api()
    test_convex_polytope_api()
    test_convex_polygon_api()
    test_pointcloud_api()
    test_sphere_collection_api()

    shapes = _build_shapes()
    names = list(shapes.keys())

    pairs = 0
    duck_pairs = 0
    for ln in names:
        lhs = shapes[ln]
        for rn in names:
            rhs = shapes[rn]
            if ln == "Pointcloud" and rn == "Pointcloud":
                continue
            native = lhs.collides(rhs)
            assert isinstance(native, bool), (
                f"{ln}.collides({rn}) returned non-bool {native!r}"
            )
            pairs += 1
            if not _field_names(rhs):
                continue
            ducked = lhs.collides(duck(rhs))
            assert ducked == native, (
                f"duck decode mismatch for {ln}.collides({rn}): "
                f"native={native} duck={ducked}"
            )
            duck_pairs += 1

    s2 = 0.5**0.5
    oriented = Cuboid(
        [0.0, 0.0, 0.0],
        [[s2, s2, 0.0], [-s2, s2, 0.0], [0.0, 0.0, 1.0]],
        [2.0, 0.5, 1.0],
    )
    assert oriented.axis_aligned is False, "oriented Cuboid should not be axis aligned"
    for ln in names:
        lhs = shapes[ln]
        native = lhs.collides(oriented)
        ducked = lhs.collides(duck(oriented))
        assert ducked == native, (
            f"oriented Cuboid duck mismatch vs {ln}: native={native} duck={ducked}"
        )

    collider_inputs = [shapes[n] for n in names if _field_names(shapes[n])]
    collider_native = Collider.from_any(collider_inputs)
    collider_duck = Collider.from_any([duck(s) for s in collider_inputs])
    probes = [
        Sphere([0.0, 0.0, 0.0], 0.5),
        Cuboid(
            [3.0, 0.0, 0.0],
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            [1.0, 1.0, 1.0],
        ),
        Sphere([50.0, 0.0, 0.0], 1.5),
        Plane([1.0, 0.0, 0.0], 0.0),
        Line([0.0, 5.0, 0.0], [1.0, 0.0, 0.0]),
        oriented,
    ]
    for p in probes:
        cn = collider_native.collides(p)
        cd = collider_duck.collides(p)
        assert isinstance(cn, bool), f"collider.collides({type(p).__name__}) non-bool"
        assert cn == cd, (
            f"collider duck mismatch for probe {type(p).__name__}: "
            f"native={cn} duck={cd}"
        )

    print(
        f"OK wreck shapes: per-class API suites passed, {pairs} native pairs, "
        f"{duck_pairs} duck-invariant pairs, {len(names)} oriented-cuboid checks, "
        f"{len(probes)} collider probes",
        flush=True,
    )
