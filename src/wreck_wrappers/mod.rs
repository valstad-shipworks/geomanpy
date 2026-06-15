//! Wreck wrapper types — `Sphere`, `Capsule`, `Cuboid`, … shared across
//! pyo3 and rustpython backends via `cfg_attr`.
//!
//! Each shape lives in its own module with both backends' method impls. The
//! cross-cutting machinery (the `Shape` tag enum, the uniform trait-forwarding
//! macros, polymorphic dispatch helpers, and registration) lives here.

use wreck::{
    Capsule, Collider, ConvexPolygon, ConvexPolytope, Cuboid, Cylinder, Line, LineSegment, Plane,
    Pointcloud, Ray, Sphere, soa::SpheresSoA,
};

pub(crate) mod capsule;
pub(crate) mod collider;
pub(crate) mod convex_polygon;
pub(crate) mod convex_polytope;
pub(crate) mod cuboid;
pub(crate) mod cylinder;
pub(crate) mod line;
pub(crate) mod line_segment;
pub(crate) mod plane;
pub(crate) mod pointcloud;
pub(crate) mod ray;
pub(crate) mod sphere;
pub(crate) mod sphere_collection;

pub use capsule::PyCapsule;
pub use collider::PyCollider;
pub use convex_polygon::PyConvexPolygon;
pub use convex_polytope::PyConvexPolytope;
pub use cuboid::PyCuboid;
pub use cylinder::PyCylinder;
pub use line::PyLine;
pub use line_segment::PyLineSegment;
pub use plane::PyPlane;
pub use pointcloud::PyPointcloud;
pub use ray::PyRay;
pub use sphere::PySphere;
pub use sphere_collection::PySphereCollection;

// PyShape: pyo3 enum exposed as a tagged class. RustPython doesn't have a
// direct equivalent (no pyclass-on-enum macro), so we only define the enum
// under pyo3 — rustpython users construct concrete shapes directly.
#[cfg(feature = "pyo3-backend")]
#[pyo3::pyclass(frozen, skip_from_py_object, name = "Shape")]
#[derive(Debug, Clone)]
pub enum PyShape {
    Sphere(PySphere),
    Capsule(PyCapsule),
    Cuboid(PyCuboid),
    Cylinder(PyCylinder),
    ConvexPolytope(PyConvexPolytope),
    ConvexPolygon(PyConvexPolygon),
    Line(PyLine),
    Ray(PyRay),
    LineSegment(PyLineSegment),
    Plane(PyPlane),
    Pointcloud(PyPointcloud),
}

