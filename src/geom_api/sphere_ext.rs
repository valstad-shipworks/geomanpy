use crate::glam_wrappers::PyDVec3;
use crate::wreck_wrappers::PySphere;
use pyo3::prelude::*;

#[pymethods]
impl PySphere {
    /// Check if a point is inside the sphere.
    fn contains(&self, point: &PyDVec3) -> bool {
        let c = self.0.center;
        let p = glam::Vec3::new(point.0.x as f32, point.0.y as f32, point.0.z as f32);
        c.distance(p) <= self.0.radius
    }
}
