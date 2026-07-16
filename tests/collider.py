import pickle
import sys
import types

CLASS_NAMES = (
    "Collider",
    "Sphere",
    "Capsule",
    "Cuboid",
    "Cylinder",
    "ConvexPolytope",
    "ConvexPolygon",
    "Line",
    "Ray",
    "LineSegment",
    "Plane",
    "Pointcloud",
    "SphereCollection",
    "Shape",
    "Vec3",
)


def _install_geomanpy(ns):
    """pickle resolves these classes as `geomanpy.<Name>`; the test registers
    them into a module named `wreck`, so alias one under the advertised name."""
    m = types.ModuleType("geomanpy")
    for name in CLASS_NAMES:
        setattr(m, name, ns[name])
    sys.modules["geomanpy"] = m


IDENTITY_AXES = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]

MASK_CAPSULES = 0x0001
MASK_CUBOIDS = 0x0002
MASK_CYLINDERS = 0x0004
MASK_PLANES = 0x0008
MASK_POLYGONS = 0x0010
MASK_POLYTOPES = 0x0020
MASK_SPHERES = 0x0080
MASK_LINES = 0x0100
MASK_RAYS = 0x0200
MASK_SEGMENTS = 0x0400
MASK_POINTCLOUDS = 0x0800


def _cube_polytope(center, half):
    cx, cy, cz = center
    planes = [
        ([1.0, 0.0, 0.0], cx + half),
        ([-1.0, 0.0, 0.0], -(cx - half)),
        ([0.0, 1.0, 0.0], cy + half),
        ([0.0, -1.0, 0.0], -(cy - half)),
        ([0.0, 0.0, 1.0], cz + half),
        ([0.0, 0.0, -1.0], -(cz - half)),
    ]
    verts = [
        [cx + sx * half, cy + sy * half, cz + sz * half]
        for sx in (-1.0, 1.0)
        for sy in (-1.0, 1.0)
        for sz in (-1.0, 1.0)
    ]
    return ConvexPolytope(planes, verts)


def _populated_collider():
    c = Collider()
    c.add(Sphere([0.0, 0.0, 0.0], 1.0))
    c.add(Capsule([0.0, 5.0, 0.0], [0.0, 6.0, 0.0], 0.5))
    c.add(
        Cuboid(
            [10.0, 0.0, 0.0],
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            [1.0, 1.0, 1.0],
        )
    )
    c.add(Cylinder([20.0, 0.0, 0.0], [20.0, 2.0, 0.0], 0.5))
    c.add(Plane([0.0, 0.0, 1.0], -30.0))
    c.add(Line([0.0, 40.0, 0.0], [1.0, 0.0, 0.0]))
    c.add(Ray([0.0, 50.0, 0.0], [1.0, 0.0, 0.0]))
    c.add(LineSegment([0.0, 60.0, 0.0], [1.0, 60.0, 0.0]))
    c.add(_cube_polytope([70.0, 0.0, 0.0], 1.0))
    c.add(Pointcloud([[80.0, 0.0, 0.0], [81.0, 0.0, 0.0]], (0.0, 5.0), 0.25))
    return c


class DuckCollider:
    """Structural stand-in for a Collider from a *different* module — same
    method surface, not the same class object, so `cast_exact` must miss and
    the attribute/method path must reconstruct it."""

    def __init__(self, src):
        self._src = src

    def spheres(self):
        return self._src.spheres()

    def capsules(self):
        return self._src.capsules()

    def cuboids(self):
        return self._src.cuboids()

    def cylinders(self):
        return self._src.cylinders()

    def planes(self):
        return self._src.planes()

    def polytopes(self):
        return self._src.polytopes()

    def polygons(self):
        return self._src.polygons()

    def lines(self):
        return self._src.lines()

    def rays(self):
        return self._src.rays()

    def segments(self):
        return self._src.segments()

    def pointclouds(self):
        return self._src.pointclouds()


class NotACollider:
    """No collider surface at all — must be rejected, not silently emptied."""

    def hello(self):
        return "world"