macro_rules! impl_from_wreck {
    ($py:ty, $inner:ty) => {
        impl From<$inner> for $py {
            #[inline]
            fn from(v: $inner) -> Self {
                Self(v)
            }
        }
        impl From<$py> for $inner {
            #[inline]
            fn from(v: $py) -> Self {
                v.0
            }
        }
        impl std::ops::Deref for $py {
            type Target = $inner;
            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

impl_from_wreck!(PySphere, Sphere);
impl_from_wreck!(PyCapsule, Capsule);
impl_from_wreck!(PyCuboid, Cuboid);
impl_from_wreck!(PyCylinder, Cylinder);
impl_from_wreck!(PyConvexPolytope, ConvexPolytope);
impl_from_wreck!(PyConvexPolygon, ConvexPolygon);
impl_from_wreck!(PyLine, Line);
impl_from_wreck!(PyRay, Ray);
impl_from_wreck!(PyLineSegment, LineSegment);
impl_from_wreck!(PyPlane, Plane);
impl_from_wreck!(PyPointcloud, Pointcloud);
impl_from_wreck!(PySphereCollection, SpheresSoA);
impl_from_wreck!(PyCollider, Collider<Pointcloud>);

// =============================================================================
// pyo3-backend: conversion helpers, FromPyObject dispatch, trait-forwarding
// macros, and registration.
// =============================================================================

#[cfg(feature = "pyo3-backend")]
pub(crate) mod pyo3_glue {
    use super::*;
    use crate::glam_wrappers::PyDVec3;
    use glam::Vec3;
    use pyo3::PyResult;
    use pyo3::prelude::*;

    #[inline]
    pub(crate) fn dv3(v: PyDVec3) -> Vec3 {
        v.0.as_vec3()
    }

    #[inline]
    pub(crate) fn v3d(v: Vec3) -> PyDVec3 {
        PyDVec3(glam::DVec3::new(v.x as f64, v.y as f64, v.z as f64))
    }

    fn extract_f32_vec3(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> PyResult<Vec3> {
        if let Ok(xs) = ob.extract::<[f64; 3]>() {
            return Ok(Vec3::new(xs[0] as f32, xs[1] as f32, xs[2] as f32));
        }
        let py = ob.py();
        let x: f64 = ob.getattr(pyo3::intern!(py, "x"))?.extract()?;
        let y: f64 = ob.getattr(pyo3::intern!(py, "y"))?.extract()?;
        let z: f64 = ob.getattr(pyo3::intern!(py, "z"))?.extract()?;
        Ok(Vec3::new(x as f32, y as f32, z as f32))
    }

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PySphere {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(*v.get());
            }
            let py = ob.py();
            let center = extract_f32_vec3(&ob.getattr(pyo3::intern!(py, "center"))?)?;
            let radius: f32 = ob.getattr(pyo3::intern!(py, "radius"))?.extract()?;
            Ok(Self(Sphere::new(center, radius)))
        }
    }

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyCuboid {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(*v.get());
            }
            let py = ob.py();
            let center = extract_f32_vec3(&ob.getattr(pyo3::intern!(py, "center"))?)?;
            let he: (f32, f32, f32) = ob.getattr(pyo3::intern!(py, "half_extents"))?.extract()?;
            let axes: ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32)) =
                ob.getattr(pyo3::intern!(py, "axes"))?.extract()?;
            Ok(Self(Cuboid::new(
                center,
                [
                    Vec3::new(axes.0.0, axes.0.1, axes.0.2),
                    Vec3::new(axes.1.0, axes.1.1, axes.1.2),
                    Vec3::new(axes.2.0, axes.2.1, axes.2.2),
                ],
                [he.0, he.1, he.2],
            )))
        }
    }

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyCylinder {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(*v.get());
            }
            let py = ob.py();
            let p1 = extract_f32_vec3(&ob.getattr(pyo3::intern!(py, "p1"))?)?;
            let p2 = extract_f32_vec3(&ob.getattr(pyo3::intern!(py, "p2"))?)?;
            let radius: f32 = ob.getattr(pyo3::intern!(py, "radius"))?.extract()?;
            Ok(Self(Cylinder::new(p1, p2, radius)))
        }
    }

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyCapsule {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(*v.get());
            }
            let py = ob.py();
            let p1 = extract_f32_vec3(&ob.getattr(pyo3::intern!(py, "p1"))?)?;
            let p2 = extract_f32_vec3(&ob.getattr(pyo3::intern!(py, "p2"))?)?;
            let radius: f32 = ob.getattr(pyo3::intern!(py, "radius"))?.extract()?;
            Ok(Self(Capsule::new(p1, p2, radius)))
        }
    }

    pub(crate) fn push_shape_into(collider: &mut Collider<Pointcloud>, shape: PyShape) {
        match shape {
            PyShape::Sphere(s) => collider.add(s.0),
            PyShape::Capsule(c) => collider.add(c.0),
            PyShape::Cuboid(c) => collider.add(c.0),
            PyShape::Cylinder(c) => collider.add(c.0),
            PyShape::ConvexPolytope(p) => collider.add(p.0),
            PyShape::ConvexPolygon(p) => collider.add(p.0),
            PyShape::Line(l) => collider.add(l.0),
            PyShape::Ray(r) => collider.add(r.0),
            PyShape::LineSegment(s) => collider.add(s.0),
            PyShape::Plane(p) => collider.add(p.0),
            PyShape::Pointcloud(p) => collider.add(p.0),
        }
    }

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyCollider {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(v.borrow().clone());
            }
            if ob.is_none() {
                return Ok(Self(Collider::<Pointcloud>::default()));
            }
            let mut collider = Collider::<Pointcloud>::default();
            if let Ok(shape) = ob.extract::<PyShape>() {
                push_shape_into(&mut collider, shape);
                return Ok(Self(collider));
            }
            if let Ok(iter) = ob.try_iter() {
                let mut any = false;
                for item in iter {
                    let item = item?;
                    let shape: PyShape = item.extract().map_err(|_| {
                        pyo3::exceptions::PyTypeError::new_err(
                            "PyCollider: sequence item is not a Shape",
                        )
                    })?;
                    push_shape_into(&mut collider, shape);
                    any = true;
                }
                if any {
                    return Ok(Self(collider));
                }
            }
            let py = ob.py();
            if let Ok(spheres_obj) = ob.call_method0(pyo3::intern!(py, "spheres")) {
                if let Ok(soa) = spheres_obj.extract::<PySphereCollection>() {
                    let n = soa.0.len();
                    for i in 0..n {
                        collider.add(soa.0.get(i));
                    }
                } else if let Ok(vs) = spheres_obj.extract::<Vec<PySphere>>() {
                    for s in vs {
                        collider.add(s.0);
                    }
                } else if let Ok(len) = spheres_obj
                    .call_method0(pyo3::intern!(py, "__len__"))
                    .and_then(|v| v.extract::<usize>())
                {
                    for i in 0..len {
                        if let Ok(item) =
                            spheres_obj.call_method1(pyo3::intern!(py, "__getitem__"), (i,))
                            && let Ok(s) = item.extract::<PySphere>()
                        {
                            collider.add(s.0);
                        }
                    }
                }
            }
            let cuboids: Vec<PyCuboid> = ob
                .call_method0(pyo3::intern!(py, "cuboids"))
                .and_then(|v| v.extract())
                .unwrap_or_default();
            for b in cuboids {
                collider.add(b.0);
            }
            let cylinders: Vec<PyCylinder> = ob
                .call_method0(pyo3::intern!(py, "cylinders"))
                .and_then(|v| v.extract())
                .unwrap_or_default();
            for c in cylinders {
                collider.add(c.0);
            }
            let capsules: Vec<PyCapsule> = ob
                .call_method0(pyo3::intern!(py, "capsules"))
                .and_then(|v| v.extract())
                .unwrap_or_default();
            for c in capsules {
                collider.add(c.0);
            }
            Ok(Self(collider))
        }
    }

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyShape {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(v.get().clone());
            }
            if let Ok(v) = ob.cast_exact::<PySphere>() {
                return Ok(Self::Sphere(*v.get()));
            }
            if let Ok(v) = ob.cast_exact::<PyCapsule>() {
                return Ok(Self::Capsule(*v.get()));
            }
            if let Ok(v) = ob.cast_exact::<PyCuboid>() {
                return Ok(Self::Cuboid(*v.get()));
            }
            if let Ok(v) = ob.cast_exact::<PyCylinder>() {
                return Ok(Self::Cylinder(*v.get()));
            }
            if let Ok(v) = ob.cast_exact::<PyConvexPolytope>() {
                return Ok(Self::ConvexPolytope(v.get().clone()));
            }
            if let Ok(v) = ob.cast_exact::<PyConvexPolygon>() {
                return Ok(Self::ConvexPolygon(v.get().clone()));
            }
            if let Ok(v) = ob.cast_exact::<PyLine>() {
                return Ok(Self::Line(*v.get()));
            }
            if let Ok(v) = ob.cast_exact::<PyRay>() {
                return Ok(Self::Ray(*v.get()));
            }
            if let Ok(v) = ob.cast_exact::<PyLineSegment>() {
                return Ok(Self::LineSegment(*v.get()));
            }
            if let Ok(v) = ob.cast_exact::<PyPlane>() {
                return Ok(Self::Plane(*v.get()));
            }
            if let Ok(v) = ob.cast_exact::<PyPointcloud>() {
                return Ok(Self::Pointcloud(v.get().clone()));
            }
            if let Ok(v) = ob.extract::<PySphere>() {
                return Ok(Self::Sphere(v));
            }
            if let Ok(v) = ob.extract::<PyCapsule>() {
                return Ok(Self::Capsule(v));
            }
            if let Ok(v) = ob.extract::<PyCuboid>() {
                return Ok(Self::Cuboid(v));
            }
            if let Ok(v) = ob.extract::<PyCylinder>() {
                return Ok(Self::Cylinder(v));
            }
            if let Ok(v) = ob.extract::<PyConvexPolytope>() {
                return Ok(Self::ConvexPolytope(v));
            }
            if let Ok(v) = ob.extract::<PyConvexPolygon>() {
                return Ok(Self::ConvexPolygon(v));
            }
            if let Ok(v) = ob.extract::<PyLine>() {
                return Ok(Self::Line(v));
            }
            if let Ok(v) = ob.extract::<PyRay>() {
                return Ok(Self::Ray(v));
            }
            if let Ok(v) = ob.extract::<PyLineSegment>() {
                return Ok(Self::LineSegment(v));
            }
            if let Ok(v) = ob.extract::<PyPlane>() {
                return Ok(Self::Plane(v));
            }
            if let Ok(v) = ob.extract::<PyPointcloud>() {
                return Ok(Self::Pointcloud(v));
            }
            Err(pyo3::exceptions::PyTypeError::new_err(
                "expected a Shape (Sphere, Cuboid, Cylinder, etc.)",
            ))
        }
    }

    macro_rules! impl_transform_scale_py {
        ($ty:ty) => {
            #[pyo3::pymethods]
            impl $ty {
                fn scaled(&self, factor: f64) -> Self {
                    Self(wreck::Scalable::scaled_d(&self.0, factor))
                }
                fn translated(&self, offset: crate::glam_wrappers::PyDVec3) -> Self {
                    Self(wreck::Transformable::translated_d(&self.0, offset.0))
                }
                fn rotated_mat(&self, mat: crate::glam_wrappers::PyDMat3) -> Self {
                    Self(wreck::Transformable::rotated_mat_d(&self.0, mat.0))
                }
                fn rotated_quat(&self, quat: crate::glam_wrappers::PyDQuat) -> Self {
                    Self(wreck::Transformable::rotated_quat_d(&self.0, quat.0))
                }
                fn transformed(&self, mat: crate::glam_wrappers::PyDAffine3) -> Self {
                    Self(wreck::Transformable::transformed_d(&self.0, mat.0))
                }
            }
        };
    }

    macro_rules! impl_bounded_py {
        ($ty:ty) => {
            #[pyo3::pymethods]
            impl $ty {
                fn broadphase(&self) -> PySphere {
                    PySphere(wreck::Bounded::broadphase(&self.0))
                }
                fn obb(&self) -> PyCuboid {
                    PyCuboid(wreck::Bounded::obb(&self.0))
                }
                fn aabb(&self) -> PyCuboid {
                    PyCuboid(wreck::Bounded::aabb(&self.0))
                }
            }
        };
    }

    macro_rules! impl_collides_all {
        ($ty:ty) => {
            #[pyo3::pymethods]
            impl $ty {
                fn collides(&self, other: &PyShape) -> bool {
                    use wreck::Collides;
                    match other {
                        PyShape::Sphere(s) => self.0.collides(&s.0),
                        PyShape::Capsule(c) => self.0.collides(&c.0),
                        PyShape::Cuboid(c) => self.0.collides(&c.0),
                        PyShape::Cylinder(c) => self.0.collides(&c.0),
                        PyShape::ConvexPolytope(p) => self.0.collides(&p.0),
                        PyShape::ConvexPolygon(p) => self.0.collides(&p.0),
                        PyShape::Line(l) => self.0.collides(&l.0),
                        PyShape::Ray(r) => self.0.collides(&r.0),
                        PyShape::LineSegment(s) => self.0.collides(&s.0),
                        PyShape::Plane(p) => self.0.collides(&p.0),
                        PyShape::Pointcloud(p) => self.0.collides(&p.0),
                    }
                }
            }
        };
    }

    macro_rules! impl_collides_no_pcl_self {
        ($ty:ty) => {
            #[pyo3::pymethods]
            impl $ty {
                fn collides(&self, other: &PyShape) -> pyo3::PyResult<bool> {
                    use wreck::Collides;
                    match other {
                        PyShape::Sphere(s) => Ok(self.0.collides(&s.0)),
                        PyShape::Capsule(c) => Ok(self.0.collides(&c.0)),
                        PyShape::Cuboid(c) => Ok(self.0.collides(&c.0)),
                        PyShape::Cylinder(c) => Ok(self.0.collides(&c.0)),
                        PyShape::ConvexPolytope(p) => Ok(self.0.collides(&p.0)),
                        PyShape::ConvexPolygon(p) => Ok(self.0.collides(&p.0)),
                        PyShape::Line(l) => Ok(self.0.collides(&l.0)),
                        PyShape::Ray(r) => Ok(self.0.collides(&r.0)),
                        PyShape::LineSegment(s) => Ok(self.0.collides(&s.0)),
                        PyShape::Plane(p) => Ok(self.0.collides(&p.0)),
                        PyShape::Pointcloud(_) => Err(pyo3::exceptions::PyValueError::new_err(
                            "Pointcloud-Pointcloud collision is not supported",
                        )),
                    }
                }
            }
        };
    }

    macro_rules! impl_approx_py {
        ($ty:ty) => {
            #[pyo3::pymethods]
            impl $ty {
                #[inline]
                fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f64) -> bool {
                    approx::AbsDiffEq::abs_diff_eq(&self.0, &rhs.0, max_abs_diff as f32)
                }
            }
        };
    }

    pub(crate) use impl_approx_py;
    pub(crate) use impl_bounded_py;
    pub(crate) use impl_collides_all;
    pub(crate) use impl_collides_no_pcl_self;
    pub(crate) use impl_transform_scale_py;
}

