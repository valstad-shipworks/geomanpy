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

    if failures:
        print(f"\n{len(failures)} FAILURE(S):", flush=True)
        for f in failures:
            print(f"  - {f}", flush=True)
        raise AssertionError(f"{len(failures)} collider check(s) failed")
    print("OK collider: duck-typing + pickle round-trips", flush=True)
