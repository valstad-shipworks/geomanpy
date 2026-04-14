use crate::glam_wrappers::PyDVec3;
use crate::wreck_wrappers::{PyCapsule, PyCollider, PyCylinder, PyLineSegment};
use pyo3::prelude::*;

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