#[cfg(feature = "pyo3-backend")]
pub(crate) use pyo3_glue::{
    impl_approx_py, impl_bounded_py, impl_collides_all, impl_collides_no_pcl_self,
    impl_transform_scale_py,
};

// Uniform trait-forwarding impls applied identically across the shape types.
#[cfg(feature = "pyo3-backend")]
mod pyo3_uniform {
    use super::*;
    use crate::{impl_dataclass_fields, impl_getnewargs_ex};

    impl_transform_scale_py!(PySphere);
    impl_transform_scale_py!(PyCapsule);
    impl_transform_scale_py!(PyCuboid);
    impl_transform_scale_py!(PyCylinder);
    impl_transform_scale_py!(PyConvexPolytope);
    impl_transform_scale_py!(PyConvexPolygon);
    impl_transform_scale_py!(PyLine);
    impl_transform_scale_py!(PyRay);
    impl_transform_scale_py!(PyLineSegment);
    impl_transform_scale_py!(PyPlane);
    impl_transform_scale_py!(PyPointcloud);
    impl_transform_scale_py!(PyCollider);

    impl_bounded_py!(PySphere);
    impl_bounded_py!(PyCapsule);
    impl_bounded_py!(PyCuboid);
    impl_bounded_py!(PyCylinder);
    impl_bounded_py!(PyConvexPolytope);
    impl_bounded_py!(PyConvexPolygon);
    impl_bounded_py!(PyLineSegment);
    impl_bounded_py!(PyPointcloud);
    impl_bounded_py!(PyCollider);

