use glam::DQuat;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

use super::{PyDAffine3, PyDMat3, PyDMat4, PyDVec2, PyDVec3, PyEulerRot, extract_numpy_vector, impl_serde_methods};

#[pyclass(skip_from_py_object, name = "Quat")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PyDQuat(pub(crate) DQuat);

impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyDQuat {
    type Error = pyo3::PyErr;
    fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(v) = ob.cast::<Self>() {
            return Ok(v.borrow().clone());
        }
        let x: f64 = ob.getattr("x")?.extract()?;
        let y: f64 = ob.getattr("y")?.extract()?;
        let z: f64 = ob.getattr("z")?.extract()?;
        let w: f64 = ob.getattr("w")?.extract()?;
        Ok(Self(DQuat::from_xyzw(x, y, z, w)))
    }
}

impl From<DQuat> for PyDQuat {
    #[inline]
    fn from(q: DQuat) -> Self {
        Self(q)
    }
}

impl From<PyDQuat> for DQuat {
    #[inline]
    fn from(q: PyDQuat) -> Self {
        q.0
    }
}

#[pymethods]
impl PyDQuat {
    #[new]
    #[inline]
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self(DQuat::from_xyzw(x, y, z, w))
    }

    #[staticmethod]
    #[inline]
    fn from_xyzw(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self(DQuat::from_xyzw(x, y, z, w))
    }

    #[staticmethod]
    #[inline]
    fn from_array(a: [f64; 4]) -> Self {
        Self(DQuat::from_array(a))
    }

    #[staticmethod]
    #[inline]
    fn from_numpy(array: PyReadonlyArray1<'_, f64>) -> PyResult<Self> {
        Ok(Self(DQuat::from_array(extract_numpy_vector::<4>(
            array, "Quat",
        )?)))
    }

    #[staticmethod]
    #[inline]
    fn identity() -> Self {
        Self(DQuat::IDENTITY)
    }

    #[staticmethod]
    #[inline]
    fn from_axis_angle(axis: PyDVec3, angle: f64) -> Self {
        Self(DQuat::from_axis_angle(axis.0, angle))
    }

    #[staticmethod]
    #[inline]
    fn from_scaled_axis(v: PyDVec3) -> Self {
        Self(DQuat::from_scaled_axis(v.0))
    }

    #[staticmethod]
    #[inline]
    fn from_rotation_x(angle: f64) -> Self {
        Self(DQuat::from_rotation_x(angle))
    }

    #[staticmethod]
    #[inline]
    fn from_rotation_y(angle: f64) -> Self {
        Self(DQuat::from_rotation_y(angle))
    }

    #[staticmethod]
    #[inline]
    fn from_rotation_z(angle: f64) -> Self {
        Self(DQuat::from_rotation_z(angle))
    }

    #[staticmethod]
    #[inline]
    fn from_euler(order: PyEulerRot, a: f64, b: f64, c: f64) -> Self {
        Self(DQuat::from_euler(order.into(), a, b, c))
    }

    #[staticmethod]
    #[inline]
    fn from_rotation_axes(x_axis: PyDVec3, y_axis: PyDVec3, z_axis: PyDVec3) -> Self {
        Self(DQuat::from_rotation_axes(x_axis.0, y_axis.0, z_axis.0))
    }

    #[staticmethod]
    #[inline]
    fn from_mat3(mat: PyDMat3) -> Self {
        Self(DQuat::from_mat3(&mat.0))
    }

    #[staticmethod]
    #[inline]
    fn from_mat4(mat: PyDMat4) -> Self {
        Self(DQuat::from_mat4(&mat.0))
    }

    #[staticmethod]
    #[inline]
    fn from_affine3(a: PyDAffine3) -> Self {
        Self(DQuat::from_affine3(&a.0))
    }

    #[staticmethod]
    #[inline]
    fn from_rotation_arc(from: PyDVec3, to: PyDVec3) -> Self {
        Self(DQuat::from_rotation_arc(from.0, to.0))
    }

    #[staticmethod]
    #[inline]
    fn from_rotation_arc_colinear(from: PyDVec3, to: PyDVec3) -> Self {
        Self(DQuat::from_rotation_arc_colinear(from.0, to.0))
    }

    #[staticmethod]
    #[inline]
    fn from_rotation_arc_2d(from: PyDVec2, to: PyDVec2) -> Self {
        Self(DQuat::from_rotation_arc_2d(from.0, to.0))
    }

    #[staticmethod]
    #[inline]
    fn look_to_lh(dir: PyDVec3, up: PyDVec3) -> Self {
        Self(DQuat::look_to_lh(dir.0, up.0))
    }

    #[staticmethod]
    #[inline]
    fn look_to_rh(dir: PyDVec3, up: PyDVec3) -> Self {
        Self(DQuat::look_to_rh(dir.0, up.0))
    }

    #[staticmethod]
    #[inline]
    fn look_at_lh(eye: PyDVec3, center: PyDVec3, up: PyDVec3) -> Self {
        Self(DQuat::look_at_lh(eye.0, center.0, up.0))
    }

    #[staticmethod]
    #[inline]
    fn look_at_rh(eye: PyDVec3, center: PyDVec3, up: PyDVec3) -> Self {
        Self(DQuat::look_at_rh(eye.0, center.0, up.0))
    }
}

#[pymethods]
impl PyDQuat {
    #[getter]
    #[inline]
    fn x(&self) -> f64 {
        self.0.x
    }
    #[getter]
    #[inline]
    fn y(&self) -> f64 {
        self.0.y
    }
    #[getter]
    #[inline]
    fn z(&self) -> f64 {
        self.0.z
    }
    #[getter]
    #[inline]
    fn w(&self) -> f64 {
        self.0.w
    }
}

