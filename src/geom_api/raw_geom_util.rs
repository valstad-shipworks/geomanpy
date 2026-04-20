use crate::impl_dataclass_fields;
use numpy::{PyArray2, PyReadonlyArray2};
use pyo3::prelude::*;

#[pyclass(frozen, name = "RawGeomUtil")]
pub struct PyRawGeomUtil;

impl_dataclass_fields!(PyRawGeomUtil, []);

#[pymethods]
impl PyRawGeomUtil {
    /// Transform a batch of 3D points by a 4x4 transformation matrix.
    #[staticmethod]
    fn batch_transform_points<'py>(
        py: Python<'py>,
        tf: PyReadonlyArray2<'_, f64>,
        points: PyReadonlyArray2<'_, f64>,
    ) -> PyResult<Bound<'py, PyArray2<f64>>> {
        let tf_view = tf.as_array();
        let pts_view = points.as_array();
        if tf_view.shape() != [4, 4] {
            return Err(pyo3::exceptions::PyValueError::new_err("tf must be 4x4"));
        }
        if pts_view.shape()[1] != 3 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "points must be (N, 3)",
            ));
        }
        let n = pts_view.shape()[0];
        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            let x = pts_view[(i, 0)];
            let y = pts_view[(i, 1)];
            let z = pts_view[(i, 2)];
            let rx =
                tf_view[(0, 0)] * x + tf_view[(0, 1)] * y + tf_view[(0, 2)] * z + tf_view[(0, 3)];
            let ry =
                tf_view[(1, 0)] * x + tf_view[(1, 1)] * y + tf_view[(1, 2)] * z + tf_view[(1, 3)];
            let rz =
                tf_view[(2, 0)] * x + tf_view[(2, 1)] * y + tf_view[(2, 2)] * z + tf_view[(2, 3)];
            result.push(vec![rx, ry, rz]);
        }
        Ok(PyArray2::from_vec2(py, &result).unwrap())
    }
}
