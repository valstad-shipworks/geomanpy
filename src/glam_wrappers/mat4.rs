use glam::DMat4;
use numpy::{PyArray2, PyReadonlyArray2};
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;

use super::{
    PyDMat3, PyDQuat, PyDVec3, PyDVec4, PyEulerRot, array2_from_rows, extract_numpy_matrix,
    transpose_array2,
};

#[pyclass(from_py_object, name = "Mat4")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PyDMat4(pub(crate) DMat4);

impl From<DMat4> for PyDMat4 {
    #[inline]
    fn from(m: DMat4) -> Self {
        Self(m)
    }
}

impl From<PyDMat4> for DMat4 {
    #[inline]
    fn from(m: PyDMat4) -> Self {
        m.0
    }
}

#[pymethods]
impl PyDMat4 {
    #[new]
    #[inline]
    fn new(x_axis: PyDVec4, y_axis: PyDVec4, z_axis: PyDVec4, w_axis: PyDVec4) -> Self {
        Self(DMat4::from_cols(x_axis.0, y_axis.0, z_axis.0, w_axis.0))
    }

    #[staticmethod]
    #[inline]
    fn identity() -> Self {
        Self(DMat4::IDENTITY)
    }

    #[staticmethod]
    #[inline]
    fn from_cols(x_axis: PyDVec4, y_axis: PyDVec4, z_axis: PyDVec4, w_axis: PyDVec4) -> Self {
        Self(DMat4::from_cols(x_axis.0, y_axis.0, z_axis.0, w_axis.0))
    }

    #[staticmethod]
    #[inline]
    fn from_cols_array(m: [f64; 16]) -> Self {
        Self(DMat4::from_cols_array(&m))
    }

    #[staticmethod]
    #[inline]
    fn from_cols_array_2d(m: [[f64; 4]; 4]) -> Self {
        Self(DMat4::from_cols_array_2d(&m))
    }

    #[staticmethod]
    #[inline]
    fn from_numpy(array: PyReadonlyArray2<'_, f64>) -> PyResult<Self> {
        let rows = extract_numpy_matrix::<4, 4>(array, "Mat4")?;
        Ok(Self(DMat4::from_cols_array_2d(&transpose_array2(rows))))
    }

    #[staticmethod]
    #[inline]
    fn from_diagonal(diagonal: PyDVec4) -> Self {
        Self(DMat4::from_diagonal(diagonal.0))
    }

    #[staticmethod]
    #[inline]
    fn from_scale_rotation_translation(
        scale: PyDVec3,
        rotation: PyDQuat,
        translation: PyDVec3,
    ) -> Self {
        Self(DMat4::from_scale_rotation_translation(
            scale.0,
            rotation.0,
            translation.0,
        ))
    }

    #[staticmethod]
    #[inline]
    fn from_rotation_translation(rotation: PyDQuat, translation: PyDVec3) -> Self {
        Self(DMat4::from_rotation_translation(rotation.0, translation.0))
    }

    #[staticmethod]
    #[inline]
    fn from_quat(rotation: PyDQuat) -> Self {
        Self(DMat4::from_quat(rotation.0))
    }

    #[staticmethod]
    #[inline]
    fn from_mat3(m: PyDMat3) -> Self {
        Self(DMat4::from_mat3(m.0))
    }

    #[staticmethod]
    #[inline]
    fn from_mat3_translation(mat3: PyDMat3, translation: PyDVec3) -> Self {
        Self(DMat4::from_mat3_translation(mat3.0, translation.0))
    }

    #[staticmethod]
    #[inline]
    fn from_translation(translation: PyDVec3) -> Self {
        Self(DMat4::from_translation(translation.0))
    }

    #[staticmethod]
    #[inline]
    fn from_axis_angle(axis: PyDVec3, angle: f64) -> Self {
        Self(DMat4::from_axis_angle(axis.0, angle))
    }