def _kind_counts(c):
    return {
        "spheres": len(c.spheres()),
        "capsules": len(c.capsules()),
        "cuboids": len(c.cuboids()),
        "cylinders": len(c.cylinders()),
        "planes": len(c.planes()),
        "polytopes": len(c.polytopes()),
        "polygons": len(c.polygons()),
        "lines": len(c.lines()),
        "rays": len(c.rays()),
        "segments": len(c.segments()),
        "pointclouds": len(c.pointclouds()),
    }


PROBES = [
    ("sphere_hit", lambda: Sphere([0.0, 0.0, 0.0], 0.5)),
    ("sphere_miss", lambda: Sphere([500.0, 500.0, 500.0], 0.5)),
    ("capsule_hit", lambda: Sphere([0.0, 5.5, 0.0], 0.5)),
    ("cuboid_hit", lambda: Sphere([10.0, 0.0, 0.0], 0.5)),
    ("cylinder_hit", lambda: Sphere([20.0, 1.0, 0.0], 0.5)),
    ("plane_hit", lambda: Sphere([0.0, 0.0, 30.0], 0.5)),
    ("line_hit", lambda: Sphere([5.0, 40.0, 0.0], 0.5)),
    ("ray_hit", lambda: Sphere([5.0, 50.0, 0.0], 0.5)),
    ("segment_hit", lambda: Sphere([0.5, 60.0, 0.0], 0.5)),
    ("polytope_hit", lambda: Sphere([70.0, 0.0, 0.0], 0.5)),
    ("pointcloud_hit", lambda: Sphere([80.0, 0.0, 0.0], 0.5)),
]


def _probe_map(c):
    return {name: c.collides(make()) for name, make in PROBES}


def test_duck_roundtrip(failures):
    src = _populated_collider()
    duck = Collider.from_any(DuckCollider(src))

    want, got = _kind_counts(src), _kind_counts(duck)
    for kind in want:
        if want[kind] != got[kind]:
            failures.append(f"duck: {kind} count {got[kind]} != native {want[kind]}")

    if src.mask() != duck.mask():
        failures.append(f"duck: mask 0x{duck.mask():04x} != native 0x{src.mask():04x}")

    want_p, got_p = _probe_map(src), _probe_map(duck)
    for name in want_p:
        if want_p[name] != got_p[name]:
            failures.append(
                f"duck: probe {name} -> {got_p[name]}, native -> {want_p[name]}"
            )


def test_duck_rejects_garbage(failures):
    try:
        c = Collider.from_any(NotACollider())
    except TypeError:
        return
    failures.append(
        f"duck: NotACollider silently accepted -> mask=0x{c.mask():04x}, "
        f"counts={_kind_counts(c)} (expected TypeError)"
    )


def test_pickle_roundtrip(failures):
    src = _populated_collider()
    for proto in (2, pickle.HIGHEST_PROTOCOL):
        try:
            blob = pickle.dumps(src, protocol=proto)
        except Exception as e:
            failures.append(f"pickle(proto={proto}): dumps raised {e!r}")
            continue
        try:
            back = pickle.loads(blob)
        except Exception as e:
            failures.append(f"pickle(proto={proto}): loads raised {e!r}")
            continue

        want, got = _kind_counts(src), _kind_counts(back)
        for kind in want:
            if want[kind] != got[kind]:
                failures.append(
                    f"pickle(proto={proto}): {kind} count {got[kind]} != {want[kind]}"
                )
        if src.mask() != back.mask():
            failures.append(
                f"pickle(proto={proto}): mask 0x{back.mask():04x} != 0x{src.mask():04x}"
            )
        want_p, got_p = _probe_map(src), _probe_map(back)
        for name in want_p:
            if want_p[name] != got_p[name]:
                failures.append(
                    f"pickle(proto={proto}): probe {name} -> {got_p[name]}, "
                    f"want {want_p[name]}"
                )


def test_pickle_empty(failures):
    try:
        back = pickle.loads(pickle.dumps(Collider()))
    except Exception as e:
        failures.append(f"pickle empty: raised {e!r}")
        return
    if back.mask() != 0:
        failures.append(f"pickle empty: mask 0x{back.mask():04x} != 0")


