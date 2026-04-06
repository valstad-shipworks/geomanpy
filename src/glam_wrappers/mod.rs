mod affine3;
mod mat3;
mod mat4;
mod quat;
mod vec2;
mod vec3;
mod vec4;

pub use affine3::PyDAffine3;
pub use mat3::PyDMat3;
pub use mat4::PyDMat4;
pub use quat::PyDQuat;
pub use vec2::PyDVec2;
pub use vec3::PyDVec3;
pub use vec4::PyDVec4;

use glam::EulerRot;
use numpy::{PyArray2, PyReadonlyArray1, PyReadonlyArray2};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass(eq, eq_int, from_py_object, name = "EulerRot")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyEulerRot {
    ZYX,
    ZXY,
    YXZ,
    YZX,
    XYZ,
    XZY,
    ZYZ,
    ZXZ,
    YXY,
    YZY,
    XYX,
    XZX,
    ZYXEx,
    ZXYEx,
    YXZEx,
    YZXEx,
    XYZEx,
    XZYEx,
    ZYZEx,
    ZXZEx,
    YXYEx,
    YZYEx,
    XYXEx,
    XZXEx,
}

impl From<PyEulerRot> for EulerRot {
    #[inline]
    fn from(e: PyEulerRot) -> Self {
        match e {
            PyEulerRot::ZYX => EulerRot::ZYX,
            PyEulerRot::ZXY => EulerRot::ZXY,
            PyEulerRot::YXZ => EulerRot::YXZ,
            PyEulerRot::YZX => EulerRot::YZX,
            PyEulerRot::XYZ => EulerRot::XYZ,
            PyEulerRot::XZY => EulerRot::XZY,
            PyEulerRot::ZYZ => EulerRot::ZYZ,
            PyEulerRot::ZXZ => EulerRot::ZXZ,
            PyEulerRot::YXY => EulerRot::YXY,
            PyEulerRot::YZY => EulerRot::YZY,
            PyEulerRot::XYX => EulerRot::XYX,
            PyEulerRot::XZX => EulerRot::XZX,
            PyEulerRot::ZYXEx => EulerRot::ZYXEx,
            PyEulerRot::ZXYEx => EulerRot::ZXYEx,
            PyEulerRot::YXZEx => EulerRot::YXZEx,
            PyEulerRot::YZXEx => EulerRot::YZXEx,
            PyEulerRot::XYZEx => EulerRot::XYZEx,
            PyEulerRot::XZYEx => EulerRot::XZYEx,
            PyEulerRot::ZYZEx => EulerRot::ZYZEx,
            PyEulerRot::ZXZEx => EulerRot::ZXZEx,
            PyEulerRot::YXYEx => EulerRot::YXYEx,
            PyEulerRot::YZYEx => EulerRot::YZYEx,
            PyEulerRot::XYXEx => EulerRot::XYXEx,
            PyEulerRot::XZXEx => EulerRot::XZXEx,
        }
    }
}

impl From<EulerRot> for PyEulerRot {
    #[inline]
    fn from(e: EulerRot) -> Self {
        match e {
            EulerRot::ZYX => PyEulerRot::ZYX,
            EulerRot::ZXY => PyEulerRot::ZXY,
            EulerRot::YXZ => PyEulerRot::YXZ,
            EulerRot::YZX => PyEulerRot::YZX,
            EulerRot::XYZ => PyEulerRot::XYZ,
            EulerRot::XZY => PyEulerRot::XZY,
            EulerRot::ZYZ => PyEulerRot::ZYZ,
            EulerRot::ZXZ => PyEulerRot::ZXZ,
            EulerRot::YXY => PyEulerRot::YXY,
            EulerRot::YZY => PyEulerRot::YZY,
            EulerRot::XYX => PyEulerRot::XYX,
            EulerRot::XZX => PyEulerRot::XZX,
            EulerRot::ZYXEx => PyEulerRot::ZYXEx,
            EulerRot::ZXYEx => PyEulerRot::ZXYEx,
            EulerRot::YXZEx => PyEulerRot::YXZEx,
            EulerRot::YZXEx => PyEulerRot::YZXEx,
            EulerRot::XYZEx => PyEulerRot::XYZEx,
            EulerRot::XZYEx => PyEulerRot::XZYEx,
            EulerRot::ZYZEx => PyEulerRot::ZYZEx,
            EulerRot::ZXZEx => PyEulerRot::ZXZEx,
            EulerRot::YXYEx => PyEulerRot::YXYEx,
            EulerRot::YZYEx => PyEulerRot::YZYEx,
            EulerRot::XYXEx => PyEulerRot::XYXEx,
            EulerRot::XZXEx => PyEulerRot::XZXEx,
        }
    }
}

