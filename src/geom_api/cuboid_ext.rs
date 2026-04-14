use crate::glam_wrappers::{PyDMat3, PyDVec3};
use crate::wreck_wrappers::PyCuboid;
use pyo3::prelude::*;

#[pymethods]
impl PyCuboid {
    /// Half-size as a tuple of f64.
    #[getter]
    fn half_size(&self) -> (f64, f64, f64) {
        let he = self.0.half_extents;
        (he[0] as f64, he[1] as f64, he[2] as f64)
    }

    /// Center and orientation as a (Vec3, Mat3) tuple.
    #[getter]
    fn pose(&self) -> (PyDVec3, PyDMat3) {
        let c = self.0.center;
        let axes = self.0.axes;
        (
            PyDVec3(glam::DVec3::new(c.x as f64, c.y as f64, c.z as f64)),
            PyDMat3(glam::DMat3::from_cols(
                glam::DVec3::new(axes[0].x as f64, axes[0].y as f64, axes[0].z as f64),
                glam::DVec3::new(axes[1].x as f64, axes[1].y as f64, axes[1].z as f64),
                glam::DVec3::new(axes[2].x as f64, axes[2].y as f64, axes[2].z as f64),
            )),
        )
    }

    /// Create from an AlignedBox3d.
    #[classmethod]
    fn from_aligned(
        _cls: &Bound<'_, pyo3::types::PyType>,
        aligned: &crate::geom_api::aligned_box3d::PyAlignedBox3d,
    ) -> Self {
        let min = aligned.min;
        let max = aligned.max;
        let center = (min + max) / 2.0;
        let he = (max - min) / 2.0;
        Self(wreck::Cuboid::new(
            glam::Vec3::new(center.x as f32, center.y as f32, center.z as f32),
            [glam::Vec3::X, glam::Vec3::Y, glam::Vec3::Z],
            [he.x.abs() as f32, he.y.abs() as f32, he.z.abs() as f32],
        ))
    }
}
