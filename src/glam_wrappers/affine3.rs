use glam::DAffine3;
use numpy::{PyArray2, PyReadonlyArray2};
use pyo3::prelude::*;

use super::{
    PyDMat3, PyDMat4, PyDQuat, PyDVec3, array2_from_rows, extract_numpy_matrix, transpose_array2,
};

#[pyclass(from_py_object, name = "Affine3")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PyDAffine3(pub(crate) DAffine3);

impl From<DAffine3> for PyDAffine3 {
    #[inline]
    fn from(a: DAffine3) -> Self {
        Self(a)
    }
}

impl From<PyDAffine3> for DAffine3 {
    #[inline]
    fn from(a: PyDAffine3) -> Self {
        a.0
    }
}

#[pymethods]
impl PyDAffine3 {
    #[new]
    #[inline]
    fn new(matrix3: PyDMat3, translation: PyDVec3) -> Self {
        Self(DAffine3::from_mat3_translation(matrix3.0, translation.0))
    }

    #[staticmethod]
    #[inline]
    fn identity() -> Self {
        Self(DAffine3::IDENTITY)
    }

    #[staticmethod]
    #[inline]
    fn from_cols(x_axis: PyDVec3, y_axis: PyDVec3, z_axis: PyDVec3, w_axis: PyDVec3) -> Self {
        Self(DAffine3::from_cols(x_axis.0, y_axis.0, z_axis.0, w_axis.0))
    }

    #[staticmethod]
    #[inline]
    fn from_cols_array(m: [f64; 12]) -> Self {
        Self(DAffine3::from_cols_array(&m))
    }

    #[staticmethod]
    #[inline]
    fn from_cols_array_2d(m: [[f64; 3]; 4]) -> Self {
        Self(DAffine3::from_cols_array_2d(&m))
    }

    #[staticmethod]
    #[inline]
    fn from_numpy(array: PyReadonlyArray2<'_, f64>) -> PyResult<Self> {
        let rows = extract_numpy_matrix::<3, 4>(array, "Affine3")?;
        Ok(Self(DAffine3::from_cols_array_2d(&transpose_array2(rows))))
    }

    #[staticmethod]
    #[inline]
    fn from_scale(scale: PyDVec3) -> Self {
        Self(DAffine3::from_scale(scale.0))
    }

    #[staticmethod]
    #[inline]
    fn from_quat(rotation: PyDQuat) -> Self {
        Self(DAffine3::from_quat(rotation.0))
    }

    #[staticmethod]
    #[inline]
    fn from_axis_angle(axis: PyDVec3, angle: f64) -> Self {
        Self(DAffine3::from_axis_angle(axis.0, angle))
    }

    #[staticmethod]
    #[inline]
    fn from_rotation_x(angle: f64) -> Self {
        Self(DAffine3::from_rotation_x(angle))
    }
    #[staticmethod]
    #[inline]
    fn from_rotation_y(angle: f64) -> Self {
        Self(DAffine3::from_rotation_y(angle))
    }
    #[staticmethod]
    #[inline]
    fn from_rotation_z(angle: f64) -> Self {
        Self(DAffine3::from_rotation_z(angle))
    }

    #[staticmethod]
    #[inline]
    fn from_translation(translation: PyDVec3) -> Self {
        Self(DAffine3::from_translation(translation.0))
    }

    #[staticmethod]
    #[inline]
    fn from_mat3(mat3: PyDMat3) -> Self {
        Self(DAffine3::from_mat3(mat3.0))
    }

    #[staticmethod]
    #[inline]
    fn from_mat3_translation(mat3: PyDMat3, translation: PyDVec3) -> Self {
        Self(DAffine3::from_mat3_translation(mat3.0, translation.0))
    }

    #[staticmethod]
    #[inline]
    fn from_scale_rotation_translation(
        scale: PyDVec3,
        rotation: PyDQuat,
        translation: PyDVec3,
    ) -> Self {
        Self(DAffine3::from_scale_rotation_translation(
            scale.0,
            rotation.0,
            translation.0,
        ))
    }