/// Generate forwarding methods that take `&self` and return `PyResult<Self>`.
macro_rules! impl_vec_unary {
    ($py_type:ty, [$($method:ident),* $(,)?]) => {
        #[pymethods]
        impl $py_type {
            $(
                #[inline]
                fn $method(&self) -> pyo3::PyResult<Self> { Ok(Self(self.0.$method())) }
            )*
        }
    };
}

/// Generate `#[classattr]` constants.
/// Each entry is `(snake_case_fn_name, UPPER_CONST_NAME, "PYTHON_NAME")`.
macro_rules! impl_vec_constants {
    ($py_type:ty, $inner:ty, [$(($fn_name:ident, $const_name:ident, $py_name:literal)),* $(,)?]) => {
        #[pymethods]
        impl $py_type {
            $(
                #[classattr]
                #[pyo3(name = $py_name)]
                fn $fn_name() -> Self { Self(<$inner>::$const_name) }
            )*
        }
    };
}

pub(crate) use impl_vec_constants;
pub(crate) use impl_vec_unary;

#[inline]
pub(crate) fn extract_numpy_vector<const N: usize>(
    array: PyReadonlyArray1<'_, f64>,
    type_name: &str,
) -> PyResult<[f64; N]> {
    let view = array.as_array();
    if view.shape() != [N] {
        return Err(PyValueError::new_err(format!(
            "{type_name}.from_numpy expected shape ({N},)"
        )));
    }

    let mut out = [0.0; N];
    for (dst, src) in out.iter_mut().zip(view.iter()) {
        *dst = *src;
    }
    Ok(out)
}

#[inline]
pub(crate) fn extract_numpy_matrix<const R: usize, const C: usize>(
    array: PyReadonlyArray2<'_, f64>,
    type_name: &str,
) -> PyResult<[[f64; C]; R]> {
    let view = array.as_array();
    if view.shape() != [R, C] {
        return Err(PyValueError::new_err(format!(
            "{type_name}.from_numpy expected shape ({R}, {C})"
        )));
    }

    let mut out = [[0.0; C]; R];
    for row in 0..R {
        for col in 0..C {
            out[row][col] = view[(row, col)];
        }
    }
    Ok(out)
}

#[inline]
pub(crate) fn transpose_array2<const R: usize, const C: usize>(
    matrix: [[f64; C]; R],
) -> [[f64; R]; C] {
    let mut out = [[0.0; R]; C];
    for row in 0..R {
        for col in 0..C {
            out[col][row] = matrix[row][col];
        }
    }
    out
}

#[inline]
pub(crate) fn array2_from_rows<'py, const R: usize, const C: usize>(
    py: Python<'py>,
    rows: [[f64; C]; R],
) -> Bound<'py, PyArray2<f64>> {
    let rows = rows.map(|row| row.to_vec());
    PyArray2::from_vec2(py, &rows).unwrap()
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEulerRot>()?;
    m.add_class::<PyDVec2>()?;
    m.add_class::<PyDVec3>()?;
    m.add_class::<PyDVec4>()?;
    m.add_class::<PyDQuat>()?;
    m.add_class::<PyDMat3>()?;
    m.add_class::<PyDMat4>()?;
    m.add_class::<PyDAffine3>()?;
    Ok(())
}