#[pymethods]
impl PyDQuat {
    #[inline]
    fn conjugate(&self) -> Self {
        Self(self.0.conjugate())
    }
    #[inline]
    fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }
    #[inline]
    fn dot(&self, rhs: Self) -> f64 {
        self.0.dot(rhs.0)
    }
    #[inline]
    fn length(&self) -> f64 {
        self.0.length()
    }
    #[inline]
    fn length_squared(&self) -> f64 {
        self.0.length_squared()
    }
    #[inline]
    fn length_recip(&self) -> f64 {
        self.0.length_recip()
    }
    #[inline]
    fn normalize(&self) -> Self {
        Self(self.0.normalize())
    }
    #[inline]
    fn mul_vec3(&self, rhs: PyDVec3) -> PyDVec3 {
        PyDVec3(self.0.mul_vec3(rhs.0))
    }
    #[inline]
    fn mul_quat(&self, rhs: Self) -> Self {
        Self(self.0.mul_quat(rhs.0))
    }
}

#[pymethods]
impl PyDQuat {
    #[inline]
    fn lerp(&self, end: Self, s: f64) -> Self {
        Self(self.0.lerp(end.0, s))
    }
    #[inline]
    fn slerp(&self, end: Self, s: f64) -> Self {
        Self(self.0.slerp(end.0, s))
    }
    #[inline]
    fn angle_between(&self, rhs: Self) -> f64 {
        self.0.angle_between(rhs.0)
    }
    #[inline]
    fn rotate_towards(&self, rhs: Self, max_angle: f64) -> Self {
        Self(self.0.rotate_towards(rhs.0, max_angle))
    }
}

#[pymethods]
impl PyDQuat {
    #[inline]
    fn to_axis_angle(&self) -> (PyDVec3, f64) {
        let (axis, angle) = self.0.to_axis_angle();
        (PyDVec3(axis), angle)
    }

    #[inline]
    fn to_scaled_axis(&self) -> PyDVec3 {
        PyDVec3(self.0.to_scaled_axis())
    }

    #[inline]
    fn to_euler(&self, order: PyEulerRot) -> (f64, f64, f64) {
        self.0.to_euler(order.into())
    }

    #[inline]
    fn to_array(&self) -> [f64; 4] {
        self.0.to_array()
    }

    #[inline]
    fn to_numpy<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_slice(py, &self.0.to_array())
    }

    #[inline]
    fn xyz(&self) -> PyDVec3 {
        PyDVec3(self.0.xyz())
    }
}

#[pymethods]
impl PyDQuat {
    #[inline]
    fn is_finite(&self) -> bool {
        self.0.is_finite()
    }
    #[inline]
    fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
    #[inline]
    fn is_normalized(&self) -> bool {
        self.0.is_normalized()
    }
    #[inline]
    fn is_near_identity(&self) -> bool {
        self.0.is_near_identity()
    }
    #[inline]
    fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f64) -> bool {
        self.0.abs_diff_eq(rhs.0, max_abs_diff)
    }
    #[inline]
    fn relative_eq(&self, rhs: Self, max_abs_diff: f64, max_relative: f64) -> bool {
        approx::RelativeEq::relative_eq(&self.0, &rhs.0, max_abs_diff, max_relative)
    }
}

#[pymethods]
impl PyDQuat {
    #[classattr]
    #[pyo3(name = "IDENTITY")]
    fn identity_const() -> Self {
        Self(DQuat::IDENTITY)
    }
    #[classattr]
    #[pyo3(name = "NAN")]
    fn nan_const() -> Self {
        Self(DQuat::NAN)
    }
}

#[pymethods]
impl PyDQuat {
    fn __repr__(&self) -> String {
        format!(
            "Quat({}, {}, {}, {})",
            self.0.x, self.0.y, self.0.z, self.0.w
        )
    }

    fn __str__(&self) -> String {
        format!("[{}, {}, {}, {}]", self.0.x, self.0.y, self.0.z, self.0.w)
    }

    fn __eq__(&self, other: Self) -> bool {
        self.0 == other.0
    }
    fn __ne__(&self, other: Self) -> bool {
        self.0 != other.0
    }
    fn __neg__(&self) -> Self {
        Self(-self.0)
    }

    fn __mul__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
        let py = other.py();
        if let Ok(q) = other.extract::<Self>() {
            Ok(Self(self.0.mul_quat(q.0))
                .into_pyobject(py)?
                .into_any()
                .unbind())
        } else if let Ok(v) = other.extract::<PyDVec3>() {
            Ok(PyDVec3(self.0.mul_vec3(v.0))
                .into_pyobject(py)?
                .into_any()
                .unbind())
        } else if let Ok(s) = other.extract::<f64>() {
            Ok(Self(self.0 * s).into_pyobject(py)?.into_any().unbind())
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type for *",
            ))
        }
    }

    fn __rmul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(s) = other.extract::<f64>() {
            Ok(Self(self.0 * s))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type for *",
            ))
        }
    }

    fn __add__(&self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
    fn __sub__(&self, other: Self) -> Self {
        Self(self.0 - other.0)
    }

    fn __truediv__(&self, other: f64) -> Self {
        Self(self.0 / other)
    }

    fn __getstate__(&self) -> [f64; 4] {
        self.0.to_array()
    }
    fn __setstate__(&mut self, state: [f64; 4]) {
        self.0 = DQuat::from_array(state);
    }
}

impl_serde_methods!(PyDQuat, DQuat);