    impl_collides_all!(PySphere);
    impl_collides_all!(PyCapsule);
    impl_collides_all!(PyCuboid);
    impl_collides_all!(PyCylinder);
    impl_collides_all!(PyConvexPolytope);
    impl_collides_all!(PyConvexPolygon);
    impl_collides_all!(PyLine);
    impl_collides_all!(PyRay);
    impl_collides_all!(PyLineSegment);
    impl_collides_all!(PyPlane);
    impl_collides_no_pcl_self!(PyPointcloud);

    impl_approx_py!(PySphere);
    impl_approx_py!(PyCapsule);
    impl_approx_py!(PyCuboid);
    impl_approx_py!(PyCylinder);
    impl_approx_py!(PyPlane);
    impl_approx_py!(PyLine);
    impl_approx_py!(PyRay);
    impl_approx_py!(PyLineSegment);
    impl_approx_py!(PyConvexPolygon);
    impl_approx_py!(PyConvexPolytope);
    impl_approx_py!(PySphereCollection);
    impl_approx_py!(PyPointcloud);

    impl_getnewargs_ex!(PySphere);
    impl_getnewargs_ex!(PyCapsule);
    impl_getnewargs_ex!(PyCuboid);
    impl_getnewargs_ex!(PyCylinder);
    impl_getnewargs_ex!(PyConvexPolytope);
    impl_getnewargs_ex!(PyConvexPolygon);
    impl_getnewargs_ex!(PyLine);
    impl_getnewargs_ex!(PyRay);
    impl_getnewargs_ex!(PyLineSegment);
    impl_getnewargs_ex!(PyPlane);
    impl_getnewargs_ex!(PyPointcloud);
    impl_getnewargs_ex!(PySphereCollection);
    impl_getnewargs_ex!(PyCollider);