def test_pickle_shapes(failures):
    """Isolate which shape kind, if any, breaks the pickle round-trip."""
    cases = {
        "Sphere": Sphere([1.0, 2.0, 3.0], 1.0),
        "Capsule": Capsule([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.5),
        "Cuboid": Cuboid(
            [1.0, 2.0, 3.0],
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            [1.0, 1.0, 1.0],
        ),
        "Cylinder": Cylinder([0.0, 0.0, 0.0], [0.0, 2.0, 0.0], 0.5),
        "Plane": Plane([0.0, 0.0, 1.0], -1.0),
        "Line": Line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        "Ray": Ray([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        "LineSegment": LineSegment([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        "ConvexPolytope": _cube_polytope([0.0, 0.0, 0.0], 1.0),
        "Pointcloud": Pointcloud([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]], (0.0, 5.0), 0.25),
    }
    for name, obj in cases.items():
        try:
            pickle.loads(pickle.dumps(obj))
        except Exception as e:
            failures.append(f"pickle {name}: {e!r}")


def test_stretch_returns_concrete_shapes(failures):
    """`stretch()` must hand back the real shape classes. Under pyo3 it returns
    `Shape` enum variants, which are distinct classes that merely share the
    concrete classes' `__name__` — and they carry no `__getnewargs_ex__`."""
    real = {
        "Sphere": Sphere,
        "Capsule": Capsule,
        "Cuboid": Cuboid,
        "Cylinder": Cylinder,
        "ConvexPolytope": ConvexPolytope,
        "ConvexPolygon": ConvexPolygon,
        "Line": Line,
        "Ray": Ray,
        "LineSegment": LineSegment,
        "Plane": Plane,
    }
    sources = {
        "Sphere": Sphere([0.0, 0.0, 0.0], 1.0),
        "Capsule": Capsule([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.5),
        "Plane": Plane([0.0, 0.0, 1.0], -1.0),
        "Line": Line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        "Ray": Ray([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        "LineSegment": LineSegment([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        "Cuboid": Cuboid(
            [0.0, 0.0, 0.0],
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            [1.0, 1.0, 1.0],
        ),
        "Cylinder": Cylinder([0.0, 0.0, 0.0], [0.0, 2.0, 0.0], 0.5),
        "ConvexPolytope": _cube_polytope([0.0, 0.0, 0.0], 1.0),
    }
    for src_name, obj in sources.items():
        try:
            out = obj.stretch([1.0, 0.0, 0.0])
        except Exception as e:
            failures.append(f"stretch {src_name}: raised {e!r}")
            continue
        for piece in out:
            tname = type(piece).__name__
            expect = real.get(tname)
            if expect is not None and type(piece) is not expect:
                failures.append(
                    f"stretch {src_name}: returned {type(piece)!r} "
                    f"(qualname={type(piece).__qualname__}), not the real "
                    f"{tname} class"
                )
            if not hasattr(piece, "__getnewargs_ex__"):
                failures.append(
                    f"stretch {src_name}: {tname} piece has no __getnewargs_ex__"
                )


def test_pickle_stretch_results(failures):
    """Anything handed to Python must survive a pickle round-trip."""
    sources = {
        "Sphere": Sphere([0.0, 0.0, 0.0], 1.0),
        "Plane": Plane([0.0, 0.0, 1.0], -1.0),
        "Capsule": Capsule([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.5),
    }
    for src_name, obj in sources.items():
        try:
            out = obj.stretch([1.0, 0.0, 0.0])
        except Exception as e:
            failures.append(f"pickle stretch {src_name}: stretch raised {e!r}")
            continue
        for piece in out:
            try:
                pickle.loads(pickle.dumps(piece))
            except Exception as e:
                failures.append(
                    f"pickle stretch {src_name} -> {type(piece).__name__}: {e!r}"
                )


def test_stretch_result_feeds_collider(failures):
    """Whatever stretch returns should be re-addable to a Collider."""
    pieces = Plane([0.0, 0.0, 1.0], -1.0).stretch([1.0, 0.0, 0.0])
    c = Collider()
    for piece in pieces:
        try:
            c.add(piece)
        except Exception as e:
            failures.append(f"collider.add(stretch piece): {e!r}")
    if pieces and c.mask() == 0:
        failures.append("collider.add(stretch piece): mask still 0")


def _expect(failures, label, got, want):
    if got != want:
        failures.append(f"{label}: {got!r} != {want!r}")


def _expect_close(failures, label, got, want, tol=1e-5):
    if abs(got - want) > tol:
        failures.append(f"{label}: {got!r} != {want!r} (tol {tol})")


def _expect_vec(failures, label, v, want, tol=1e-5):
    got = (v.x, v.y, v.z)
    if any(abs(g - w) > tol for g, w in zip(got, want)):
        failures.append(f"{label}: {got} != {tuple(want)} (tol {tol})")


def _nested_collider():
    n = Collider()
    n.add(Cuboid([10.0, 0.0, 0.0], IDENTITY_AXES, [1.0, 1.0, 1.0]))
    n.add(Pointcloud([[80.0, 0.0, 0.0], [81.0, 0.0, 0.0]], (0.0, 5.0), 0.25))
    return n


def _foreign_pointcloud_clone(pc):
    """Structural clone from a different class also named Pointcloud; carries
    only the __getnewargs_ex__ surface, which is all a Pointcloud exposes."""
    cls = type(
        "Pointcloud",
        (),
        {"__getnewargs_ex__": lambda self: pc.__getnewargs_ex__()},
    )
    return cls()


def test_constructor_keyword_only(failures):
    c = Collider()
    _expect(failures, "Collider().mask()", c.mask(), 0)
    _expect(failures, "repr(Collider())", repr(c), "Collider(mask=0x0000)")
    for label, call in (
        ("Collider(1)", lambda: Collider(1)),
        ("Collider(Sphere(...))", lambda: Collider(Sphere([0.0, 0.0, 0.0], 1.0))),
        ("Collider([])", lambda: Collider([])),
        ("Collider(foo=1)", lambda: Collider(foo=1)),
    ):
        try:
            call()
        except TypeError:
            continue
        failures.append(f"{label} did not raise TypeError")


def test_mask_bits(failures):
    cases = [
        ("Sphere", Sphere([0.0, 0.0, 0.0], 1.0), MASK_SPHERES),
        ("Capsule", Capsule([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.5), MASK_CAPSULES),
        (
            "Cuboid",
            Cuboid([0.0, 0.0, 0.0], IDENTITY_AXES, [1.0, 1.0, 1.0]),
            MASK_CUBOIDS,
        ),
        ("Cylinder", Cylinder([0.0, 0.0, 0.0], [0.0, 2.0, 0.0], 0.5), MASK_CYLINDERS),
        ("Plane", Plane([0.0, 0.0, 1.0], -1.0), MASK_PLANES),
        (
            "ConvexPolygon",
            ConvexPolygon(
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]],
            ),
            MASK_POLYGONS,
        ),
        ("ConvexPolytope", _cube_polytope([0.0, 0.0, 0.0], 1.0), MASK_POLYTOPES),
        ("Line", Line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]), MASK_LINES),
        ("Ray", Ray([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]), MASK_RAYS),
        (
            "LineSegment",
            LineSegment([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
            MASK_SEGMENTS,
        ),
        (
            "Pointcloud",
            Pointcloud([[0.0, 0.0, 0.0]], (0.0, 5.0), 0.25),
            MASK_POINTCLOUDS,
        ),
    ]
    for name, shape, bit in cases:
        c = Collider()
        c.add(shape)
        _expect(failures, f"mask after add({name})", c.mask(), bit)

    full = _populated_collider()
    want = (
        MASK_SPHERES
        | MASK_CAPSULES
        | MASK_CUBOIDS
        | MASK_CYLINDERS
        | MASK_PLANES
        | MASK_LINES
        | MASK_RAYS
        | MASK_SEGMENTS
        | MASK_POLYTOPES
        | MASK_POINTCLOUDS
    )
    _expect(failures, "populated mask", full.mask(), want)
    _expect(failures, "populated repr", repr(full), f"Collider(mask=0x{want:04x})")


def test_add_include_refine_mutate_in_place(failures):
    c = Collider()
    ret = c.add(Sphere([1.0, 2.0, 3.0], 0.75))
    if ret is not None:
        failures.append(f"add() returned {ret!r}, expected None")
    _expect(failures, "mask after add", c.mask(), MASK_SPHERES)

    sc = c.spheres()
    _expect(failures, "len(spheres())", len(sc), 1)
    s0 = sc.get(0)
    _expect_vec(failures, "spheres().get(0).center", s0.center, (1.0, 2.0, 3.0))
    _expect_close(failures, "spheres().get(0).radius", s0.radius, 0.75)
    _expect_vec(failures, "spheres()[-1].center", sc[-1].center, (1.0, 2.0, 3.0))
    try:
        sc.get(1)
    except IndexError:
        pass
    else:
        failures.append("spheres().get(1) out of range did not raise IndexError")

    other = Collider()
    other.add(Cuboid([10.0, 0.0, 0.0], IDENTITY_AXES, [1.0, 1.0, 1.0]))
    ret = c.include(other)
    if ret is not None:
        failures.append(f"include() returned {ret!r}, expected None")
    _expect(failures, "mask after include", c.mask(), MASK_SPHERES | MASK_CUBOIDS)
    _expect(failures, "cuboids after include", len(c.cuboids()), 1)
    _expect(failures, "include source unchanged", other.mask(), MASK_CUBOIDS)

    ret = c.refine_bounding()
    if ret is not None:
        failures.append(f"refine_bounding() returned {ret!r}, expected None")
    _expect(
        failures, "mask after refine", c.mask(), MASK_SPHERES | MASK_CUBOIDS
    )


def test_getters_return_stored_shapes(failures):
    c = _populated_collider()
    for name in (
        "capsules",
        "cuboids",
        "cylinders",
        "planes",
        "polytopes",
        "polygons",
        "lines",
        "rays",
        "segments",
        "pointclouds",
    ):
        got = getattr(c, name)()
        if not isinstance(got, list):
            failures.append(f"{name}() returned {type(got).__name__}, expected list")

    cap = c.capsules()[0]
    _expect_vec(failures, "capsule.p1", cap.p1, (0.0, 5.0, 0.0))
    _expect_vec(failures, "capsule.p2", cap.p2, (0.0, 6.0, 0.0))
    _expect_close(failures, "capsule.radius", cap.radius, 0.5)

    cub = c.cuboids()[0]
    _expect_vec(failures, "cuboid.center", cub.center, (10.0, 0.0, 0.0))
    for i, h in enumerate(cub.half_extents):
        _expect_close(failures, f"cuboid.half_extents[{i}]", h, 1.0)

    cyl = c.cylinders()[0]
    _expect_vec(failures, "cylinder.p1", cyl.p1, (20.0, 0.0, 0.0))
    _expect_vec(failures, "cylinder.p2", cyl.p2, (20.0, 2.0, 0.0))
    _expect_close(failures, "cylinder.radius", cyl.radius, 0.5)
    _expect_close(failures, "cylinder.length()", cyl.length(), 2.0)

    pl = c.planes()[0]
    _expect_vec(failures, "plane.normal", pl.normal, (0.0, 0.0, 1.0))
    _expect_close(failures, "plane.d", pl.d, -30.0)

    ln = c.lines()[0]
    _expect_vec(failures, "line.origin", ln.origin, (0.0, 40.0, 0.0))
    _expect_vec(failures, "line.dir", ln.dir, (1.0, 0.0, 0.0))

    ry = c.rays()[0]
    _expect_vec(failures, "ray.origin", ry.origin, (0.0, 50.0, 0.0))
    _expect_vec(failures, "ray.dir", ry.dir, (1.0, 0.0, 0.0))

    seg = c.segments()[0]
    _expect_vec(failures, "segment.p1", seg.p1, (0.0, 60.0, 0.0))
    _expect_vec(failures, "segment.p2", seg.p2, (1.0, 60.0, 0.0))

    poly = c.polytopes()[0]
    _expect(failures, "polytope vertex count", len(poly.vertices), 8)

    _expect(failures, "pointcloud count", len(c.pointclouds()), 1)


def test_from_any_flattens_mixed_iterables(failures):
    nested = _nested_collider()
    mixed = Collider.from_any(
        [
            Sphere([0.0, 0.0, 0.0], 1.0),
            nested,
            Capsule([0.0, 5.0, 0.0], [0.0, 6.0, 0.0], 0.5),
        ]
    )
    _expect(
        failures,
        "mixed mask",
        mixed.mask(),
        MASK_SPHERES | MASK_CUBOIDS | MASK_POINTCLOUDS | MASK_CAPSULES,
    )
    _expect(failures, "mixed spheres", len(mixed.spheres()), 1)
    _expect(failures, "mixed cuboids", len(mixed.cuboids()), 1)
    _expect(failures, "mixed pointclouds", len(mixed.pointclouds()), 1)
    _expect(failures, "mixed capsules", len(mixed.capsules()), 1)

    probes = [
        ("own sphere", Sphere([0.0, 0.0, 0.0], 0.5), True),
        ("nested cuboid", Sphere([10.0, 0.0, 0.0], 0.5), True),
        ("nested pointcloud", Sphere([80.0, 0.0, 0.0], 0.5), True),
        ("own capsule", Sphere([0.0, 5.5, 0.0], 0.25), True),
        ("far miss", Sphere([500.0, 500.0, 500.0], 0.5), False),
    ]
    for label, probe, want in probes:
        _expect(failures, f"mixed collides ({label})", mixed.collides(probe), want)

    _expect(failures, "nested unchanged by from_any", len(nested.cuboids()), 1)

    gen = Collider.from_any(x for x in [Sphere([0.0, 0.0, 0.0], 1.0), nested])
    _expect(failures, "generator cuboids", len(gen.cuboids()), 1)
    _expect(failures, "generator spheres", len(gen.spheres()), 1)

    _expect(failures, "from_any(None) mask", Collider.from_any(None).mask(), 0)
    _expect(
        failures,
        "from_any(shape) mask",
        Collider.from_any(Sphere([0.0, 0.0, 0.0], 1.0)).mask(),
        MASK_SPHERES,
    )
    _expect(
        failures,
        "from_any(collider) mask",
        Collider.from_any(nested).mask(),
        nested.mask(),
    )
    try:
        Collider.from_any([42])
    except TypeError:
        pass
    else:
        failures.append("from_any([42]) did not raise TypeError")


def test_merge_and_with_any_flatten(failures):
    nested = _nested_collider()
    base = Collider()
    base.add(Sphere([0.0, 0.0, 0.0], 1.0))

    merged = base.merge([nested, LineSegment([0.0, 60.0, 0.0], [1.0, 60.0, 0.0])])
    _expect(
        failures,
        "merged mask",
        merged.mask(),
        MASK_SPHERES | MASK_CUBOIDS | MASK_POINTCLOUDS | MASK_SEGMENTS,
    )
    _expect(
        failures,
        "merged collides nested cuboid",
        merged.collides(Sphere([10.0, 0.0, 0.0], 0.5)),
        True,
    )
    _expect(
        failures,
        "merged collides nested pointcloud",
        merged.collides(Sphere([80.0, 0.0, 0.0], 0.5)),
        True,
    )
    _expect(failures, "merge left base mask untouched", base.mask(), MASK_SPHERES)
    _expect(failures, "merge left base cuboids untouched", len(base.cuboids()), 0)

    single = base.merge(Cuboid([10.0, 0.0, 0.0], IDENTITY_AXES, [1.0, 1.0, 1.0]))
    _expect(failures, "merge(shape) mask", single.mask(), MASK_SPHERES | MASK_CUBOIDS)

    w = base.with_any((nested,))
    _expect(
        failures,
        "with_any mask",
        w.mask(),
        MASK_SPHERES | MASK_CUBOIDS | MASK_POINTCLOUDS,
    )
    _expect(
        failures,
        "with_any collides nested cuboid",
        w.collides(Sphere([10.0, 0.0, 0.0], 0.5)),
        True,
    )
    _expect(failures, "with_any left base untouched", base.mask(), MASK_SPHERES)
    _expect(failures, "with_any(None) mask", base.with_any(None).mask(), MASK_SPHERES)


def test_collides_other(failures):
    a = Collider()
    a.add(Sphere([0.0, 0.0, 0.0], 1.0))
    near = Collider()
    near.add(Sphere([1.5, 0.0, 0.0], 1.0))
    far = Collider()
    far.add(Sphere([100.0, 0.0, 0.0], 1.0))
    _expect(failures, "a vs near", a.collides_other(near), True)
    _expect(failures, "near vs a", near.collides_other(a), True)
    _expect(failures, "a vs far", a.collides_other(far), False)
    _expect(failures, "empty vs a", Collider().collides_other(a), False)


def test_transform_delegation(failures):
    c = Collider()
    c.add(Sphere([1.0, 0.0, 0.0], 1.0))

    t = c.translated([2.0, 3.0, 4.0])
    s0 = t.spheres().get(0)
    _expect_vec(failures, "translated center", s0.center, (3.0, 3.0, 4.0))
    _expect_close(failures, "translated radius", s0.radius, 1.0)

    s = c.scaled(2.0)
    s0 = s.spheres().get(0)
    _expect_vec(failures, "scaled center", s0.center, (1.0, 0.0, 0.0))
    _expect_close(failures, "scaled radius", s0.radius, 2.0)

    rz90 = Mat3(Vec3(0.0, 1.0, 0.0), Vec3(-1.0, 0.0, 0.0), Vec3(0.0, 0.0, 1.0))
    r = c.rotated_mat(rz90)
    _expect_vec(failures, "rotated_mat center", r.spheres().get(0).center, (0.0, 1.0, 0.0))

    half_sqrt2 = 0.7071067811865476
    q = c.rotated_quat(Quat(0.0, 0.0, half_sqrt2, half_sqrt2))
    _expect_vec(failures, "rotated_quat center", q.spheres().get(0).center, (0.0, 1.0, 0.0))

    ident = Mat3(Vec3(1.0, 0.0, 0.0), Vec3(0.0, 1.0, 0.0), Vec3(0.0, 0.0, 1.0))
    tf = c.transformed(Affine3(Vec3(5.0, 6.0, 7.0), ident))
    _expect_vec(failures, "transformed center", tf.spheres().get(0).center, (6.0, 6.0, 7.0))

    s0 = c.spheres().get(0)
    _expect_vec(failures, "original center untouched", s0.center, (1.0, 0.0, 0.0))
    _expect_close(failures, "original radius untouched", s0.radius, 1.0)


def test_bounding_volumes(failures):
    c = Collider()
    c.add(Sphere([3.0, 0.0, 0.0], 1.0))
    bp = c.broadphase()
    _expect_vec(failures, "broadphase center", bp.center, (3.0, 0.0, 0.0))
    _expect_close(failures, "broadphase radius", bp.radius, 1.0)

    box = c.aabb()
    _expect_vec(failures, "aabb center", box.center, (3.0, 0.0, 0.0))
    for i, h in enumerate(box.half_extents):
        _expect_close(failures, f"aabb half_extents[{i}]", h, 1.0)
    if box.axis_aligned is not True:
        failures.append(f"aabb axis_aligned is {box.axis_aligned!r}, expected True")
    _expect_vec(failures, "obb center", c.obb().center, (3.0, 0.0, 0.0))

    c2 = Collider()
    c2.add(Sphere([0.0, 0.0, 0.0], 1.0))
    c2.add(Sphere([10.0, 0.0, 0.0], 1.0))
    bp2 = c2.broadphase()
    _expect_vec(failures, "expanded broadphase center", bp2.center, (5.0, 0.0, 0.0))
    _expect_close(failures, "expanded broadphase radius", bp2.radius, 6.0)

    c2.refine_bounding()
    bp3 = c2.broadphase()
    if bp3.radius > 6.0 + 1e-5:
        failures.append(f"refine_bounding inflated radius to {bp3.radius}")
    for cx in (0.0, 10.0):
        d = (
            (bp3.center.x - cx) ** 2 + bp3.center.y**2 + bp3.center.z**2
        ) ** 0.5
        if d + 1.0 > bp3.radius + 1e-4:
            failures.append(
                f"refined bounding does not enclose sphere at x={cx}: "
                f"dist+r={d + 1.0} > {bp3.radius}"
            )


def test_try_stretch_d(failures):
    c = Collider()
    c.add(Sphere([0.0, 0.0, 0.0], 1.0))
    st = c.try_stretch_d([10.0, 0.0, 0.0])
    if st is None:
        failures.append("try_stretch_d returned None for a pointcloud-free collider")
        return
    if not isinstance(st, Collider):
        failures.append(f"try_stretch_d returned {type(st).__name__}, expected Collider")
    _expect(
        failures,
        "stretched covers midpoint",
        st.collides(Sphere([5.0, 0.0, 0.0], 0.1)),
        True,
    )
    _expect(
        failures,
        "stretched misses off-axis",
        st.collides(Sphere([5.0, 3.0, 0.0], 0.1)),
        False,
    )
    _expect(
        failures,
        "original does not cover midpoint",
        c.collides(Sphere([5.0, 0.0, 0.0], 0.1)),
        False,
    )
    pcl = _populated_collider()
    if pcl.try_stretch_d([1.0, 0.0, 0.0]) is not None:
        failures.append("try_stretch_d with pointclouds should return None")


def test_foreign_pointcloud_duck(failures):
    pc = Pointcloud([[80.0, 0.0, 0.0], [81.0, 0.0, 0.0]], (0.0, 5.0), 0.25)

    c = Collider()
    try:
        ret = c.add(_foreign_pointcloud_clone(pc))
    except TypeError as e:
        failures.append(f"add(foreign Pointcloud) raised {e!r}")
    else:
        if ret is not None:
            failures.append(f"add(foreign Pointcloud) returned {ret!r}")
        _expect(failures, "foreign pcl mask", c.mask(), MASK_POINTCLOUDS)
        _expect(
            failures,
            "foreign pcl collides",
            c.collides(Sphere([80.0, 0.0, 0.0], 0.5)),
            True,
        )
        _expect(
            failures,
            "foreign pcl misses",
            c.collides(Sphere([500.0, 0.0, 0.0], 0.5)),
            False,
        )

    try:
        seq = Collider.from_any(
            [_foreign_pointcloud_clone(pc), Sphere([0.0, 0.0, 0.0], 1.0)]
        )
    except TypeError as e:
        failures.append(f"from_any([foreign Pointcloud, ...]) raised {e!r}")
    else:
        _expect(failures, "seq pointclouds", len(seq.pointclouds()), 1)
        _expect(failures, "seq spheres", len(seq.spheres()), 1)

    pc_col = Collider()
    pc_col.add(pc)
    try:
        duck = Collider.from_any(DuckCollider(pc_col))
    except TypeError as e:
        failures.append(f"from_any(duck collider with Pointcloud) raised {e!r}")
    else:
        _expect(failures, "duck pcl count", len(duck.pointclouds()), 1)
        _expect(
            failures,
            "duck pcl collides",
            duck.collides(Sphere([80.0, 0.0, 0.0], 0.5)),
            True,
        )

    foreign_only = type(
        "ForeignPclCollider",
        (),
        {"pointclouds": lambda self: [_foreign_pointcloud_clone(pc)]},
    )()
    try:
        deep = Collider.from_any(foreign_only)
    except TypeError as e:
        failures.append(f"from_any(collider-like with foreign Pointcloud) raised {e!r}")
    else:
        _expect(failures, "deep duck pcl count", len(deep.pointclouds()), 1)
        _expect(
            failures,
            "deep duck pcl collides",
            deep.collides(Sphere([80.0, 0.0, 0.0], 0.5)),
            True,
        )


def main() -> None:
    _install_geomanpy(globals())
    failures = []
    test_duck_roundtrip(failures)
    test_duck_rejects_garbage(failures)
    test_pickle_roundtrip(failures)
    test_pickle_empty(failures)
    test_pickle_shapes(failures)
    test_stretch_returns_concrete_shapes(failures)
    test_pickle_stretch_results(failures)
    test_stretch_result_feeds_collider(failures)
    test_constructor_keyword_only(failures)
    test_mask_bits(failures)
    test_add_include_refine_mutate_in_place(failures)
    test_getters_return_stored_shapes(failures)
    test_from_any_flattens_mixed_iterables(failures)
    test_merge_and_with_any_flatten(failures)
    test_collides_other(failures)
    test_transform_delegation(failures)
    test_bounding_volumes(failures)
    test_try_stretch_d(failures)
    test_foreign_pointcloud_duck(failures)

    if failures:
        print(f"\n{len(failures)} FAILURE(S):", flush=True)
        for f in failures:
            print(f"  - {f}", flush=True)
        raise AssertionError(f"{len(failures)} collider check(s) failed")
    print(
        "OK collider: duck-typing, pickle, constructor, flattening, "
        "in-place mutation, transforms, bounding",
        flush=True,
    )
