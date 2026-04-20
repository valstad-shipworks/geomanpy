use glam::DAffine3;
use numpy::{PyArray2, PyReadonlyArray2};
use pyo3::prelude::*;

use super::{
    PyDMat3, PyDMat4, PyDQuat, PyDVec3, array2_from_rows, extract_numpy_matrix,
    impl_serde_methods, transpose_array2,
};
use crate::pickle::pickle_decode;
use crate::{impl_dataclass_fields, impl_getnewargs_ex};

#[pyclass(frozen, skip_from_py_object, name = "Affine3")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PyDAffine3(pub(crate) DAffine3);

impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyDAffine3 {
    type Error = pyo3::PyErr;
    fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(v) = ob.cast_exact::<Self>() {
            return Ok(v.get().clone());
        }
        let py = ob.py();
        let mat3: PyDMat3 = ob.getattr(pyo3::intern!(py, "matrix3"))?.extract()?;
        let trans: PyDVec3 = ob.getattr(pyo3::intern!(py, "translation"))?.extract()?;
        Ok(Self(DAffine3 {
            matrix3: mat3.0.into(),
            translation: trans.0,
        }))
    }
}

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
    #[pyo3(signature = (translation=None, rotation=None, *, __pickle_state__=None))]
    #[inline]
    fn new(
        translation: Option<PyDVec3>,
        rotation: Option<PyDMat3>,
        __pickle_state__: Option<Vec<u8>>,
    ) -> PyResult<Self> {
        if let Some(state) = __pickle_state__ {
            return Ok(Self(pickle_decode::<DAffine3>(&state)?));
        }
        match (translation, rotation) {
            (Some(t), Some(r)) => Ok(Self(DAffine3::from_mat3_translation(r.0, t.0))),
            _ => Err(pyo3::exceptions::PyValueError::new_err(
                "Affine3 requires translation and rotation arguments",
            )),
        }
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
}

/// Additional helper methods ported from valstad geom.py.
#[pymethods]
impl PyDAffine3 {
    /// Convert to a 4x4 homogeneous transformation matrix (numpy).
    fn to_matrix<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        let m = self.0.matrix3;
        let t = self.0.translation;
        let rows: [[f64; 4]; 4] = [
            [m.x_axis.x, m.y_axis.x, m.z_axis.x, t.x],
            [m.x_axis.y, m.y_axis.y, m.z_axis.y, t.y],
            [m.x_axis.z, m.y_axis.z, m.z_axis.z, t.z],
            [0.0, 0.0, 0.0, 1.0],
        ];
        array2_from_rows(py, rows)
    }

    /// Create an Affine3 from a 4x4 homogeneous transformation matrix (numpy).
    #[staticmethod]
    fn from_matrix(array: PyReadonlyArray2<'_, f64>) -> PyResult<Self> {
        let rows = extract_numpy_matrix::<4, 4>(array, "Affine3.from_matrix")?;
        let rot = glam::DMat3::from_cols(
            glam::DVec3::new(rows[0][0], rows[1][0], rows[2][0]),
            glam::DVec3::new(rows[0][1], rows[1][1], rows[2][1]),
            glam::DVec3::new(rows[0][2], rows[1][2], rows[2][2]),
        );
        let t = glam::DVec3::new(rows[0][3], rows[1][3], rows[2][3]);
        Ok(Self(DAffine3::from_mat3_translation(rot, t)))
    }
}

impl_serde_methods!(PyDAffine3, DAffine3);
impl_getnewargs_ex!(PyDAffine3);
impl_dataclass_fields!(PyDAffine3, ["matrix3", "translation"]);
