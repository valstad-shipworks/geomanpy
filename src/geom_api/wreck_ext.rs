use crate::glam_wrappers::{PyDAffine3, PyDVec3};
use crate::wreck_wrappers::{
    PyCapsule, PyCollider, PyConvexPolygon, PyConvexPolytope, PyCuboid, PyCylinder, PyLine,
    PyLineSegment, PyPlane, PyPointcloud, PyRay, PySphere,
};
use pyo3::prelude::*;
use wreck::{Scalable, Transformable};

/// `transform()` / `scale()` aliases for types that expose `transformed()` /
/// `scaled()`. Kept out of the core wreck wrapper definitions (which only
/// emit the `_ed` naming) so the shorter names remain part of the geom-api
/// surface while living alongside the other ergonomics helpers. Both
/// delegate directly to the underlying wreck trait methods — identical
/// behavior to `transformed` / `scaled`.
macro_rules! impl_transform_scale_alias {
    ($ty:ty) => {
        #[pymethods]
        impl $ty {
            #[inline]
            fn transform(&self, tf: PyDAffine3) -> Self {
                Self(self.0.transformed_d(tf.0))
            }
            #[inline]
            fn scale(&self, factor: f64) -> Self {
                Self(self.0.scaled_d(factor))
            }
        }
    };
}

impl_transform_scale_alias!(PySphere);
impl_transform_scale_alias!(PyCapsule);
impl_transform_scale_alias!(PyCuboid);
impl_transform_scale_alias!(PyCylinder);
impl_transform_scale_alias!(PyConvexPolytope);
impl_transform_scale_alias!(PyConvexPolygon);
impl_transform_scale_alias!(PyLine);
impl_transform_scale_alias!(PyRay);
impl_transform_scale_alias!(PyLineSegment);
impl_transform_scale_alias!(PyPlane);
impl_transform_scale_alias!(PyPointcloud);
impl_transform_scale_alias!(PyCollider);

#[pymethods]
impl PyCapsule {
    fn bounding_sphere(&self) -> (PyDVec3, f64) {
        let (c, r) = self.0.bounding_sphere();
        (
            PyDVec3(glam::DVec3::new(c.x as f64, c.y as f64, c.z as f64)),
            r as f64,
        )
    }
}

#[pymethods]
impl PyCylinder {
    fn bounding_sphere(&self) -> (PyDVec3, f64) {
        let (c, r) = self.0.bounding_sphere();
        (
            PyDVec3(glam::DVec3::new(c.x as f64, c.y as f64, c.z as f64)),
            r as f64,
        )
    }
}

#[pymethods]
impl PyLineSegment {
    fn bounding_sphere(&self) -> (PyDVec3, f64) {
        let (c, r) = self.0.bounding_sphere();
        (
            PyDVec3(glam::DVec3::new(c.x as f64, c.y as f64, c.z as f64)),
            r as f64,
        )
    }
}

#[pymethods]
impl PyCollider {
    fn stretch(&self, translation: PyDVec3) -> PyResult<Self> {
        self.0.try_stretch_d(translation.0)
            .map(|c| PyCollider(c.into()))
            .ok_or(pyo3::exceptions::PyRuntimeError::new_err("Stretch operation failed"))
    }
}