    impl_dataclass_fields!(PySphere, ["center", "radius"]);
    impl_dataclass_fields!(PyCapsule, ["p1", "p2", "radius"]);
    impl_dataclass_fields!(PyCuboid, ["center", "axes", "half_extents"]);
    impl_dataclass_fields!(PyCylinder, ["p1", "p2", "radius"]);
    impl_dataclass_fields!(PyConvexPolytope, ["planes", "vertices"]);
    impl_dataclass_fields!(
        PyConvexPolygon,
        ["center", "normal", "u_axis", "v_axis", "vertices_2d"]
    );
    impl_dataclass_fields!(PyLine, ["origin", "dir"]);
    impl_dataclass_fields!(PyRay, ["origin", "dir"]);
    impl_dataclass_fields!(PyLineSegment, ["p1", "p2"]);
    impl_dataclass_fields!(PyPlane, ["normal", "d"]);
    impl_dataclass_fields!(PyPointcloud, []);
    impl_dataclass_fields!(PySphereCollection, []);
    impl_dataclass_fields!(PyCollider, []);
    impl_dataclass_fields!(PyShape, []);
}

#[cfg(feature = "pyo3-backend")]
pub fn register(m: &pyo3::Bound<'_, pyo3::types::PyModule>) -> pyo3::PyResult<()> {
    use pyo3::prelude::*;
    m.add_class::<PySphere>()?;
    m.add_class::<PyCapsule>()?;
    m.add_class::<PyCuboid>()?;
    m.add_class::<PyCylinder>()?;
    m.add_class::<PyConvexPolytope>()?;
    m.add_class::<PyConvexPolygon>()?;
    m.add_class::<PyLine>()?;
    m.add_class::<PyRay>()?;
    m.add_class::<PyLineSegment>()?;
    m.add_class::<PyPlane>()?;
    m.add_class::<PyPointcloud>()?;
    m.add_class::<PySphereCollection>()?;
    m.add_class::<PyShape>()?;
    m.add_class::<PyCollider>()?;
    Ok(())
}

