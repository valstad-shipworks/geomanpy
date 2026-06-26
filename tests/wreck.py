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


def main() -> None:
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
        f"OK wreck shapes: {pairs} native pairs, {duck_pairs} duck-invariant pairs, "
        f"{len(names)} oriented-cuboid checks, {len(probes)} collider probes",
        flush=True,
    )
