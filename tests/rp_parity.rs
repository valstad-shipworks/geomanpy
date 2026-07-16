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

# Pointcloud-pointcloud collision
pc = Pointcloud.from_list([(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)], 0.1)
assert pc.collides(pc) is True
pc_far = Pointcloud.from_list([(50.0, 0.0, 0.0)], 0.1)
assert pc.collides(pc_far) is False

# SphereCollection: len/getitem/push/clear mutate in place/any_collides_sphere/pickle
sc = SphereCollection.from_slice([Sphere(Vec3(0.0, 0.0, 0.0), 1.0)])
assert len(sc) == 1
assert sc[0].radius == 1.0
assert sc.push(Sphere(Vec3(5.0, 0.0, 0.0), 1.0)) is None
assert len(sc) == 2
assert sc.any_collides_sphere(Sphere(Vec3(0.5, 0.0, 0.0), 1.0)) is True
_, kwsc = sc.__getnewargs_ex__()
assert len(SphereCollection(**kwsc)) == 2
assert sc.clear() is None
assert len(sc) == 0

# Collider: add/include/refine_bounding mutate in place; collides, pickle, try_stretch_d
col = Collider()
assert col.add(Sphere(Vec3(0.0, 0.0, 0.0), 1.0)) is None
assert col.collides(Sphere(Vec3(0.5, 0.0, 0.0), 1.0)) is True
assert col.mask() != 0
mask_before = col.mask()
other = Collider()
other.add(Capsule(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 2.0), 0.5))
assert col.include(other) is None
assert col.mask() != mask_before
assert col.include(col) is None
assert col.refine_bounding() is None
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
col = Collider()
col.add(s)
col.add(cap)
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
fn keyword_constructors_bind_like_positionals() {
    run(r#"
from geomanpy import Vec2, Vec3, Vec4, Quat, Sphere, Cylinder, Capsule, Plane

assert Vec2(x=1.0, y=2.0) == Vec2(1.0, 2.0)
assert Vec2(1.0) == Vec2(1.0, 0.0)
assert Vec2() == Vec2(0.0, 0.0)
assert Vec2(y=3.0) == Vec2(0.0, 3.0)
assert Vec3(x=1.0, z=3.0) == Vec3(1.0, 0.0, 3.0)
assert Vec3(1.0, 2.0) == Vec3(1.0, 2.0, 0.0)
assert Vec4(w=4.0) == Vec4(0.0, 0.0, 0.0, 4.0)
assert Vec4(1.0, y=2.0) == Vec4(1.0, 2.0, 0.0, 0.0)

q = Quat(w=0.5)
assert q.x == 0.0 and q.y == 0.0 and q.z == 0.0 and q.w == 0.5
assert Quat() == Quat.IDENTITY

s = Sphere(center=Vec3(1.0, 2.0, 3.0), radius=2.5)
assert s.center == Vec3(1.0, 2.0, 3.0) and s.radius == 2.5
s2 = Sphere(radius=2.5, center=Vec3(1.0, 2.0, 3.0))
assert s2.center == Vec3(1.0, 2.0, 3.0) and s2.radius == 2.5

cyl = Cylinder(p1=Vec3(0.0, 0.0, 0.0), p2=Vec3(0.0, 0.0, 4.0), radius=1.5)
assert cyl.radius == 1.5
assert cyl.p1 == Vec3(0.0, 0.0, 0.0) and cyl.p2 == Vec3(0.0, 0.0, 4.0)
assert abs(cyl.length() - 4.0) < 1e-5

cap = Capsule(radius=0.25, p1=Vec3(0.0, 0.0, 0.0), p2=Vec3(3.0, 0.0, 0.0))
assert cap.radius == 0.25 and cap.p2 == Vec3(3.0, 0.0, 0.0)

p = Plane(normal=Vec3(0.0, 0.0, 1.0))
assert p.d == 0.0 and p.normal == Vec3(0.0, 0.0, 1.0)
assert Plane(Vec3(0.0, 1.0, 0.0), d=2.0).d == 2.0
"#);
}

#[test]
fn constructor_required_args_and_bad_kwargs() {
    run(r#"
from geomanpy import (
    Vec2, Vec3, Mat3, Mat4, Affine3, Sphere, Cylinder, Capsule, Cuboid,
    Collider, Pointcloud,
)

def raises(exc, fn):
    try:
        fn()
    except exc:
        return True
    except Exception as e:
        raise AssertionError("expected %s, got %r" % (exc.__name__, e))
    raise AssertionError("expected %s, nothing raised" % exc.__name__)

assert raises(ValueError, lambda: Sphere(Vec3(0.0, 0.0, 0.0)))
assert raises(ValueError, lambda: Sphere())
assert raises(TypeError, lambda: Sphere(Vec3(0.0, 0.0, 0.0), 1.0, 2.0))
assert raises(TypeError, lambda: Vec2(x=1.0, q=2.0))
assert raises(TypeError, lambda: Vec2(1.0, 2.0, 3.0))
assert raises(TypeError, lambda: Vec2(1.0, x=2.0))
assert raises(ValueError, lambda: Mat3())
assert raises(ValueError, lambda: Mat4())
assert raises(ValueError, lambda: Affine3())
assert raises(ValueError, lambda: Cylinder(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 1.0)))
assert raises(ValueError, lambda: Capsule(p1=Vec3(0.0, 0.0, 0.0), p2=Vec3(0.0, 0.0, 1.0)))
assert raises(
    TypeError,
    lambda: Capsule(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 1.0), 0.5, p1=Vec3(0.0, 0.0, 0.0)),
)
assert raises(ValueError, lambda: Cuboid())
assert raises(
    TypeError,
    lambda: Cuboid(
        center=Vec3(0.0, 0.0, 0.0),
        axes=[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        half_extents=[1.0, 1.0, 1.0],
        bogus=1,
    ),
)
assert raises(TypeError, lambda: Collider(1))
assert raises(ValueError, lambda: Pointcloud())
"#);
}

#[test]
fn cylinder_capsule_from_center_orientation() {
    run(r#"
from geomanpy import Cylinder, Capsule, Mat3, Vec3

c = Vec3(1.0, 2.0, 3.0)
m = Mat3.IDENTITY

cyl = Cylinder.from_center_orientation(c, m, 10.0, 0.5)
assert cyl.radius == 0.5
assert abs(cyl.length() - 10.0) < 1e-5
assert (cyl.p1 - Vec3(1.0, -3.0, 3.0)).length() < 1e-5
assert (cyl.p2 - Vec3(1.0, 7.0, 3.0)).length() < 1e-5
assert (cyl.p2 - cyl.p1 - (m * Vec3(0.0, 10.0, 0.0))).length() < 1e-5

cc, orient = cyl.center_orientation()
assert (cc - c).length() < 1e-5
assert (orient * Vec3(0.0, 1.0, 0.0) - Vec3(0.0, 1.0, 0.0)).length() < 1e-5

cap = Capsule.from_center_orientation(c, m, 10.0, 0.5)
assert cap.radius == 0.5
assert abs(cap.length() - 10.0) < 1e-5
assert (cap.p1 - Vec3(1.0, -3.0, 3.0)).length() < 1e-5
assert (cap.p2 - Vec3(1.0, 7.0, 3.0)).length() < 1e-5
"#);
}

#[test]
fn to_array_and_cols_arrays_return_lists() {
    run(r#"
from geomanpy import Vec2, Vec3, Vec4, Quat, Mat3

a2 = Vec2(1.0, 2.0).to_array()
assert isinstance(a2, list) and a2 == [1.0, 2.0]
a3 = Vec3(1.0, 2.0, 3.0).to_array()
assert isinstance(a3, list) and a3 == [1.0, 2.0, 3.0]
a4 = Vec4(1.0, 2.0, 3.0, 4.0).to_array()
assert isinstance(a4, list) and a4 == [1.0, 2.0, 3.0, 4.0]
aq = Quat.IDENTITY.to_array()
assert isinstance(aq, list) and aq == [0.0, 0.0, 0.0, 1.0]

ca = Mat3.IDENTITY.to_cols_array()
assert isinstance(ca, list)
assert ca == [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
ca2 = Mat3.IDENTITY.to_cols_array_2d()
assert isinstance(ca2, list) and isinstance(ca2[0], list)
assert ca2 == [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
"#);
}

#[test]
fn vec3_normalize_and_length_and_negative_zero_hash() {
    run(r#"
from geomanpy import Vec2, Vec3, Vec4

nv, l = Vec3(3.0, 0.0, 0.0).normalize_and_length()
assert nv == Vec3(1.0, 0.0, 0.0)
assert abs(l - 3.0) < 1e-9
nv2, l2 = Vec3(0.0, 4.0, 3.0).normalize_and_length()
assert abs(l2 - 5.0) < 1e-9
assert abs(nv2.y - 0.8) < 1e-9 and abs(nv2.z - 0.6) < 1e-9

v = Vec3(-0.0, 0.0, 0.0)
w = Vec3(0.0, 0.0, 0.0)
assert v == w
assert hash(v) == hash(w)
assert hash(Vec2(-0.0, 0.0)) == hash(Vec2(0.0, 0.0))
assert hash(Vec4(-0.0, -0.0, 0.0, 0.0)) == hash(Vec4(0.0, 0.0, 0.0, 0.0))
"#);
}

#[test]
fn cuboid_real_construction() {
    run(r#"
from geomanpy import Cuboid, Vec3

axes = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
c = Cuboid(center=Vec3(1.0, 2.0, 3.0), axes=axes, half_extents=[0.5, 1.0, 2.0])
assert c.center == Vec3(1.0, 2.0, 3.0)

he = c.half_extents
assert isinstance(he, list) and he == [0.5, 1.0, 2.0]
ax = c.axes
assert isinstance(ax, list) and isinstance(ax[0], list)
assert ax == axes
fe = c.full_extents
assert isinstance(fe, list) and fe == [1.0, 2.0, 4.0]
assert c.axis_aligned is True

assert c.contains_point(Vec3(1.0, 2.0, 3.0)) is True
assert c.contains_point(Vec3(10.0, 2.0, 3.0)) is False
assert len(c.corners()) == 8

c2 = Cuboid(Vec3(1.0, 2.0, 3.0), axes, [0.5, 1.0, 2.0])
assert c2.half_extents == he and c2.center == c.center
"#);
}

#[test]
fn convex_polygon_construction() {
    run(r#"
from geomanpy import ConvexPolygon, Vec3

verts = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
poly = ConvexPolygon(center=Vec3(0.0, 0.0, 1.0), normal=Vec3(0.0, 0.0, 1.0), vertices_2d=verts)
assert poly.center == Vec3(0.0, 0.0, 1.0)
assert poly.normal == Vec3(0.0, 0.0, 1.0)
v2 = poly.vertices_2d
assert isinstance(v2, list) and isinstance(v2[0], list)
assert v2 == [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]]

p2 = ConvexPolygon.with_axes(
    Vec3(0.0, 0.0, 0.0),
    Vec3(0.0, 0.0, 1.0),
    Vec3(1.0, 0.0, 0.0),
    Vec3(0.0, 1.0, 0.0),
    verts,
)
assert p2.u_axis == Vec3(1.0, 0.0, 0.0)
assert p2.v_axis == Vec3(0.0, 1.0, 0.0)
assert p2.vertices_2d == [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]]
"#);
}

#[test]
fn convex_polytope_construction() {
    run(r#"
from geomanpy import ConvexPolytope, Cuboid, Sphere, Vec3

planes = [
    ((1.0, 0.0, 0.0), 0.5), ((-1.0, 0.0, 0.0), 0.5),
    ((0.0, 1.0, 0.0), 0.5), ((0.0, -1.0, 0.0), 0.5),
    ((0.0, 0.0, 1.0), 0.5), ((0.0, 0.0, -1.0), 0.5),
]
verts = [
    (x, y, z)
    for x in (-0.5, 0.5)
    for y in (-0.5, 0.5)
    for z in (-0.5, 0.5)
]
pt = ConvexPolytope(planes=planes, vertices=verts)

vs = pt.vertices
assert isinstance(vs, list) and len(vs) == 8 and isinstance(vs[0], list)
assert sorted(map(tuple, vs)) == sorted(verts)
ps = pt.planes
assert isinstance(ps, list) and len(ps) == 6
n0, d0 = ps[0]
assert isinstance(n0, list) and n0 == [1.0, 0.0, 0.0] and d0 == 0.5

assert isinstance(pt.obb(), Cuboid)
assert pt.collides(Sphere(Vec3(0.0, 0.0, 0.0), 0.25)) is True
assert pt.collides(Sphere(Vec3(5.0, 0.0, 0.0), 0.25)) is False

obb = Cuboid.from_aabb(Vec3(-0.5, -0.5, -0.5), Vec3(0.5, 0.5, 0.5))
pt2 = ConvexPolytope.with_obb(planes, verts, obb)
assert len(pt2.vertices) == 8
assert pt2.obb().axis_aligned is True

try:
    ConvexPolytope.with_obb(planes, verts, 5)
    raise AssertionError("expected TypeError for a non-Cuboid obb")
except TypeError:
    pass
"#);
}

#[test]
fn pointcloud_construction_and_far_collision() {
    run(r#"
from geomanpy import Pointcloud, Sphere, Vec3

pc = Pointcloud.from_list([(100.0, 0.0, 0.0), (101.0, 0.0, 0.0)], 0.1)
assert pc.collides(Sphere(Vec3(100.5, 0.0, 0.0), 0.45)) is True
assert pc.collides(Sphere(Vec3(100.0, 5.0, 0.0), 0.45)) is False

pc_r = Pointcloud.from_list([(100.0, 0.0, 0.0)], 0.1, (0.0, 10.0))
assert pc_r.collides(Sphere(Vec3(100.5, 0.0, 0.0), 0.45)) is True

pn = Pointcloud.from_numpy([[100.0, 0.0, 0.0], [101.0, 0.0, 0.0]], 0.1)
assert pn.collides(Sphere(Vec3(100.5, 0.0, 0.0), 0.45)) is True
assert pn.collides(Sphere(Vec3(100.0, 5.0, 0.0), 0.45)) is False
try:
    Pointcloud.from_numpy([1.0, 2.0])
    raise AssertionError("expected ValueError for non-(N,3) input")
except ValueError:
    pass

direct = Pointcloud([(0.0, 0.0, 0.0)], point_radius=0.1)
assert direct.collides(Sphere(Vec3(0.0, 0.0, 0.0), 0.05)) is True
"#);
}

#[test]
fn euler_rot_equality_and_hash() {
    run(r#"
from geomanpy import EulerRot, Quat

assert EulerRot.ZYX == EulerRot.ZYX
assert not (EulerRot.ZYX == EulerRot.XYZ)
assert EulerRot.ZYX != EulerRot.XYZ
assert EulerRot.ZYX == 0
assert 0 == EulerRot.ZYX
assert EulerRot.XYZ == 4
assert EulerRot.ZYX != 4
assert not (EulerRot.ZYX == "ZYX")

s = {EulerRot.ZYX, EulerRot.XYZ, EulerRot.ZYX}
assert len(s) == 2
assert EulerRot.XYZ in s

q = Quat.from_euler(EulerRot.ZYX, 0.3, 0.2, 0.1)
a, b, c = q.to_euler(EulerRot.ZYX)
assert abs(a - 0.3) < 1e-6 and abs(b - 0.2) < 1e-6 and abs(c - 0.1) < 1e-6
"#);
}

#[test]
fn polyline_spline_points_keyword() {
    run(r#"
from geomanpy import Polyline, Spline, Vec3

pl = Polyline(points=[Vec3(0.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0), Vec3(1.0, 1.0, 0.0)])
assert abs(pl.length() - 2.0) < 1e-5
assert len(pl.points) == 3
assert len(Polyline().points) == 0
assert len(Polyline(None).points) == 0

sp = Spline(points=[Vec3(0.0, 0.0, 0.0), Vec3(1.0, 1.0, 0.0), Vec3(2.0, 0.0, 0.0)])
assert len(sp.points) == 3
assert abs(sp.point(0.5).y - 1.0) < 1e-5

try:
    Polyline([Vec3(0.0, 0.0, 0.0)], points=[Vec3(0.0, 0.0, 0.0)])
    raise AssertionError("expected TypeError for duplicate points argument")
except TypeError:
    pass
try:
    Spline(bogus=[])
    raise AssertionError("expected TypeError for unknown keyword")
except TypeError:
    pass
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
assert i.abs_diff_eq(Interval(0.0, 2.0), 1e-6)
assert not i.abs_diff_eq(Interval(0.0, 3.0), 1e-6)

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
assert n.abs_diff_eq(n, 1e-6)

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

#[test]
fn vec_duck_typed_operands() {
    run(r#"
from geomanpy import Vec2, Vec3, Vec4

for cls, n in ((Vec2, 2), (Vec3, 3), (Vec4, 4)):
    a = [1.5, -2.0, 3.25, 0.5][:n]
    b = [4.0, 0.5, -1.0, 2.0][:n]
    v = cls(*a)
    w = cls(*b)
    foreign = type(cls.__name__, (), dict(zip("xyzw", b)))()

    assert v == list(a)
    assert v == tuple(a)
    assert not (v == list(a) + [0.0])
    assert not (v == list(a)[:-1])
    assert not (v == list(b))
    assert v != list(b)
    assert not (v == "x")

    summed = cls(*[x + y for x, y in zip(a, b)])
    assert v + tuple(b) == summed
    assert v + list(b) == summed
    assert list(b) + v == summed
    assert v + foreign == summed
    assert v - tuple(b) == cls(*[x - y for x, y in zip(a, b)])
    assert v * list(b) == cls(*[x * y for x, y in zip(a, b)])
    assert v == type(cls.__name__, (), dict(zip("xyzw", a)))()
    try:
        v + tuple(list(b) + [1.0])
        raise AssertionError("expected TypeError for oversized sequence operand")
    except TypeError:
        pass
    try:
        v + list(b)[:-1]
        raise AssertionError("expected TypeError for undersized sequence operand")
    except TypeError:
        pass

    dotted = sum(x * y for x, y in zip(a, b))
    assert abs(v.dot(list(b)) - dotted) < 1e-9
    assert abs(v.dot(tuple(b)) - dotted) < 1e-9
    assert abs(v.dot(foreign) - dotted) < 1e-9
    assert v.min(tuple(b)) == cls(*[min(x, y) for x, y in zip(a, b)])
    assert v.max(list(b)) == cls(*[max(x, y) for x, y in zip(a, b)])
    assert v.lerp(foreign, 0.5) == cls(*[(x + y) / 2.0 for x, y in zip(a, b)])
    assert v.lerp(list(b), 1.0) == w

    sentinel = object()
    radd = type("Radd", (), {"__radd__": lambda self, other: sentinel})()
    assert (v + radd) is sentinel
"#);
}

#[test]
fn quat_duck_typed_operands() {
    run(r#"
from geomanpy import Quat, Vec3

q = Quat.from_rotation_z(0.0)
assert q == [0.0, 0.0, 0.0, 1.0]
assert q == (0.0, 0.0, 0.0, 1.0)
assert q == type("Quat", (), {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0})()
assert not (q == [0.0, 0.0, 0.0])
assert not (q == "x")

assert q + [0.0, 0.0, 0.0, 1.0] == Quat(0.0, 0.0, 0.0, 2.0)
assert q - (0.0, 0.0, 0.0, 1.0) == Quat(0.0, 0.0, 0.0, 0.0)
assert q * [0.0, 0.0, 0.0, 1.0] == q

rotated = q * [1.0, 2.0, 3.0]
assert isinstance(rotated, Vec3)
assert rotated == Vec3(1.0, 2.0, 3.0)

try:
    q + [0.0, 0.0, 1.0]
    raise AssertionError("expected TypeError for undersized sequence operand")
except TypeError:
    pass
"#);
}

#[test]
fn dataclass_protocol_surface() {
    run(r#"
from geomanpy import Vec3, Quat, Sphere, Capsule, Interval, Collider

f = Sphere.__dataclass_fields__
assert isinstance(f, dict)
assert sorted(f.keys()) == ["center", "radius"]

s = Sphere(Vec3(1.0, 2.0, 3.0), 0.5)
assert type(s).__dataclass_fields__ is f
assert sorted(s.__dataclass_fields__.keys()) == ["center", "radius"]

assert Vec3(1.0, 2.0, 3.0).__dict__ == {"x": 1.0, "y": 2.0, "z": 3.0}

assert Collider.__dataclass_fields__ == {}
assert Collider().__dict__ == {}

for obj in (
    s,
    Capsule(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 2.0), 0.5),
    Vec3(1.0, 2.0, 3.0),
    Quat(0.1, 0.2, 0.3, 0.4),
    Interval(0.0, 2.0),
):
    d = obj.__dict__
    assert sorted(d.keys()) == sorted(type(obj).__dataclass_fields__.keys())
    for k in d:
        assert d[k] == getattr(obj, k)

try:
    import dataclasses
except ImportError:
    dataclasses = None
if dataclasses is not None:
    assert dataclasses.is_dataclass(Sphere)
    assert sorted(fld.name for fld in dataclasses.fields(Sphere)) == ["center", "radius"]
"#);
}