// =============================================================================
// rustpython-backend: conversion helpers and polymorphic shape dispatch.
// =============================================================================

#[cfg(feature = "rustpython-backend")]
pub(crate) mod rustpython_glue {
    use super::*;
    use crate::glam_wrappers::{PyDAffine3, PyDMat3, PyDVec3};
    use glam::Vec3;
    use rustpython_vm::{PyObjectRef, PyResult, VirtualMachine};

    #[inline]
    pub(crate) fn dv3(v: glam::DVec3) -> Vec3 {
        v.as_vec3()
    }

    #[inline]
    pub(crate) fn v3d(v: Vec3) -> PyDVec3 {
        PyDVec3(glam::DVec3::new(v.x as f64, v.y as f64, v.z as f64))
    }

    pub(crate) fn extract_mat3(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<glam::DMat3> {
        obj.downcast_ref::<PyDMat3>()
            .map(|m| m.0)
            .ok_or_else(|| vm.new_type_error("expected Mat3".to_owned()))
    }
    pub(crate) fn extract_affine3(
        obj: &PyObjectRef,
        vm: &VirtualMachine,
    ) -> PyResult<glam::DAffine3> {
        obj.downcast_ref::<PyDAffine3>()
            .map(|a| a.0)
            .ok_or_else(|| vm.new_type_error("expected Affine3".to_owned()))
    }

    /// Polymorphic shape dispatch — accepts any wrapper and pushes it into
    /// the collider. Used by `PyCollider.add` and `PyCollider.collides`.
    pub(crate) fn add_to_collider(
        c: &mut Collider<Pointcloud>,
        obj: &PyObjectRef,
        vm: &VirtualMachine,
    ) -> PyResult<()> {
        if let Some(v) = obj.downcast_ref::<PySphere>() {
            c.add(v.0);
            return Ok(());
        }
        if let Some(v) = obj.downcast_ref::<PyCapsule>() {
            c.add(v.0);
            return Ok(());
        }
        if let Some(v) = obj.downcast_ref::<PyCuboid>() {
            c.add(v.0);
            return Ok(());
        }
        if let Some(v) = obj.downcast_ref::<PyCylinder>() {
            c.add(v.0);
            return Ok(());
        }
        if let Some(v) = obj.downcast_ref::<PyConvexPolytope>() {
            c.add(v.0.clone());
            return Ok(());
        }
        if let Some(v) = obj.downcast_ref::<PyConvexPolygon>() {
            c.add(v.0.clone());
            return Ok(());
        }
        if let Some(v) = obj.downcast_ref::<PyLine>() {
            c.add(v.0);
            return Ok(());
        }
        if let Some(v) = obj.downcast_ref::<PyRay>() {
            c.add(v.0);
            return Ok(());
        }
        if let Some(v) = obj.downcast_ref::<PyLineSegment>() {
            c.add(v.0);
            return Ok(());
        }
        if let Some(v) = obj.downcast_ref::<PyPlane>() {
            c.add(v.0);
            return Ok(());
        }
        if let Some(v) = obj.downcast_ref::<PyPointcloud>() {
            c.add(v.0.clone());
            return Ok(());
        }
        Err(vm.new_type_error(
            "expected a Shape (Sphere/Capsule/Cuboid/Cylinder/ConvexPolytope/ConvexPolygon/Line/Ray/LineSegment/Plane/Pointcloud)".to_owned()
        ))
    }

    /// Polymorphic shape-vs-collider collision.
    pub(crate) fn shape_collides_collider(
        c: &Collider<Pointcloud>,
        obj: &PyObjectRef,
        vm: &VirtualMachine,
    ) -> PyResult<bool> {
        if let Some(v) = obj.downcast_ref::<PySphere>() {
            return Ok(c.collides(&v.0));
        }
        if let Some(v) = obj.downcast_ref::<PyCapsule>() {
            return Ok(c.collides(&v.0));
        }
        if let Some(v) = obj.downcast_ref::<PyCuboid>() {
            return Ok(c.collides(&v.0));
        }
        if let Some(v) = obj.downcast_ref::<PyCylinder>() {
            return Ok(c.collides(&v.0));
        }
        if let Some(v) = obj.downcast_ref::<PyConvexPolytope>() {
            return Ok(c.collides(&v.0));
        }
        if let Some(v) = obj.downcast_ref::<PyConvexPolygon>() {
            return Ok(c.collides(&v.0));
        }
        if let Some(v) = obj.downcast_ref::<PyLine>() {
            return Ok(c.collides(&v.0));
        }
        if let Some(v) = obj.downcast_ref::<PyRay>() {
            return Ok(c.collides(&v.0));
        }
        if let Some(v) = obj.downcast_ref::<PyLineSegment>() {
            return Ok(c.collides(&v.0));
        }
        if let Some(v) = obj.downcast_ref::<PyPlane>() {
            return Ok(c.collides(&v.0));
        }
        Err(vm.new_type_error("expected a Shape".to_owned()))
    }

    /// Dispatch `lhs.collides(other)` where `other` is any concrete shape
    /// wrapper. Mirrors the pyo3 `impl_collides_all!` match.
    pub(crate) fn shape_collides<S>(lhs: &S, obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<bool>
    where
        S: wreck::Collides<Sphere>
            + wreck::Collides<Capsule>
            + wreck::Collides<Cuboid>
            + wreck::Collides<Cylinder>
            + wreck::Collides<ConvexPolytope>
            + wreck::Collides<ConvexPolygon>
            + wreck::Collides<Line>
            + wreck::Collides<Ray>
            + wreck::Collides<LineSegment>
            + wreck::Collides<Plane>
            + wreck::Collides<Pointcloud>,
    {
        if let Some(v) = obj.downcast_ref::<PySphere>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyCapsule>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyCuboid>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyCylinder>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyConvexPolytope>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyConvexPolygon>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyLine>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyRay>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyLineSegment>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyPlane>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyPointcloud>() { return Ok(lhs.collides(&v.0)); }
        Err(vm.new_type_error("collides() expects a shape".to_owned()))
    }

    /// Like [`shape_collides`] but rejects a `Pointcloud` argument — used by
    /// `Pointcloud.collides` (pointcloud-vs-pointcloud is unsupported).
    pub(crate) fn shape_collides_no_pcl<S>(
        lhs: &S,
        obj: &PyObjectRef,
        vm: &VirtualMachine,
    ) -> PyResult<bool>
    where
        S: wreck::Collides<Sphere>
            + wreck::Collides<Capsule>
            + wreck::Collides<Cuboid>
            + wreck::Collides<Cylinder>
            + wreck::Collides<ConvexPolytope>
            + wreck::Collides<ConvexPolygon>
            + wreck::Collides<Line>
            + wreck::Collides<Ray>
            + wreck::Collides<LineSegment>
            + wreck::Collides<Plane>,
    {
        if let Some(v) = obj.downcast_ref::<PySphere>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyCapsule>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyCuboid>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyCylinder>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyConvexPolytope>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyConvexPolygon>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyLine>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyRay>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyLineSegment>() { return Ok(lhs.collides(&v.0)); }
        if let Some(v) = obj.downcast_ref::<PyPlane>() { return Ok(lhs.collides(&v.0)); }
        if obj.downcast_ref::<PyPointcloud>().is_some() {
            return Err(vm.new_value_error(
                "Pointcloud-Pointcloud collision is not supported".to_owned(),
            ));
        }
        Err(vm.new_type_error("collides() expects a shape".to_owned()))
    }
}
