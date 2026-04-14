use crate::wreck_wrappers::PyPointcloud;
use numpy::PyReadonlyArray2;
use pyo3::prelude::*;

#[pymethods]
impl PyPointcloud {
    /// Create a Pointcloud from an Nx3 numpy array of f64 points.
    #[classmethod]
    #[pyo3(signature = (points, point_radius = 0.033))]
    fn from_numpy(
        _cls: &Bound<'_, pyo3::types::PyType>,
        points: PyReadonlyArray2<'_, f64>,
        point_radius: f32,
    ) -> PyResult<Self> {
        let view = points.as_array();
        if view.shape()[1] != 3 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "points must be (N, 3)",
            ));
        }
        let n = view.shape()[0];
        let mut pts = Vec::with_capacity(n);
        let mut min_r = f32::MAX;
        let mut max_r = 0.0f32;
        for i in 0..n {
            let x = view[(i, 0)] as f32;
            let y = view[(i, 1)] as f32;
            let z = view[(i, 2)] as f32;
            let r = (x * x + y * y + z * z).sqrt();
            if r < min_r {
                min_r = r;
            }
            if r > max_r {
                max_r = r;
            }
            pts.push([x, y, z]);
        }
        if pts.is_empty() {
            min_r = 0.0;
            max_r = 0.0;
        }
        Ok(Self(wreck::Pointcloud::new(
            &pts,
            (min_r, max_r + point_radius),
            point_radius,
        )))
    }
}
