use crate::glam_wrappers::{PyDMat3, PyDQuat};
use glam::{DMat3, DQuat, DVec3};
use numpy::{PyArray1, PyArray2, PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;

#[pymethods]
impl PyDMat3 {
    /// Create from a 3x3 numpy array (row-major). Alias for geom.py Rotation3d.from_matrix.
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
        Ok(Self(DMat3::from_cols_array_2d(&cols)))
    }

    /// Create from a 4x4 numpy array (uses upper-left 3x3).
    #[classmethod]
    fn from_matrix_four_by_four(
        _cls: &Bound<'_, pyo3::types::PyType>,
        matrix: PyReadonlyArray2<'_, f64>,
    ) -> PyResult<Self> {
        let view = matrix.as_array();
        if view.shape() != [4, 4] {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "expected 4x4 matrix",
            ));
        }
        let cols = [
            [view[(0, 0)], view[(1, 0)], view[(2, 0)]],
            [view[(0, 1)], view[(1, 1)], view[(2, 1)]],
            [view[(0, 2)], view[(1, 2)], view[(2, 2)]],
        ];
        Ok(Self(DMat3::from_cols_array_2d(&cols)))
    }

    /// Create from a Quaternion or (x,y,z,w) tuple.
    #[classmethod]
    fn from_quaternion(
        _cls: &Bound<'_, pyo3::types::PyType>,
        quat: &Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        if let Ok(q) = quat.extract::<PyDQuat>() {
            return Ok(Self(DMat3::from_quat(q.0)));
        }
        if let Ok((x, y, z, w)) = quat.extract::<(f64, f64, f64, f64)>() {
            return Ok(Self(DMat3::from_quat(
                DQuat::from_xyzw(x, y, z, w).normalize(),
            )));
        }
        Err(pyo3::exceptions::PyTypeError::new_err(
            "expected Quat or (x,y,z,w) tuple",
        ))
    }

    /// Create from a rotation vector (direction=axis, magnitude=angle in radians).
    ///
    /// Accepts a numpy ndarray, list, tuple, or any object implementing
    /// :class:`PyDVec3`'s extraction protocol.
    #[classmethod]
    fn from_vector(
        _cls: &Bound<'_, pyo3::types::PyType>,
        vec: &Bound<'_, pyo3::PyAny>,
    ) -> PyResult<Self> {
        let v: DVec3 = if let Ok(arr) = vec.extract::<PyReadonlyArray1<'_, f64>>() {
            let view = arr.as_array();
            if view.len() != 3 {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "expected 3-element array",
                ));
            }
            DVec3::new(view[0], view[1], view[2])
        } else {
            // Fallback: list / tuple / Vec3-like object.
            let pv: crate::glam_wrappers::PyDVec3 = vec.extract().map_err(|_| {
                pyo3::exceptions::PyTypeError::new_err(
                    "expected a 3-element ndarray, list, tuple, or Vec3",
                )
            })?;
            pv.0
        };
        let angle = v.length();
        if angle < 1e-12 {
            return Ok(Self(DMat3::IDENTITY));
        }
        let axis = v / angle;
        Ok(Self(DMat3::from_quat(DQuat::from_axis_angle(axis, angle))))
    }

    /// Identity rotation.
    #[classmethod]
    fn default(_cls: &Bound<'_, pyo3::types::PyType>) -> Self {
        Self(DMat3::IDENTITY)
    }

    /// Returns as 3x3 numpy array (row-major).
    fn as_matrix<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        let c = self.0.to_cols_array_2d();
        let rows = vec![
            vec![c[0][0], c[1][0], c[2][0]],
            vec![c[0][1], c[1][1], c[2][1]],
            vec![c[0][2], c[1][2], c[2][2]],
        ];
        PyArray2::from_vec2(py, &rows).unwrap()
    }

    /// Returns as 4x4 numpy array with rotation in upper-left 3x3.
    fn as_matrix_four_by_four<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        let c = self.0.to_cols_array_2d();
        let rows = vec![
            vec![c[0][0], c[1][0], c[2][0], 0.0],
            vec![c[0][1], c[1][1], c[2][1], 0.0],
            vec![c[0][2], c[1][2], c[2][2], 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];
        PyArray2::from_vec2(py, &rows).unwrap()
    }

    /// Convert to quaternion.
    fn as_quaternion(&self) -> PyDQuat {
        PyDQuat(DQuat::from_mat3(&self.0))
    }

    /// Returns (roll, pitch, yaw) extrinsic XYZ Euler angles.
    #[pyo3(signature = (*, degrees = false))]
    fn as_euler(&self, degrees: bool) -> (f64, f64, f64) {
        let q = DQuat::from_mat3(&self.0).normalize();
        let (x, y, z, w) = (q.x, q.y, q.z, q.w);
        let sinr_cosp = 2.0 * (w * x + y * z);
        let cosr_cosp = 1.0 - 2.0 * (x * x + y * y);
        let roll = sinr_cosp.atan2(cosr_cosp);
        let pitch_ratio = 2.0 * (w * y - z * x);
        let pitch = if pitch_ratio.abs() >= 1.0 {
            std::f64::consts::FRAC_PI_2.copysign(pitch_ratio)
        } else {
            pitch_ratio.asin()
        };
        let siny_cosp = 2.0 * (w * z + x * y);
        let cosy_cosp = 1.0 - 2.0 * (y * y + z * z);
        let yaw = siny_cosp.atan2(cosy_cosp);
        if degrees {
            (roll.to_degrees(), pitch.to_degrees(), yaw.to_degrees())
        } else {
            (roll, pitch, yaw)
        }
    }

    /// Returns (axis_numpy_array, angle_float).
    fn as_axis_angle<'py>(&self, py: Python<'py>) -> (Bound<'py, PyArray1<f64>>, f64) {
        let q = DQuat::from_mat3(&self.0).normalize();
        let w = q.w.clamp(-1.0, 1.0);
        let angle = 2.0 * w.acos();
        let s = (1.0 - w * w).max(0.0).sqrt();
        if s < 1e-9 {
            (PyArray1::from_vec(py, vec![1.0, 0.0, 0.0]), 0.0)
        } else {
            (
                PyArray1::from_vec(py, vec![q.x / s, q.y / s, q.z / s]),
                angle,
            )
        }
    }

    /// Geodesic angle between two rotations in radians.
    fn geodesic_angle(&self, other: &Self) -> f64 {
        let a = DQuat::from_mat3(&self.0).normalize();
        let b = DQuat::from_mat3(&other.0).normalize();
        let cos_half = a.dot(b).abs().clamp(-1.0, 1.0);
        2.0 * cos_half.acos()
    }

    /// Geodesic angle ignoring roll component.
    fn geodesic_angle_no_roll(&self, other: &Self) -> f64 {
        let qa = DQuat::from_mat3(&self.0).normalize();
        let qb = DQuat::from_mat3(&other.0).normalize();
        let relative = (qa.conjugate() * qb).normalize();
        let rel_mat = DMat3::from_quat(relative);
        let rel_cols = rel_mat.to_cols_array_2d();
        let roll_angle = rel_cols[1][2].atan2(rel_cols[2][2]);
        let neg_roll = DQuat::from_axis_angle(DVec3::X, -roll_angle);
        let no_roll = relative * neg_roll;
        let cos_half = no_roll.normalize().w.abs().clamp(-1.0, 1.0);
        2.0 * cos_half.acos()
    }

    /// Spherical interpolation between rotations.
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        let t = t.clamp(0.0, 1.0);
        let qa = DQuat::from_mat3(&self.0).normalize();
        let qb = DQuat::from_mat3(&other.0).normalize();
        let neg_a = qa.conjugate().normalize();
        let delta = (neg_a * qb).normalize();
        let w = delta.w.clamp(-1.0, 1.0);
        let angle = 2.0 * w.acos();
        let mut axis = DVec3::new(delta.x, delta.y, delta.z);
        if delta.w < 0.0 {
            axis = -axis;
        }
        let n = axis.length();
        let scaled = if n < 1e-12 || angle == 0.0 || t == 0.0 {
            DQuat::IDENTITY
        } else {
            DQuat::from_axis_angle(axis / n, t * angle)
        };
        Self(DMat3::from_quat((qa * scaled).normalize()))
    }

    fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let arr = self.0.to_cols_array();
        let mut hasher = DefaultHasher::new();
        for v in arr {
            v.to_bits().hash(&mut hasher);
        }
        hasher.finish()
    }
}
