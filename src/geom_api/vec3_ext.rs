use crate::glam_wrappers::{PyDMat3, PyDVec3};
use glam::DVec3;
use numpy::{PyArray1, PyArray2, PyReadonlyArray1};
use pyo3::prelude::*;

#[pymethods]
impl PyDVec3 {
    /// Create from a 3-element sequence (numpy array, list, or tuple).
    /// Geom.py Translation3d.from_matrix().
    #[classmethod]
    fn from_matrix(
        _cls: &Bound<'_, pyo3::types::PyType>,
        matrix: &Bound<'_, pyo3::PyAny>,
    ) -> PyResult<Self> {
        if let Ok(arr) = matrix.extract::<PyReadonlyArray1<'_, f64>>() {
            let view = arr.as_array();
            if view.len() != 3 {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "expected 3-element array",
                ));
            }
            return Ok(Self(DVec3::new(view[0], view[1], view[2])));
        }
        let xs: [f64; 3] = matrix.extract().map_err(|_| {
            pyo3::exceptions::PyTypeError::new_err(
                "expected a 3-element ndarray, list, or tuple",
            )
        })?;
        Ok(Self(DVec3::new(xs[0], xs[1], xs[2])))
    }

    /// Zero vector.
    #[classmethod]
    fn default(_cls: &Bound<'_, pyo3::types::PyType>) -> Self {
        Self(DVec3::ZERO)
    }

    /// Returns as numpy array [x, y, z].
    fn as_array<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, vec![self.0.x, self.0.y, self.0.z])
    }

    /// Returns 4x4 translation matrix as numpy array.
    fn as_matrix_four_by_four<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        let rows = vec![
            vec![1.0, 0.0, 0.0, self.0.x],
            vec![0.0, 1.0, 0.0, self.0.y],
            vec![0.0, 0.0, 1.0, self.0.z],
            vec![0.0, 0.0, 0.0, 1.0],
        ];
        PyArray2::from_vec2(py, &rows).unwrap()
    }

    fn as_tuple(&self) -> (f64, f64, f64) {
        (self.0.x, self.0.y, self.0.z)
    }

    fn norm(&self) -> f64 {
        self.0.length()
    }

    fn squared_norm(&self) -> f64 {
        self.0.length_squared()
    }

    /// Linear interpolation.
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        Self(self.0.lerp(other.0, t))
    }

    /// Rotate this vector by a rotation matrix.
    fn rotate_by(&self, rotation: &PyDMat3) -> Self {
        Self(rotation.0 * self.0)
    }

    /// Returns the rotation (Mat3) from ref_axis to this vector's direction.
    /// This is an alias for `to_rotation` matching geom.py's Translation3d.angle().
    #[pyo3(signature = (ref_axis = None))]
    fn angle(&self, ref_axis: Option<[f64; 3]>) -> PyResult<PyDMat3> {
        let v = self.0;
        let r = ref_axis
            .map(|a| DVec3::new(a[0], a[1], a[2]))
            .unwrap_or(DVec3::X);

        let v_len = v.length();
        let r_len = r.length();
        if v_len < 1e-10 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Rotation undefined for zero translation vector.",
            ));
        }
        if r_len < 1e-10 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "ref_axis must be non-zero",
            ));
        }
        let v_hat = v / v_len;
        let r_hat = r / r_len;
        let cross = r_hat.cross(v_hat);
        let dot = r_hat.dot(v_hat);
        let cross_len = cross.length();

        if cross_len < 1e-10 {
            if dot > 0.0 {
                Ok(PyDMat3(glam::DMat3::IDENTITY))
            } else {
                let perp = if r_hat.x.abs() < 0.9 {
                    r_hat.cross(DVec3::X).normalize()
                } else {
                    r_hat.cross(DVec3::Y).normalize()
                };
                Ok(PyDMat3(glam::DMat3::from_quat(
                    glam::DQuat::from_axis_angle(perp, std::f64::consts::PI),
                )))
            }
        } else {
            let axis = cross / cross_len;
            let angle = f64::atan2(cross_len, dot);
            Ok(PyDMat3(glam::DMat3::from_quat(
                glam::DQuat::from_axis_angle(axis, angle),
            )))
        }
    }

    fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        fn stable_hash(f: f64) -> i64 {
            if f == 0.0 {
                0
            } else {
                f64::ceil(f * 100_000.0) as i64
            }
        }
        let mut hasher = DefaultHasher::new();
        (
            stable_hash(self.0.x),
            stable_hash(self.0.y),
            stable_hash(self.0.z),
        )
            .hash(&mut hasher);
        hasher.finish()
    }
}
