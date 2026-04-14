use crate::glam_wrappers::{PyDAffine3, PyDMat3, PyDVec3};
use glam::{DAffine3, DMat3, DQuat, DVec3};
use numpy::PyArray2;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

fn scale_rotation_mat(m: DMat3, s: f64) -> DMat3 {
    let q = DQuat::from_mat3(&m).normalize();
    let w = q.w.clamp(-1.0, 1.0);
    let angle = 2.0 * w.acos();
    let mut axis = DVec3::new(q.x, q.y, q.z);
    if q.w < 0.0 {
        axis = -axis;
    }
    let n = axis.length();
    if n < 1e-12 || angle == 0.0 || s == 0.0 {
        return DMat3::IDENTITY;
    }
    DMat3::from_quat(DQuat::from_axis_angle(axis / n, s * angle))
}

fn invert_rotation(m: DMat3) -> DMat3 {
    m.transpose()
}

#[pymethods]
impl PyDAffine3 {
    /// Create from translation (Vec3) and rotation (Mat3).
    #[classmethod]
    fn from_components(
        _cls: &Bound<'_, pyo3::types::PyType>,
        p: PyDVec3,
        r: PyDMat3,
    ) -> Self {
        Self(DAffine3 {
            matrix3: r.0,
            translation: p.0,
        })
    }

    /// Create transform from one pose to another.
    #[classmethod]
    fn from_between(
        _cls: &Bound<'_, pyo3::types::PyType>,
        from_p: (PyDVec3, PyDMat3),
        to_p: (PyDVec3, PyDMat3),
    ) -> Self {
        let delta_t = to_p.0 .0 - from_p.0 .0;
        let inv_from_r = invert_rotation(from_p.1 .0);
        let t = inv_from_r * delta_t;
        let r = to_p.1 .0 * inv_from_r;
        Self(DAffine3 {
            matrix3: r,
            translation: t,
        })
    }

    #[classmethod]
    fn from_split(
        _cls: &Bound<'_, pyo3::types::PyType>,
        t: PyDVec3,
        r: PyDMat3,
    ) -> Self {
        Self(DAffine3 {
            matrix3: r.0,
            translation: t.0,
        })
    }

    /// Create from a single Translation3d or Rotation3d component.
    #[classmethod]
    fn just(
        _cls: &Bound<'_, pyo3::types::PyType>,
        component: &Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        if let Ok(t) = component.extract::<PyDVec3>() {
            return Ok(Self(DAffine3 {
                matrix3: DMat3::IDENTITY,
                translation: t.0,
            }));
        }
        if let Ok(r) = component.extract::<PyDMat3>() {
            return Ok(Self(DAffine3 {
                matrix3: r.0,
                translation: DVec3::ZERO,
            }));
        }
        Err(pyo3::exceptions::PyTypeError::new_err(
            "expected Vec3 or Mat3",
        ))
    }

    /// Identity transform.
    #[classmethod]
    fn default(_cls: &Bound<'_, pyo3::types::PyType>) -> Self {
        Self(DAffine3::IDENTITY)
    }

    /// Compose multiple transforms.
    #[staticmethod]
    #[pyo3(signature = (*transforms))]
    fn chain(transforms: Vec<Bound<'_, PyAny>>) -> PyResult<Self> {
        let mut pose = DAffine3::IDENTITY;
        for item in &transforms {
            let tf = if let Ok(a) = item.extract::<PyDAffine3>() {
                a.0
            } else if let Ok(t) = item.extract::<PyDVec3>() {
                DAffine3 {
                    matrix3: DMat3::IDENTITY,
                    translation: t.0,
                }
            } else if let Ok(r) = item.extract::<PyDMat3>() {
                DAffine3 {
                    matrix3: r.0,
                    translation: DVec3::ZERO,
                }
            } else {
                return Err(pyo3::exceptions::PyTypeError::new_err(
                    "chain expects Affine3, Vec3, or Mat3",
                ));
            };
            let new_t = tf.translation + tf.matrix3 * pose.translation;
            let new_r = tf.matrix3 * pose.matrix3;
            pose = DAffine3 {
                matrix3: new_r,
                translation: new_t,
            };
        }
        Ok(Self(pose))
    }

