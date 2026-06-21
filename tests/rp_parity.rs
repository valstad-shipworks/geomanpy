//! Runtime smoke tests for the RustPython backend parity surface.
#![cfg(all(feature = "rustpython-backend", feature = "not_build_only"))]

use rustpython_vm::Interpreter;

fn interp() -> Interpreter {
    let b = Interpreter::builder(Default::default());
    let def = geomanpy::rustpython_bindings::make_module(&b.ctx);
    b.add_native_module(def).build()
}

fn run(source: &str) {
    let interp = interp();
    interp.enter(|vm| {
        let scope = vm.new_scope_with_builtins();
        let code = vm
            .compile(source, rustpython_vm::compiler::Mode::Exec, "<test>".into())
            .expect("compile");
        if let Err(e) = vm.run_code_obj(code, scope) {
            let mut s = String::new();
            let _ = vm.write_exception(&mut s, &e);
            panic!("run failed:\n{s}");
        }
    });
}

#[test]
fn vec2_full_surface() {
    run(r#"
from geomanpy import Vec2

v = Vec2(1.0, 2.0)
assert v.x == 1.0 and v.y == 2.0
assert len(v) == 2
assert v[0] == 1.0 and v[1] == 2.0 and v[-1] == 2.0

a = Vec2(1.0, 2.0) + Vec2(3.0, 4.0)
assert a == Vec2(4.0, 6.0)
assert a != Vec2(0.0, 0.0)
assert (Vec2(2.0, 0.0) * 3.0) == Vec2(6.0, 0.0)
assert (Vec2(6.0, 4.0) / 2.0) == Vec2(3.0, 2.0)
assert (-Vec2(1.0, -2.0)) == Vec2(-1.0, 2.0)
assert (Vec2(10.0, 10.0) - Vec2(1.0, 2.0)) == Vec2(9.0, 8.0)

assert Vec2.ZERO == Vec2(0.0, 0.0)
assert Vec2.ONE == Vec2(1.0, 1.0)
assert Vec2.X == Vec2(1.0, 0.0)

assert repr(Vec2(1.0, 2.0)) == "Vec2(1, 2)"
assert str(Vec2(1.0, 2.0)) == "[1, 2]"
assert abs(Vec2(3.0, 4.0).length() - 5.0) < 1e-9
assert Vec2(1.0, 2.0).dot(Vec2(3.0, 4.0)) == 11.0
assert Vec2(1.0, 2.0).extend(3.0).z == 3.0

# hashable -> usable as dict key / set member
d = {Vec2(1.0, 2.0): "p"}
assert d[Vec2(1.0, 2.0)] == "p"

# pickle round-trip through __getnewargs_ex__
_, kwargs = Vec2(1.5, -2.5).__getnewargs_ex__()
assert Vec2(**kwargs) == Vec2(1.5, -2.5)

# dataclass fields attribute exists (may be empty if stdlib dataclasses absent)
_ = Vec2(1.0, 2.0).__dataclass_fields__
"#);
}

#[test]
fn vec3_vec4_surface() {
    run(r#"
from geomanpy import Vec3, Vec4

a = Vec3(1.0, 2.0, 3.0)
assert len(a) == 3 and a[2] == 3.0 and a[-1] == 3.0
assert (a + Vec3(1.0, 1.0, 1.0)) == Vec3(2.0, 3.0, 4.0)
assert (a * 2.0) == Vec3(2.0, 4.0, 6.0)
assert Vec3.ZERO == Vec3(0.0, 0.0, 0.0)
assert Vec3.Z == Vec3(0.0, 0.0, 1.0)
assert Vec3(1.0, 0.0, 0.0).cross(Vec3(0.0, 1.0, 0.0)) == Vec3(0.0, 0.0, 1.0)
assert hash(Vec3(1.0, 2.0, 3.0)) == hash(Vec3(1.0, 2.0, 3.0))
# serde round trips (Vec3 has json/dict)
assert Vec3.from_json(a.to_json()) == a
assert Vec3.from_dict(a.to_dict()) == a
# pickle round trip
_, kw = a.__getnewargs_ex__()
assert Vec3(**kw) == a

b = Vec4(1.0, 2.0, 3.0, 4.0)
assert len(b) == 4 and b[3] == 4.0
assert (b + Vec4(1.0, 1.0, 1.0, 1.0)) == Vec4(2.0, 3.0, 4.0, 5.0)
assert Vec4.W == Vec4(0.0, 0.0, 0.0, 1.0)
assert b.truncate() == Vec3(1.0, 2.0, 3.0)
_, kw4 = b.__getnewargs_ex__()
assert Vec4(**kw4) == b
"#);
}

#[test]
fn quat_mat_affine_surface() {
    run(r#"
from geomanpy import Quat, Mat3, Mat4, Affine3, Vec3

# Quat: identity, multiplication, equality, hash, serde, pickle
q = Quat.IDENTITY
assert q == Quat.IDENTITY
assert (q * q) == q
assert (q * Vec3(1.0, 2.0, 3.0)) == Vec3(1.0, 2.0, 3.0)
assert hash(q) == hash(Quat.IDENTITY)
assert Quat.from_json(q.to_json()) == q
assert Quat.from_dict(q.to_dict()) == q
_, kw = q.__getnewargs_ex__()
assert Quat(**kw) == q

# Mat3
m = Mat3.IDENTITY
assert (m * m) == m
assert (m * Vec3(1.0, 2.0, 3.0)) == Vec3(1.0, 2.0, 3.0)
assert m == Mat3.IDENTITY
assert Mat3.from_json(m.to_json()) == m
_, kwm = m.__getnewargs_ex__()
assert Mat3(**kwm) == m

# Mat4 (no serde, but operators + pickle)
m4 = Mat4.IDENTITY
assert (m4 * m4) == m4
assert m4 == Mat4.IDENTITY
_, kwm4 = m4.__getnewargs_ex__()
assert Mat4(**kwm4) == m4

# Affine3
af = Affine3.IDENTITY
assert (af * af) == af
assert af == Affine3.IDENTITY
assert Affine3.from_json(af.to_json()) == af
_, kwa = af.__getnewargs_ex__()
assert Affine3(**kwa) == af

# EulerRot variants are accessible and usable
from geomanpy import EulerRot
q2 = Quat.from_euler(EulerRot.XYZ, 0.1, 0.2, 0.3)
ex, ey, ez = q2.to_euler(EulerRot.XYZ)
assert abs(ex - 0.1) < 1e-6 and abs(ey - 0.2) < 1e-6 and abs(ez - 0.3) < 1e-6
"#);
}

#[test]
fn sphere_wreck_surface() {
    run(r#"
from geomanpy import Sphere, Vec3

s = Sphere(Vec3(0.0, 0.0, 0.0), 1.0)
assert s.radius == 1.0
assert s.center == Vec3(0.0, 0.0, 0.0)

# collides with a concrete shape
near = Sphere(Vec3(0.5, 0.0, 0.0), 1.0)
far = Sphere(Vec3(10.0, 0.0, 0.0), 1.0)
assert s.collides(near) is True
assert s.collides(far) is False

# stretch returns a list of concrete shapes
parts = s.stretch(Vec3(0.0, 0.0, 0.0))
assert isinstance(parts, list) and len(parts) >= 1

# abs_diff_eq
assert s.abs_diff_eq(Sphere(Vec3(0.0, 0.0, 0.0), 1.0), 1e-6) is True

# pickle round trip
_, kw = s.__getnewargs_ex__()
s2 = Sphere(**kw)
assert s2.radius == 1.0 and s2.center == Vec3(0.0, 0.0, 0.0)

# dataclass fields attribute exists
_ = s.__dataclass_fields__
"#);
}

#[test]
fn wreck_containers_and_shapes() {
    run(r#"
from geomanpy import (
    Capsule, Cuboid, Line, Pointcloud, Collider, SphereCollection, Sphere, Vec3,
)

# Capsule: collides + stretch + pickle
cap = Capsule(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 2.0), 0.5)
assert cap.collides(Sphere(Vec3(0.0, 0.0, 0.0), 1.0)) is True
assert isinstance(cap.stretch(Vec3(1.0, 0.0, 0.0)), list)
_, kw = cap.__getnewargs_ex__()
assert Capsule(**kw).radius == 0.5

# Pointcloud collides rejects pointcloud arg
pc = Pointcloud.from_list([(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)], 0.1)
try:
    pc.collides(pc)
    raise AssertionError("expected ValueError for pointcloud-pointcloud")
except ValueError:
    pass

# SphereCollection: len/getitem/push(returns new)/any_collides_sphere/pickle
sc = SphereCollection.from_slice([Sphere(Vec3(0.0, 0.0, 0.0), 1.0)])
assert len(sc) == 1
assert sc[0].radius == 1.0
sc2 = sc.push(Sphere(Vec3(5.0, 0.0, 0.0), 1.0))
assert len(sc2) == 2
assert sc.any_collides_sphere(Sphere(Vec3(0.5, 0.0, 0.0), 1.0)) is True
_, kwsc = sc.__getnewargs_ex__()
assert len(SphereCollection(**kwsc)) == 1

# Collider: add(returns new), collides, pickle, try_stretch_d
col = Collider().add(Sphere(Vec3(0.0, 0.0, 0.0), 1.0))
assert col.collides(Sphere(Vec3(0.5, 0.0, 0.0), 1.0)) is True
assert col.mask() != 0
_, kwc = col.__getnewargs_ex__()
col2 = Collider(**kwc)
assert col2.mask() == col.mask()
_ = col.try_stretch_d(Vec3(1.0, 0.0, 0.0))
"#);
}

#[test]
fn any_shape_dispatch() {
    run(r#"
from geomanpy import Sphere, Capsule, Collider, Pointcloud, Vec3

s = Sphere(Vec3(0.0, 0.0, 0.0), 1.0)
cap = Capsule(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 2.0), 0.5)

# A single method accepts different concrete shape kinds and dispatches.
assert s.collides(s) is True
assert s.collides(cap) is True

# Collider queried against assorted shapes through the same path.
col = Collider().add(s).add(cap)
assert col.collides(s) is True

# A Collider<Pointcloud> rejects a Pointcloud query (ValueError, like pyo3).
pc = Pointcloud.from_list([(0.0, 0.0, 0.0)], 0.1)
try:
    col.collides(pc)
    raise AssertionError("expected ValueError for collider-vs-pointcloud")
except ValueError:
    pass

# A non-shape argument is rejected rather than silently mishandled.
try:
    s.collides(Vec3(0.0, 0.0, 0.0))
    raise AssertionError("expected an error for a non-shape argument")
except (TypeError, ValueError):
    pass
"#);
}

#[test]
fn shape_base_type_is_exposed() {
    run(r#"
from geomanpy import Shape, Sphere, Vec3

# Shape is a registered, importable type, matching the pyo3 backend.
assert isinstance(Shape, type)
assert Shape.__name__ == "Shape"

# stretch() yields concrete shapes directly; the Shape base is nominal
# (declared in the type stubs), so no runtime subclassing is involved.
parts = Sphere(Vec3(0.0, 0.0, 0.0), 1.0).stretch(Vec3(0.0, 0.0, 0.0))
assert isinstance(parts, list) and len(parts) >= 1
"#);
}

#[test]
fn domain_aliases_resolve_natively() {
    run(r#"
# The embedded module must expose the same public names as the CPython
# package facade, with no __init__.py layered on top.
from geomanpy import (
    Quaternion, Rotation3d, Translation3d, Transform3d,
    Box3d, Sphere3d, Cylinder3d, ObstacleUnion, PointCloud,
)
from geomanpy import Quat, Mat3, Vec3, Affine3, Cuboid, Sphere, Cylinder, Collider, Pointcloud

# Each alias is the very same type object as its canonical name.
assert Quaternion is Quat
assert Rotation3d is Mat3
assert Translation3d is Vec3
assert Transform3d is Affine3
assert Box3d is Cuboid
assert Sphere3d is Sphere
assert Cylinder3d is Cylinder
assert ObstacleUnion is Collider
assert PointCloud is Pointcloud

# And they are usable for construction through the alias.
assert Quaternion.IDENTITY == Quat.IDENTITY
assert Translation3d(1.0, 2.0, 3.0) == Vec3(1.0, 2.0, 3.0)
assert Sphere3d(Vec3(0.0, 0.0, 0.0), 1.0).radius == 1.0
"#);
}

#[test]
fn convenience_layer_surface() {
    run(r#"
HALF_PI = 1.5707963267948966
from geomanpy import Vec3, Mat3, Quat, Affine3, Cuboid, Cylinder, ConvexPolytope, Collider, Sphere

# Vec3.from_spherical: rotation applied to +X, scaled by radius
v = Vec3.from_spherical(Mat3.IDENTITY, 2.5)
assert v == Vec3(2.5, 0.0, 0.0)

# Vec3.angle: rotation taking the reference axis onto the vector's direction
r = Vec3(0.0, 1.0, 0.0).angle()
assert (r * Vec3(1.0, 0.0, 0.0)).distance(Vec3(0.0, 1.0, 0.0)) < 1e-9
assert (Vec3(1.0, 0.0, 0.0).angle()) == Mat3.IDENTITY
r2 = Vec3(0.0, 0.0, 1.0).angle(Vec3(0.0, 1.0, 0.0))
assert (r2 * Vec3(0.0, 1.0, 0.0)).distance(Vec3(0.0, 0.0, 1.0)) < 1e-9

# Mat3.from_scaled_axis: rotation from a rotation vector (axis * angle)
m = Mat3.from_scaled_axis(Vec3(0.0, 0.0, HALF_PI))
assert (m * Vec3(1.0, 0.0, 0.0)).distance(Vec3(0.0, 1.0, 0.0)) < 1e-9
assert Mat3.from_scaled_axis(Vec3(0.0, 0.0, 0.0)) == Mat3.IDENTITY

# Affine3.just: lone translation or lone rotation
assert Affine3.just(Vec3(1.0, 2.0, 3.0)) == Affine3.from_translation(Vec3(1.0, 2.0, 3.0))
assert Affine3.just(Mat3.IDENTITY) == Affine3.from_mat3(Mat3.IDENTITY)

# Cuboid.orientation: column axes as a Mat3
cub = Cuboid.from_aabb(Vec3(-1.0, -1.0, -1.0), Vec3(1.0, 1.0, 1.0))
assert cub.orientation == Mat3.IDENTITY

# Cylinder.center_orientation: reconstruct (center, orientation)
cyl = Cylinder(Vec3(0.0, -1.0, 0.0), Vec3(0.0, 1.0, 0.0), 0.5)
c, o = cyl.center_orientation()
assert c.distance(Vec3(0.0, 0.0, 0.0)) < 1e-9
assert o == Mat3.IDENTITY

# ConvexPolytope.swept: concrete sweep, accepted by a Collider.
# An oriented cuboid stretch yields a real ConvexPolytope to sweep.
oriented = Cuboid.from_aabb(Vec3(-1.0, -1.0, -1.0), Vec3(1.0, 1.0, 1.0)).rotated_mat(
    Mat3.from_rotation_z(0.5)
)
# World +X is not one of the rotated cuboid's axes, so the stretch is
# unaligned and yields a concrete ConvexPolytope.
poly = [p for p in oriented.stretch(Vec3(5.0, 0.0, 0.0)) if isinstance(p, ConvexPolytope)][0]
swept = poly.swept(Vec3(5.0, 0.0, 0.0))
assert isinstance(swept, ConvexPolytope)
shrunk = poly.swept(Vec3(5.0, 0.0, 0.0), 0.1)
# Shrinking pulls every plane inward, so the offset constants drop.
assert all(s[1] <= w[1] + 1e-6 for s, w in zip(shrunk.planes(), swept.planes()))
col = Collider().add(swept)  # rustpython add returns a new Collider
assert col.collides(Sphere(Vec3(0.0, 0.0, 0.0), 0.5))

# Collider.from_any / merge / with_any
empty = Collider.from_any(None)
assert empty.mask() == 0
one = Collider.from_any(Sphere(Vec3(0.0, 0.0, 0.0), 1.0))
assert one.collides(Sphere(Vec3(1.0, 0.0, 0.0), 0.5))
many = Collider.from_any([Sphere(Vec3(0.0, 0.0, 0.0), 1.0), Cuboid.from_aabb(Vec3(4.0, 4.0, 4.0), Vec3(6.0, 6.0, 6.0))])
assert many.collides(Sphere(Vec3(5.0, 5.0, 5.0), 0.5))
merged = one.merge(many)
assert merged.collides(Sphere(Vec3(5.0, 5.0, 5.0), 0.5))
extended = one.with_any(Cuboid.from_aabb(Vec3(4.0, 4.0, 4.0), Vec3(6.0, 6.0, 6.0)))
assert extended.collides(Sphere(Vec3(5.0, 5.0, 5.0), 0.5))
"#);
}

#[test]
fn squiggle_curve_surface() {
    run(r#"
from geomanpy import (
    Vec3,
    Interval,
    Nearest,
    QuadraticBezier,
    CubicBezier,
    Polyline,
    Spline,
    Cuboid,
    LineSegment,
)

# Interval value type
i = Interval(0.0, 2.0)
assert i.min == 0.0 and i.max == 2.0
assert i.span() == 2.0
assert i.clamp(3.0) == 2.0
assert i.lerp(0.5) == 1.0
assert i.contains(1.0) and not i.contains(5.0)
assert i.is_finite()
assert not Interval.all().is_finite()
assert Interval.unit().max == 1.0

# A cubic Bézier rising along a straight diagonal is parameterized so its
# endpoints land on the first and last control points.
cb = CubicBezier(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0), Vec3(2.0, 0.0, 0.0), Vec3(3.0, 0.0, 0.0))
assert len(cb.points) == 4
assert cb.domain().max == 1.0
start, end = cb.endpoints()
assert start == Vec3(0.0, 0.0, 0.0)
assert end == Vec3(3.0, 0.0, 0.0)
assert abs(cb.point(0.5).x - 1.5) < 1e-5
left, right = cb.split(0.5)
assert isinstance(left, CubicBezier) and isinstance(right, CubicBezier)
moved = cb.translated(Vec3(0.0, 1.0, 0.0))
assert abs(moved.point(0.0).y - 1.0) < 1e-5
assert isinstance(cb.aabb(), Cuboid)

n = cb.nearest(Vec3(1.5, 1.0, 0.0))
assert isinstance(n, Nearest)
assert abs(n.point.x - 1.5) < 1e-2
assert abs(n.dist_sq - 1.0) < 1e-2
assert abs(n.distance() - 1.0) < 1e-2

qb = QuadraticBezier(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 1.0, 0.0), Vec3(2.0, 0.0, 0.0))
assert len(qb.points) == 3
assert qb.reversed().point(0.0) == Vec3(2.0, 0.0, 0.0)

# Polyline: an L of two unit legs has length 2 and yields two segments.
pl = Polyline([Vec3(0.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0), Vec3(1.0, 1.0, 0.0)])
assert abs(pl.length() - 2.0) < 1e-5
segs = pl.segments()
assert len(segs) == 2 and isinstance(segs[0], LineSegment)
assert abs(pl.point_at_distance(1.0).x - 1.0) < 1e-5
assert pl.abs_diff_eq(Polyline(pl.points), 1e-6)

# Spline interpolates its knots.
sp = Spline([Vec3(0.0, 0.0, 0.0), Vec3(1.0, 1.0, 0.0), Vec3(2.0, 0.0, 0.0)])
assert len(sp.points) == 3
assert abs(sp.point(0.5).y - 1.0) < 1e-5
assert isinstance(sp.scaled(2.0), Spline)
"#);
}