    #[staticmethod]
    #[inline]
    fn from_euler(order: PyEulerRot, a: f64, b: f64, c: f64) -> Self {
        Self(DMat4::from_euler(order.into(), a, b, c))
    }

    #[staticmethod]
    #[inline]
    fn from_rotation_x(angle: f64) -> Self {
        Self(DMat4::from_rotation_x(angle))
    }
    #[staticmethod]
    #[inline]
    fn from_rotation_y(angle: f64) -> Self {
        Self(DMat4::from_rotation_y(angle))
    }
    #[staticmethod]
    #[inline]
    fn from_rotation_z(angle: f64) -> Self {
        Self(DMat4::from_rotation_z(angle))
    }

    #[staticmethod]
    #[inline]
    fn from_scale(scale: PyDVec3) -> Self {
        Self(DMat4::from_scale(scale.0))
    }
}

#[pymethods]
impl PyDMat4 {
    #[staticmethod]
    fn perspective_rh_gl(fov_y_radians: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Self {
        Self(DMat4::perspective_rh_gl(
            fov_y_radians,
            aspect_ratio,
            z_near,
            z_far,
        ))
    }
    #[staticmethod]
    fn perspective_lh(fov_y_radians: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Self {
        Self(DMat4::perspective_lh(
            fov_y_radians,
            aspect_ratio,
            z_near,
            z_far,
        ))
    }
    #[staticmethod]
    fn perspective_rh(fov_y_radians: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Self {
        Self(DMat4::perspective_rh(
            fov_y_radians,
            aspect_ratio,
            z_near,
            z_far,
        ))
    }
    #[staticmethod]
    fn perspective_infinite_lh(fov_y_radians: f64, aspect_ratio: f64, z_near: f64) -> Self {
        Self(DMat4::perspective_infinite_lh(
            fov_y_radians,
            aspect_ratio,
            z_near,
        ))
    }
    #[staticmethod]
    fn perspective_infinite_rh(fov_y_radians: f64, aspect_ratio: f64, z_near: f64) -> Self {
        Self(DMat4::perspective_infinite_rh(
            fov_y_radians,
            aspect_ratio,
            z_near,
        ))
    }
    #[staticmethod]
    fn perspective_infinite_reverse_lh(fov_y_radians: f64, aspect_ratio: f64, z_near: f64) -> Self {
        Self(DMat4::perspective_infinite_reverse_lh(
            fov_y_radians,
            aspect_ratio,
            z_near,
        ))
    }
    #[staticmethod]
    fn perspective_infinite_reverse_rh(fov_y_radians: f64, aspect_ratio: f64, z_near: f64) -> Self {
        Self(DMat4::perspective_infinite_reverse_rh(
            fov_y_radians,
            aspect_ratio,
            z_near,
        ))
    }
    #[staticmethod]
    fn orthographic_rh_gl(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    ) -> Self {
        Self(DMat4::orthographic_rh_gl(
            left, right, bottom, top, near, far,
        ))
    }
    #[staticmethod]
    fn orthographic_lh(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> Self {
        Self(DMat4::orthographic_lh(left, right, bottom, top, near, far))
    }
    #[staticmethod]
    fn orthographic_rh(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> Self {
        Self(DMat4::orthographic_rh(left, right, bottom, top, near, far))
    }
    #[staticmethod]
    fn frustum_rh_gl(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        z_near: f64,
        z_far: f64,
    ) -> Self {
        Self(DMat4::frustum_rh_gl(
            left, right, bottom, top, z_near, z_far,
        ))
    }
    #[staticmethod]
    fn frustum_lh(left: f64, right: f64, bottom: f64, top: f64, z_near: f64, z_far: f64) -> Self {
        Self(DMat4::frustum_lh(left, right, bottom, top, z_near, z_far))
    }
    #[staticmethod]
    fn frustum_rh(left: f64, right: f64, bottom: f64, top: f64, z_near: f64, z_far: f64) -> Self {
        Self(DMat4::frustum_rh(left, right, bottom, top, z_near, z_far))
    }
}

#[pymethods]
impl PyDMat4 {
    #[staticmethod]
    fn look_to_lh(eye: PyDVec3, dir: PyDVec3, up: PyDVec3) -> Self {
        Self(DMat4::look_to_lh(eye.0, dir.0, up.0))
    }
    #[staticmethod]
    fn look_to_rh(eye: PyDVec3, dir: PyDVec3, up: PyDVec3) -> Self {
        Self(DMat4::look_to_rh(eye.0, dir.0, up.0))
    }
    #[staticmethod]
    fn look_at_lh(eye: PyDVec3, center: PyDVec3, up: PyDVec3) -> Self {
        Self(DMat4::look_at_lh(eye.0, center.0, up.0))
    }
    #[staticmethod]
    fn look_at_rh(eye: PyDVec3, center: PyDVec3, up: PyDVec3) -> Self {
        Self(DMat4::look_at_rh(eye.0, center.0, up.0))
    }
}

#[pymethods]
impl PyDMat4 {
    #[getter]
    #[inline]
    fn x_axis(&self) -> PyDVec4 {
        PyDVec4(self.0.x_axis)
    }
    #[getter]
    #[inline]
    fn y_axis(&self) -> PyDVec4 {
        PyDVec4(self.0.y_axis)
    }
    #[getter]
    #[inline]
    fn z_axis(&self) -> PyDVec4 {
        PyDVec4(self.0.z_axis)
    }
    #[getter]
    #[inline]
    fn w_axis(&self) -> PyDVec4 {
        PyDVec4(self.0.w_axis)
    }

    #[inline]
    fn col(&self, index: usize) -> PyResult<PyDVec4> {
        if index < 4 {
            Ok(PyDVec4(self.0.col(index)))
        } else {
            Err(PyIndexError::new_err("column index out of range"))
        }
    }

    #[inline]
    fn row(&self, index: usize) -> PyResult<PyDVec4> {
        if index < 4 {
            Ok(PyDVec4(self.0.row(index)))
        } else {
            Err(PyIndexError::new_err("row index out of range"))
        }
    }
}

#[pymethods]
impl PyDMat4 {
    #[inline]
    fn to_cols_array(&self) -> [f64; 16] {
        self.0.to_cols_array()
    }
    #[inline]
    fn to_cols_array_2d(&self) -> [[f64; 4]; 4] {
        self.0.to_cols_array_2d()
    }

    #[inline]
    fn to_numpy<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        array2_from_rows(py, transpose_array2(self.0.to_cols_array_2d()))
    }
}

#[pymethods]
impl PyDMat4 {
    #[inline]
    fn diagonal(&self) -> PyDVec4 {
        PyDVec4(self.0.diagonal())
    }
    #[inline]
    fn determinant(&self) -> f64 {
        self.0.determinant()
    }
    #[inline]
    fn transpose(&self) -> Self {
        Self(self.0.transpose())
    }
    #[inline]
    fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }
    #[inline]
    fn try_inverse(&self) -> Option<Self> {
        self.0.try_inverse().map(Self)
    }
    #[inline]
    fn inverse_or_zero(&self) -> Self {
        Self(self.0.inverse_or_zero())
    }

    #[inline]
    fn to_scale_rotation_translation(&self) -> (PyDVec3, PyDQuat, PyDVec3) {
        let (s, r, t) = self.0.to_scale_rotation_translation();
        (PyDVec3(s), PyDQuat(r), PyDVec3(t))
    }

    #[inline]
    fn to_euler(&self, order: PyEulerRot) -> (f64, f64, f64) {
        self.0.to_euler(order.into())
    }
}

#[pymethods]
impl PyDMat4 {
    #[inline]
    fn mul_vec4(&self, rhs: PyDVec4) -> PyDVec4 {
        PyDVec4(self.0.mul_vec4(rhs.0))
    }
    #[inline]
    fn mul_transpose_vec4(&self, rhs: PyDVec4) -> PyDVec4 {
        PyDVec4(self.0.mul_transpose_vec4(rhs.0))
    }
    #[inline]
    fn project_point3(&self, rhs: PyDVec3) -> PyDVec3 {
        PyDVec3(self.0.project_point3(rhs.0))
    }
    #[inline]
    fn transform_point3(&self, rhs: PyDVec3) -> PyDVec3 {
        PyDVec3(self.0.transform_point3(rhs.0))
    }
    #[inline]
    fn transform_vector3(&self, rhs: PyDVec3) -> PyDVec3 {
        PyDVec3(self.0.transform_vector3(rhs.0))
    }
}

#[pymethods]
impl PyDMat4 {
    #[inline]
    fn mul_mat4(&self, rhs: Self) -> Self {
        Self(self.0.mul_mat4(&rhs.0))
    }
    #[inline]
    fn add_mat4(&self, rhs: Self) -> Self {
        Self(self.0.add_mat4(&rhs.0))
    }
    #[inline]
    fn sub_mat4(&self, rhs: Self) -> Self {
        Self(self.0.sub_mat4(&rhs.0))
    }
    #[inline]
    fn mul_scalar(&self, rhs: f64) -> Self {
        Self(self.0.mul_scalar(rhs))
    }
    #[inline]
    fn div_scalar(&self, rhs: f64) -> Self {
        Self(self.0.div_scalar(rhs))
    }
    #[inline]
    fn mul_diagonal_scale(&self, scale: PyDVec4) -> Self {
        Self(self.0.mul_diagonal_scale(scale.0))
    }
}

#[pymethods]
impl PyDMat4 {
    #[inline]
    fn abs(&self) -> Self {
        Self(self.0.abs())
    }
    #[inline]
    fn recip(&self) -> Self {
        Self(self.0.recip())
    }
    #[inline]
    fn is_finite(&self) -> bool {
        self.0.is_finite()
    }
    #[inline]
    fn is_nan(&self) -> bool {
        self.0.is_nan()
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
impl PyDMat4 {
    #[classattr]
    #[pyo3(name = "IDENTITY")]
    fn identity_const() -> Self {
        Self(DMat4::IDENTITY)
    }
    #[classattr]
    #[pyo3(name = "ZERO")]
    fn zero_const() -> Self {
        Self(DMat4::ZERO)
    }
    #[classattr]
    #[pyo3(name = "NAN")]
    fn nan_const() -> Self {
        Self(DMat4::NAN)
    }
}

#[pymethods]
impl PyDMat4 {
    fn __repr__(&self) -> String {
        let c = self.0.to_cols_array_2d();
        format!(
            "Mat4([{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}])",
            c[0][0],
            c[0][1],
            c[0][2],
            c[0][3],
            c[1][0],
            c[1][1],
            c[1][2],
            c[1][3],
            c[2][0],
            c[2][1],
            c[2][2],
            c[2][3],
            c[3][0],
            c[3][1],
            c[3][2],
            c[3][3],
        )
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
        if let Ok(m) = other.extract::<Self>() {
            Ok(Self(self.0 * m.0).into_pyobject(py)?.into_any().unbind())
        } else if let Ok(v) = other.extract::<PyDVec4>() {
            Ok(PyDVec4(self.0 * v.0).into_pyobject(py)?.into_any().unbind())
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
            Ok(Self(s * self.0))
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

    fn __getstate__(&self) -> [f64; 16] {
        self.0.to_cols_array()
    }
    fn __setstate__(&mut self, state: [f64; 16]) {
        self.0 = DMat4::from_cols_array(&state);
    }

    fn __array__<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        self.to_numpy(py)
    }
}
