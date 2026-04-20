use glam::DMat3;
use numpy::{PyArray2, PyReadonlyArray2};
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;

use super::{
    PyDMat4, PyDQuat, PyDVec2, PyDVec3, PyEulerRot, array2_from_rows, extract_numpy_matrix,
    impl_serde_methods, transpose_array2,
};
use crate::pickle::pickle_decode;
use crate::{impl_dataclass_fields, impl_getnewargs_ex};

#[pyclass(frozen, skip_from_py_object, name = "Mat3")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PyDMat3(pub(crate) DMat3);

impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyDMat3 {
    type Error = pyo3::PyErr;
    fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(v) = ob.cast_exact::<Self>() {
            return Ok(v.get().clone());
        }
        // Extract via to_cols_array_2d -> ((r00,r01,r02),(r10,r11,r12),(r20,r21,r22))
        let py = ob.py();
        let cols: ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64)) = ob
            .call_method0(pyo3::intern!(py, "to_cols_array_2d"))?
            .extract()?;
        Ok(Self(DMat3::from_cols(
            glam::DVec3::new(cols.0.0, cols.0.1, cols.0.2),
            glam::DVec3::new(cols.1.0, cols.1.1, cols.1.2),
            glam::DVec3::new(cols.2.0, cols.2.1, cols.2.2),
        )))
    }
}

impl From<DMat3> for PyDMat3 {
    #[inline]
    fn from(m: DMat3) -> Self {
        Self(m)
    }
}

impl From<PyDMat3> for DMat3 {
    #[inline]
    fn from(m: PyDMat3) -> Self {
        m.0
    }
}

#[pymethods]
impl PyDMat3 {
    #[new]
    #[pyo3(signature = (x_axis=None, y_axis=None, z_axis=None, *, __pickle_state__=None))]
    #[inline]
    fn new(
        x_axis: Option<PyDVec3>,
        y_axis: Option<PyDVec3>,
        z_axis: Option<PyDVec3>,
        __pickle_state__: Option<Vec<u8>>,
    ) -> PyResult<Self> {
        if let Some(state) = __pickle_state__ {
            return Ok(Self(pickle_decode::<DMat3>(&state)?));
        }
        match (x_axis, y_axis, z_axis) {
            (Some(x), Some(y), Some(z)) => Ok(Self(DMat3::from_cols(x.0, y.0, z.0))),
            _ => Err(pyo3::exceptions::PyValueError::new_err(
                "Mat3 requires x_axis, y_axis, z_axis arguments",
            )),
        }
    }

    #[staticmethod]
    #[inline]
    fn identity() -> Self {
        Self(DMat3::IDENTITY)
    }

    #[staticmethod]
    #[inline]
    fn from_cols(x_axis: PyDVec3, y_axis: PyDVec3, z_axis: PyDVec3) -> Self {
        Self(DMat3::from_cols(x_axis.0, y_axis.0, z_axis.0))
    }

    #[staticmethod]
    #[inline]
    fn from_cols_array(m: [f64; 9]) -> Self {
        Self(DMat3::from_cols_array(&m))
    }

    #[staticmethod]
    #[inline]
    fn from_cols_array_2d(m: [[f64; 3]; 3]) -> Self {
        Self(DMat3::from_cols_array_2d(&m))
    }

    #[staticmethod]
    #[inline]
    fn from_numpy(array: PyReadonlyArray2<'_, f64>) -> PyResult<Self> {
        let rows = extract_numpy_matrix::<3, 3>(array, "Mat3")?;
        Ok(Self(DMat3::from_cols_array_2d(&transpose_array2(rows))))
    }

    #[staticmethod]
    #[inline]
    fn from_diagonal(diagonal: PyDVec3) -> Self {
        Self(DMat3::from_diagonal(diagonal.0))
    }

    #[staticmethod]
    #[inline]
    fn from_quat(rotation: PyDQuat) -> Self {
        Self(DMat3::from_quat(rotation.0))
    }

    #[staticmethod]
    #[inline]
    fn from_axis_angle(axis: PyDVec3, angle: f64) -> Self {
        Self(DMat3::from_axis_angle(axis.0, angle))
    }

    #[staticmethod]
    #[inline]
    fn from_euler(order: PyEulerRot, a: f64, b: f64, c: f64) -> Self {
        Self(DMat3::from_euler(order.into(), a, b, c))
    }

    #[staticmethod]
    #[inline]
    fn from_rotation_x(angle: f64) -> Self {
        Self(DMat3::from_rotation_x(angle))
    }

    #[staticmethod]
    #[inline]
    fn from_rotation_y(angle: f64) -> Self {
        Self(DMat3::from_rotation_y(angle))
    }

    #[staticmethod]
    #[inline]
    fn from_rotation_z(angle: f64) -> Self {
        Self(DMat3::from_rotation_z(angle))
    }

    #[staticmethod]
    #[inline]
    fn from_angle(angle: f64) -> Self {
        Self(DMat3::from_angle(angle))
    }

    #[staticmethod]
    #[inline]
    fn from_translation(translation: PyDVec2) -> Self {
        Self(DMat3::from_translation(translation.0))
    }

    #[staticmethod]
    #[inline]
    fn from_scale(scale: PyDVec2) -> Self {
        Self(DMat3::from_scale(scale.0))
    }

    #[staticmethod]
    #[inline]
    fn from_scale_angle_translation(scale: PyDVec2, angle: f64, translation: PyDVec2) -> Self {
        Self(DMat3::from_scale_angle_translation(
            scale.0,
            angle,
            translation.0,
        ))
    }