    /// Returns 4x4 SE(3) matrix as numpy array (row-major).
    fn as_matrix<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        let c = self.0.matrix3.to_cols_array_2d();
        let t = self.0.translation;
        let rows = vec![
            vec![c[0][0], c[1][0], c[2][0], t.x],
            vec![c[0][1], c[1][1], c[2][1], t.y],
            vec![c[0][2], c[1][2], c[2][2], t.z],
            vec![0.0, 0.0, 0.0, 1.0],
        ];
        PyArray2::from_vec2(py, &rows).unwrap()
    }

    /// Returns (Vec3, Mat3) tuple.
    fn as_components<'py>(&self, py: Python<'py>) -> PyResult<Py<PyTuple>> {
        let t = PyDVec3(self.0.translation);
        let r = PyDMat3(self.0.matrix3);
        let tuple = PyTuple::new(py, [
            t.into_pyobject(py)?.into_any(),
            r.into_pyobject(py)?.into_any(),
        ])?;
        Ok(tuple.unbind())
    }

    /// Inverse transform.
    fn inv(&self) -> Self {
        Self(self.0.inverse())
    }

    /// Getter for rotation as Mat3 (alias for matrix3).
    #[getter]
    fn rotation(&self) -> PyDMat3 {
        PyDMat3(self.0.matrix3)
    }

    /// Apply transform to a Vec3, Mat3, or (Vec3, Mat3) tuple.
    fn apply<'py>(&self, obj: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
        let py = obj.py();
        if let Ok(tuple) = obj.cast::<PyTuple>() {
            if tuple.len() == 2 {
                let t = tuple.get_item(0)?.extract::<PyDVec3>()?;
                let r = tuple.get_item(1)?.extract::<PyDMat3>()?;
                let new_t = PyDVec3(self.0.translation + self.0.matrix3 * t.0);
                let new_r = PyDMat3(self.0.matrix3 * r.0);
                let result = PyTuple::new(py, [
                    new_t.into_pyobject(py)?.into_any(),
                    new_r.into_pyobject(py)?.into_any(),
                ])?;
                return Ok(result.into_any().unbind());
            }
        }
        if let Ok(t) = obj.extract::<PyDVec3>() {
            let new_t = PyDVec3(self.0.translation + self.0.matrix3 * t.0);
            return Ok(new_t.into_pyobject(py)?.into_any().unbind());
        }
        if let Ok(r) = obj.extract::<PyDMat3>() {
            let new_r = PyDMat3(self.0.matrix3 * r.0);
            return Ok(new_r.into_pyobject(py)?.into_any().unbind());
        }
        Err(pyo3::exceptions::PyTypeError::new_err(
            "apply expects Vec3, Mat3, or (Vec3, Mat3) tuple",
        ))
    }

    fn apply_to_components(&self, other: &Self) -> Self {
        let new_t = self.0.translation + self.0.matrix3 * other.0.translation;
        let new_r = self.0.matrix3 * other.0.matrix3;
        Self(DAffine3 {
            matrix3: new_r,
            translation: new_t,
        })
    }

    fn interpolate_components(&self, other: &Self, t: f64) -> Self {
        let t = t.clamp(0.0, 1.0);
        let new_translation = self.0.translation.lerp(other.0.translation, t);
        let qa = DQuat::from_mat3(&self.0.matrix3).normalize();
        let qb = DQuat::from_mat3(&other.0.matrix3).normalize();
        let neg_a = qa.conjugate().normalize();
        let delta = (neg_a * qb).normalize();
        let scaled = {
            let w = delta.w.clamp(-1.0, 1.0);
            let angle = 2.0 * w.acos();
            let mut axis = DVec3::new(delta.x, delta.y, delta.z);
            if delta.w < 0.0 {
                axis = -axis;
            }
            let n = axis.length();
            if n < 1e-12 || angle == 0.0 || t == 0.0 {
                DQuat::IDENTITY
            } else {
                DQuat::from_axis_angle(axis / n, t * angle)
            }
        };
        let new_rotation = DMat3::from_quat((qa * scaled).normalize());
        Self(DAffine3 {
            matrix3: new_rotation,
            translation: new_translation,
        })
    }

    fn __truediv__(&self, scalar: f64) -> Self {
        let new_t = self.0.translation / scalar;
        let new_r = scale_rotation_mat(self.0.matrix3, 1.0 / scalar);
        Self(DAffine3 {
            matrix3: new_r,
            translation: new_t,
        })
    }
}
