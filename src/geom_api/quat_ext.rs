use crate::glam_wrappers::PyDQuat;
use glam::{DMat3, DQuat};
use numpy::{PyArray1, PyArray2, PyReadonlyArray2};
use pyo3::exceptions::PyZeroDivisionError;
use pyo3::prelude::*;

#[pymethods]
impl PyDQuat {
    /// Create from a 3x3 rotation matrix (numpy array, row-major).
    #[classmethod]
    fn from_matrix(
        _cls: &Bound<'_, pyo3::types::PyType>,
        matrix: PyReadonlyArray2<'_, f64>,
    ) -> PyResult<Self> {
        let view = matrix.as_array();
        if view.shape() != [3, 3] {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "expected 3x3 matrix",
            ));
        }
        let cols = [
            [view[(0, 0)], view[(1, 0)], view[(2, 0)]],
            [view[(0, 1)], view[(1, 1)], view[(2, 1)]],
            [view[(0, 2)], view[(1, 2)], view[(2, 2)]],
        ];
        let mat = DMat3::from_cols_array_2d(&cols);
        Ok(Self(DQuat::from_mat3(&mat)))
    }

    /// Returns (w, x, y, z) tuple.
    fn wxyz(&self) -> (f64, f64, f64, f64) {
        (self.0.w, self.0.x, self.0.y, self.0.z)
    }

    /// Returns (x, y, z, w) tuple.
    fn xyzw(&self) -> (f64, f64, f64, f64) {
        (self.0.x, self.0.y, self.0.z, self.0.w)
    }

    /// Returns quaternion as a 4-element numpy array [x, y, z, w].
    fn as_array<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, vec![self.0.x, self.0.y, self.0.z, self.0.w])
    }

    /// Returns 3x3 rotation matrix as numpy array (row-major).
    fn as_matrix<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        let mat = DMat3::from_quat(self.0);
        let c = mat.to_cols_array_2d();
        let rows = vec![
            vec![c[0][0], c[1][0], c[2][0]],
            vec![c[0][1], c[1][1], c[2][1]],
            vec![c[0][2], c[1][2], c[2][2]],
        ];
        PyArray2::from_vec2(py, &rows).unwrap()
    }

    /// Inverse for non-unit quaternions. Raises ZeroDivisionError if zero.
    fn inv(&self) -> PyResult<Self> {
        let norm_sq =
            self.0.x * self.0.x + self.0.y * self.0.y + self.0.z * self.0.z + self.0.w * self.0.w;
        if norm_sq == 0.0 {
            return Err(PyZeroDivisionError::new_err(
                "Cannot invert zero quaternion",
            ));
        }
        let conj = self.0.conjugate();
        let s = 1.0 / norm_sq;
        Ok(Self(DQuat::from_xyzw(
            conj.x * s,
            conj.y * s,
            conj.z * s,
            conj.w * s,
        )))
    }

    /// Magnitude of the quaternion.
    fn norm(&self) -> f64 {
        (self.0.x * self.0.x + self.0.y * self.0.y + self.0.z * self.0.z + self.0.w * self.0.w)
            .sqrt()
    }
}