    #[staticmethod]
    #[inline]
    fn from_mat4(m: PyDMat4) -> Self {
        Self(DMat3::from_mat4(m.0))
    }

    #[staticmethod]
    #[inline]
    fn from_mat4_minor(m: PyDMat4, i: usize, j: usize) -> Self {
        Self(DMat3::from_mat4_minor(m.0, i, j))
    }

    #[staticmethod]
    #[inline]
    fn look_to_lh(dir: PyDVec3, up: PyDVec3) -> Self {
        Self(DMat3::look_to_lh(dir.0, up.0))
    }

    #[staticmethod]
    #[inline]
    fn look_to_rh(dir: PyDVec3, up: PyDVec3) -> Self {
        Self(DMat3::look_to_rh(dir.0, up.0))
    }
}

#[pymethods]
impl PyDMat3 {
    #[getter]
    #[inline]
    fn x_axis(&self) -> PyDVec3 {
        PyDVec3(self.0.x_axis)
    }
    #[getter]
    #[inline]
    fn y_axis(&self) -> PyDVec3 {
        PyDVec3(self.0.y_axis)
    }
    #[getter]
    #[inline]
    fn z_axis(&self) -> PyDVec3 {
        PyDVec3(self.0.z_axis)
    }

    #[inline]
    fn col(&self, index: usize) -> PyResult<PyDVec3> {
        if index < 3 {
            Ok(PyDVec3(self.0.col(index)))
        } else {
            Err(PyIndexError::new_err("column index out of range"))
        }
    }

    #[inline]
    fn row(&self, index: usize) -> PyResult<PyDVec3> {
        if index < 3 {
            Ok(PyDVec3(self.0.row(index)))
        } else {
            Err(PyIndexError::new_err("row index out of range"))
        }
    }
}

#[pymethods]
impl PyDMat3 {
    #[inline]
    fn to_cols_array(&self) -> [f64; 9] {
        self.0.to_cols_array()
    }

    #[inline]
    fn to_cols_array_2d(&self) -> [[f64; 3]; 3] {
        self.0.to_cols_array_2d()
    }

    #[inline]
    fn to_numpy<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        array2_from_rows(py, transpose_array2(self.0.to_cols_array_2d()))
    }
}

#[pymethods]
impl PyDMat3 {
    #[inline]
    fn diagonal(&self) -> PyDVec3 {
        PyDVec3(self.0.diagonal())
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
    fn to_euler(&self, order: PyEulerRot) -> (f64, f64, f64) {
        self.0.to_euler(order.into())
    }
}

#[pymethods]
impl PyDMat3 {
    #[inline]
    fn mul_vec3(&self, rhs: PyDVec3) -> PyDVec3 {
        PyDVec3(self.0.mul_vec3(rhs.0))
    }
    #[inline]
    fn mul_transpose_vec3(&self, rhs: PyDVec3) -> PyDVec3 {
        PyDVec3(self.0.mul_transpose_vec3(rhs.0))
    }
    #[inline]
    fn transform_point2(&self, rhs: PyDVec2) -> PyDVec2 {
        PyDVec2(self.0.transform_point2(rhs.0))
    }
    #[inline]
    fn transform_vector2(&self, rhs: PyDVec2) -> PyDVec2 {
        PyDVec2(self.0.transform_vector2(rhs.0))
    }
}

#[pymethods]
impl PyDMat3 {
    #[inline]
    fn mul_mat3(&self, rhs: Self) -> Self {
        Self(self.0.mul_mat3(&rhs.0))
    }
    #[inline]
    fn add_mat3(&self, rhs: Self) -> Self {
        Self(self.0.add_mat3(&rhs.0))
    }
    #[inline]
    fn sub_mat3(&self, rhs: Self) -> Self {
        Self(self.0.sub_mat3(&rhs.0))
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
    fn mul_diagonal_scale(&self, scale: PyDVec3) -> Self {
        Self(self.0.mul_diagonal_scale(scale.0))
    }
}

#[pymethods]
impl PyDMat3 {
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
impl PyDMat3 {
    #[classattr]
    #[pyo3(name = "IDENTITY")]
    fn identity_const() -> Self {
        Self(DMat3::IDENTITY)
    }
    #[classattr]
    #[pyo3(name = "ZERO")]
    fn zero_const() -> Self {
        Self(DMat3::ZERO)
    }
    #[classattr]
    #[pyo3(name = "NAN")]
    fn nan_const() -> Self {
        Self(DMat3::NAN)
    }
}

#[pymethods]
impl PyDMat3 {
    fn __repr__(&self) -> String {
        format!(
            "Mat3([{}, {}, {}], [{}, {}, {}], [{}, {}, {}])",
            self.0.x_axis.x,
            self.0.x_axis.y,
            self.0.x_axis.z,
            self.0.y_axis.x,
            self.0.y_axis.y,
            self.0.y_axis.z,
            self.0.z_axis.x,
            self.0.z_axis.y,
            self.0.z_axis.z,
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
        } else if let Ok(v) = other.extract::<PyDVec3>() {
            Ok(PyDVec3(self.0 * v.0).into_pyobject(py)?.into_any().unbind())
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

    fn __array__<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        self.to_numpy(py)
    }
}

impl_serde_methods!(PyDMat3, DMat3);
impl_getnewargs_ex!(PyDMat3);
impl_dataclass_fields!(PyDMat3, ["x_axis", "y_axis", "z_axis"]);