    #[staticmethod]
    #[inline]
    fn from_rotation_translation(rotation: PyDQuat, translation: PyDVec3) -> Self {
        Self(DAffine3::from_rotation_translation(
            rotation.0,
            translation.0,
        ))
    }

    #[staticmethod]
    #[inline]
    fn from_mat4(m: PyDMat4) -> Self {
        Self(DAffine3::from_mat4(m.0))
    }

    #[staticmethod]
    #[inline]
    fn look_to_lh(eye: PyDVec3, dir: PyDVec3, up: PyDVec3) -> Self {
        Self(DAffine3::look_to_lh(eye.0, dir.0, up.0))
    }

    #[staticmethod]
    #[inline]
    fn look_to_rh(eye: PyDVec3, dir: PyDVec3, up: PyDVec3) -> Self {
        Self(DAffine3::look_to_rh(eye.0, dir.0, up.0))
    }

    #[staticmethod]
    #[inline]
    fn look_at_lh(eye: PyDVec3, center: PyDVec3, up: PyDVec3) -> Self {
        Self(DAffine3::look_at_lh(eye.0, center.0, up.0))
    }

    #[staticmethod]
    #[inline]
    fn look_at_rh(eye: PyDVec3, center: PyDVec3, up: PyDVec3) -> Self {
        Self(DAffine3::look_at_rh(eye.0, center.0, up.0))
    }
}

#[pymethods]
impl PyDAffine3 {
    #[getter]
    #[inline]
    fn matrix3(&self) -> PyDMat3 {
        PyDMat3(self.0.matrix3)
    }

    #[getter]
    #[inline]
    fn translation(&self) -> PyDVec3 {
        PyDVec3(self.0.translation)
    }
}

#[pymethods]
impl PyDAffine3 {
    #[inline]
    fn transform_point3(&self, rhs: PyDVec3) -> PyDVec3 {
        PyDVec3(self.0.transform_point3(rhs.0))
    }

    #[inline]
    fn transform_vector3(&self, rhs: PyDVec3) -> PyDVec3 {
        PyDVec3(self.0.transform_vector3(rhs.0))
    }

    #[inline]
    fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }

    #[inline]
    fn to_scale_rotation_translation(&self) -> (PyDVec3, PyDQuat, PyDVec3) {
        let (s, r, t) = self.0.to_scale_rotation_translation();
        (PyDVec3(s), PyDQuat(r), PyDVec3(t))
    }

    #[inline]
    fn to_cols_array(&self) -> [f64; 12] {
        self.0.to_cols_array()
    }

    #[inline]
    fn to_cols_array_2d(&self) -> [[f64; 3]; 4] {
        self.0.to_cols_array_2d()
    }

    #[inline]
    fn to_numpy<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        array2_from_rows(py, transpose_array2(self.0.to_cols_array_2d()))
    }
}

#[pymethods]
impl PyDAffine3 {
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
impl PyDAffine3 {
    #[classattr]
    #[pyo3(name = "IDENTITY")]
    fn identity_const() -> Self {
        Self(DAffine3::IDENTITY)
    }
    #[classattr]
    #[pyo3(name = "ZERO")]
    fn zero_const() -> Self {
        Self(DAffine3::ZERO)
    }
    #[classattr]
    #[pyo3(name = "NAN")]
    fn nan_const() -> Self {
        Self(DAffine3::NAN)
    }
}

#[pymethods]
impl PyDAffine3 {
    fn __repr__(&self) -> String {
        format!(
            "Affine3(matrix3={:?}, translation={:?})",
            self.0.matrix3, self.0.translation
        )
    }

    fn __eq__(&self, other: Self) -> bool {
        self.0 == other.0
    }
    fn __ne__(&self, other: Self) -> bool {
        self.0 != other.0
    }

    fn __mul__(&self, other: Self) -> Self {
        Self(self.0 * other.0)
    }

    fn __getstate__(&self) -> [f64; 12] {
        self.0.to_cols_array()
    }
    fn __setstate__(&mut self, state: [f64; 12]) {
        self.0 = DAffine3::from_cols_array(&state);
    }
}
