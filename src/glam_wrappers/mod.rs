//! Glam-type Python wrappers.
//!
//! Each struct (`PyDVec3`, `PyDMat4`, …) is decorated with both backends'
//! pyclass macros via `cfg_attr`, so the same Rust definition serves the
//! active backend. Method impls are cfg-gated per backend.

pub(crate) mod affine3;
pub(crate) mod mat3;
pub(crate) mod mat4;
pub(crate) mod quat;
pub(crate) mod vec2;
pub(crate) mod vec3;
pub(crate) mod vec4;

pub use affine3::PyDAffine3;
pub use mat3::PyDMat3;
pub use mat4::PyDMat4;
pub use quat::PyDQuat;
pub use vec2::PyDVec2;
pub use vec3::PyDVec3;
pub use vec4::PyDVec4;

use glam::EulerRot;

#[cfg(feature = "pyo3-backend")]
use numpy::{AllowTypeChange, PyArray2, PyArrayLike1, PyArrayLike2};
#[cfg(feature = "pyo3-backend")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "pyo3-backend")]
use pyo3::prelude::*;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(eq, eq_int, from_py_object, name = "EulerRot")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "_geomanpy", name = "EulerRot")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
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

#[cfg(feature = "rustpython-backend")]
#[rustpython_vm::pyclass]
impl PyEulerRot {}

/// Expose the rotation-order variants as class attributes (`EulerRot.ZYX`, …).
/// RustPython has no enum-pyclass macro, so they're installed onto the static
/// type post-creation, like the vector/matrix constants.
#[cfg(feature = "rustpython-backend")]
pub(crate) fn install_euler_constants(
    typ: &rustpython_vm::builtins::PyTypeRef,
    vm: &rustpython_vm::VirtualMachine,
) {
    use rustpython_vm::PyPayload;
    let set = |name: &str, v: PyEulerRot| {
        typ.set_attr(vm.ctx.intern_str(name), v.into_pyobject(vm));
    };
    set("ZYX", PyEulerRot::ZYX);
    set("ZXY", PyEulerRot::ZXY);
    set("YXZ", PyEulerRot::YXZ);
    set("YZX", PyEulerRot::YZX);
    set("XYZ", PyEulerRot::XYZ);
    set("XZY", PyEulerRot::XZY);
    set("ZYZ", PyEulerRot::ZYZ);
    set("ZXZ", PyEulerRot::ZXZ);
    set("YXY", PyEulerRot::YXY);
    set("YZY", PyEulerRot::YZY);
    set("XYX", PyEulerRot::XYX);
    set("XZX", PyEulerRot::XZX);
    set("ZYXEx", PyEulerRot::ZYXEx);
    set("ZXYEx", PyEulerRot::ZXYEx);
    set("YXZEx", PyEulerRot::YXZEx);
    set("YZXEx", PyEulerRot::YZXEx);
    set("XYZEx", PyEulerRot::XYZEx);
    set("XZYEx", PyEulerRot::XZYEx);
    set("ZYZEx", PyEulerRot::ZYZEx);
    set("ZXZEx", PyEulerRot::ZXZEx);
    set("YXYEx", PyEulerRot::YXYEx);
    set("YZYEx", PyEulerRot::YZYEx);
    set("XYXEx", PyEulerRot::XYXEx);
    set("XZXEx", PyEulerRot::XZXEx);
}

// =============================================================================
// pyo3-backend: macros + numpy helpers + register()
// =============================================================================

#[cfg(feature = "pyo3-backend")]
mod pyo3_glue {
    use super::*;

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

    /// Implement serde methods for a wrapper type whose inner type implements
    /// `serde::{Serialize, Deserialize}`.
    macro_rules! impl_serde_methods {
        ($py_type:ty, $inner:ty) => {
            #[pymethods]
            impl $py_type {
                fn to_json(&self) -> pyo3::PyResult<String> {
                    serde_json::to_string(&self.0)
                        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
                }

                #[staticmethod]
                fn from_json(json: &str) -> pyo3::PyResult<Self> {
                    let inner: $inner = serde_json::from_str(json)
                        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
                    Ok(Self(inner))
                }

                #[staticmethod]
                fn try_from_json(json: &str) -> Option<Self> {
                    serde_json::from_str::<$inner>(json).ok().map(Self)
                }

                fn to_dict<'py>(
                    &self,
                    py: pyo3::Python<'py>,
                ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::types::PyAny>> {
                    pythonize::pythonize(py, &self.0)
                        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
                }

                #[staticmethod]
                fn from_dict(obj: &pyo3::Bound<'_, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
                    let inner: $inner = pythonize::depythonize(obj)
                        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
                    Ok(Self(inner))
                }

                #[staticmethod]
                fn try_from_dict(obj: &pyo3::Bound<'_, pyo3::types::PyAny>) -> Option<Self> {
                    pythonize::depythonize::<$inner>(obj).ok().map(Self)
                }
            }
        };
    }

    pub(crate) use impl_serde_methods;
    pub(crate) use impl_vec_constants;
    pub(crate) use impl_vec_unary;

    #[inline]
    pub(crate) fn extract_numpy_vector<const N: usize>(
        array: PyArrayLike1<'_, f64, AllowTypeChange>,
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
        array: PyArrayLike2<'_, f64, AllowTypeChange>,
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
}

// Re-export pyo3 helpers at module scope so wrapper files can `use super::*;`
// as before (no churn in vec3.rs, mat3.rs, etc.).
#[cfg(feature = "pyo3-backend")]
pub(crate) use pyo3_glue::{
    array2_from_rows, extract_numpy_matrix, extract_numpy_vector, impl_serde_methods,
    impl_vec_constants, impl_vec_unary, transpose_array2,
};

// =============================================================================
// rustpython-backend: numpy interop helpers via the `rumpy` crate
// =============================================================================

#[cfg(feature = "rustpython-backend")]
pub(crate) mod rustpython_numpy {
    use ndarray::{ArrayD, IxDyn};
    use rumpy::{PyNdArray, convert::obj_to_typed, dtype::ArraysD};
    use rustpython_vm::{PyObjectRef, PyPayload, PyResult, VirtualMachine};

    /// Coerce any python object (list/tuple/ndarray/scalar) into a fixed-length
    /// `[f64; N]`. Mirrors `extract_numpy_vector` on the pyo3 side.
    #[inline]
    pub(crate) fn extract_numpy_vector<const N: usize>(
        obj: &PyObjectRef,
        type_name: &str,
        vm: &VirtualMachine,
    ) -> PyResult<[f64; N]> {
        let arr: ArrayD<f64> = obj_to_typed::<f64>(obj, vm)?;
        if arr.shape() != [N] {
            return Err(vm.new_value_error(format!("{type_name}.from_numpy expected shape ({N},)")));
        }
        let mut out = [0.0; N];
        for (dst, src) in out.iter_mut().zip(arr.iter()) {
            *dst = *src;
        }
        Ok(out)
    }

    /// Coerce any python object into a fixed-shape `[[f64; C]; R]`. Mirrors
    /// `extract_numpy_matrix` on the pyo3 side.
    #[inline]
    pub(crate) fn extract_numpy_matrix<const R: usize, const C: usize>(
        obj: &PyObjectRef,
        type_name: &str,
        vm: &VirtualMachine,
    ) -> PyResult<[[f64; C]; R]> {
        let arr: ArrayD<f64> = obj_to_typed::<f64>(obj, vm)?;
        if arr.shape() != [R, C] {
            return Err(
                vm.new_value_error(format!("{type_name}.from_numpy expected shape ({R}, {C})"))
            );
        }
        let mut out = [[0.0; C]; R];
        for r in 0..R {
            for c in 0..C {
                out[r][c] = arr[[r, c]];
            }
        }
        Ok(out)
    }

    /// Transpose a fixed-shape array — used to swap between glam's column-major
    /// `to_cols_array_2d()` and numpy's row-major view. Mirrors
    /// `transpose_array2` on the pyo3 side.
    #[inline]
    pub(crate) fn transpose_array2<const R: usize, const C: usize>(
        matrix: [[f64; C]; R],
    ) -> [[f64; R]; C] {
        let mut out = [[0.0; R]; C];
        for r in 0..R {
            for c in 0..C {
                out[c][r] = matrix[r][c];
            }
        }
        out
    }

    /// Build a 1-D `numpy.ndarray` (rumpy `PyNdArray`) from a slice of f64.
    #[inline]
    pub(crate) fn pyndarray_from_slice(values: &[f64], vm: &VirtualMachine) -> PyObjectRef {
        let arr = ArrayD::from_shape_vec(IxDyn(&[values.len()]), values.to_vec())
            .expect("shape matches data length");
        PyNdArray::from_arrays(ArraysD::F64(arr)).into_pyobject(vm)
    }

    /// Build a 2-D `numpy.ndarray` (rumpy `PyNdArray`) from a row-major fixed
    /// matrix. Mirrors `array2_from_rows` on the pyo3 side.
    #[inline]
    pub(crate) fn pyndarray_from_rows<const R: usize, const C: usize>(
        rows: [[f64; C]; R],
        vm: &VirtualMachine,
    ) -> PyObjectRef {
        let mut flat = Vec::with_capacity(R * C);
        for row in rows {
            for val in row {
                flat.push(val);
            }
        }
        let arr = ArrayD::from_shape_vec(IxDyn(&[R, C]), flat).expect("shape matches data length");
        PyNdArray::from_arrays(ArraysD::F64(arr)).into_pyobject(vm)
    }
}

#[cfg(feature = "rustpython-backend")]
pub(crate) use rustpython_numpy::{
    extract_numpy_matrix as extract_numpy_matrix_rp,
    extract_numpy_vector as extract_numpy_vector_rp, pyndarray_from_rows, pyndarray_from_slice,
    transpose_array2 as transpose_array2_rp,
};

// =============================================================================
// rustpython-backend: operator / comparison / hash / sequence slot helpers
// =============================================================================

#[cfg(feature = "rustpython-backend")]
pub(crate) mod rp_ops {
    use rustpython_vm::{
        PyObject, PyObjectRef, PyPayload, PyResult, TryFromObject, VirtualMachine,
        protocol::PyNumber, types::AsNumber,
    };

    /// A glam vector wrapper that supports scalar-broadcast arithmetic.
    pub trait RpVec: PyPayload + std::fmt::Debug + Sized {
        type Inner: Copy;
        fn inner(&self) -> Self::Inner;
        fn wrap(i: Self::Inner) -> Self;
        fn splat(s: f64) -> Self::Inner;
    }

    /// Number-protocol binary slot: coerce both operands (wrapper or scalar)
    /// and apply `op`, or return `NotImplemented`.
    pub fn binop<P: RpVec>(
        a: &PyObject,
        b: &PyObject,
        vm: &VirtualMachine,
        op: fn(P::Inner, P::Inner) -> P::Inner,
    ) -> PyResult<PyObjectRef> {
        let coerce = |o: &PyObject| -> Option<P::Inner> {
            if let Some(v) = o.downcast_ref::<P>() {
                Some(v.inner())
            } else {
                f64::try_from_object(vm, o.to_owned()).ok().map(P::splat)
            }
        };
        match (coerce(a), coerce(b)) {
            (Some(x), Some(y)) => Ok(P::wrap(op(x, y)).into_pyobject(vm)),
            _ => Ok(vm.ctx.not_implemented()),
        }
    }

    /// Number-protocol unary slot.
    pub fn unaryop<P: RpVec + AsNumber>(
        num: PyNumber<'_>,
        vm: &VirtualMachine,
        op: fn(P::Inner) -> P::Inner,
    ) -> PyResult<PyObjectRef> {
        let z = P::number_downcast(num);
        Ok(P::wrap(op(z.inner())).into_pyobject(vm))
    }
}

/// Generate `AsNumber` (`+ - * / %`, unary `- +`), `Comparable` (`== !=`),
/// `Hashable`, and `AsMapping` (`len`, `[]`) for a glam vector wrapper whose
/// inner type exposes `splat`, `to_array`, and the arithmetic operators.
#[cfg(feature = "rustpython-backend")]
macro_rules! impl_rp_vec_ops {
    ($py:ty, $inner:ty, $n:literal) => {
        impl $crate::glam_wrappers::rp_ops::RpVec for $py {
            type Inner = $inner;
            #[inline]
            fn inner(&self) -> $inner {
                self.0
            }
            #[inline]
            fn wrap(i: $inner) -> Self {
                Self(i)
            }
            #[inline]
            fn splat(s: f64) -> $inner {
                <$inner>::splat(s)
            }
        }

        impl rustpython_vm::types::AsNumber for $py {
            fn as_number() -> &'static rustpython_vm::protocol::PyNumberMethods {
                static N: rustpython_vm::protocol::PyNumberMethods =
                    rustpython_vm::protocol::PyNumberMethods {
                        add: Some(|a, b, vm| {
                            $crate::glam_wrappers::rp_ops::binop::<$py>(a, b, vm, |x, y| x + y)
                        }),
                        subtract: Some(|a, b, vm| {
                            $crate::glam_wrappers::rp_ops::binop::<$py>(a, b, vm, |x, y| x - y)
                        }),
                        multiply: Some(|a, b, vm| {
                            $crate::glam_wrappers::rp_ops::binop::<$py>(a, b, vm, |x, y| x * y)
                        }),
                        true_divide: Some(|a, b, vm| {
                            $crate::glam_wrappers::rp_ops::binop::<$py>(a, b, vm, |x, y| x / y)
                        }),
                        remainder: Some(|a, b, vm| {
                            $crate::glam_wrappers::rp_ops::binop::<$py>(a, b, vm, |x, y| x % y)
                        }),
                        negative: Some(|num, vm| {
                            $crate::glam_wrappers::rp_ops::unaryop::<$py>(num, vm, |x| -x)
                        }),
                        positive: Some(|num, vm| {
                            $crate::glam_wrappers::rp_ops::unaryop::<$py>(num, vm, |x| x)
                        }),
                        ..rustpython_vm::protocol::PyNumberMethods::NOT_IMPLEMENTED
                    };
                &N
            }
        }

        impl rustpython_vm::types::Comparable for $py {
            fn cmp(
                zelf: &rustpython_vm::Py<Self>,
                other: &rustpython_vm::PyObject,
                op: rustpython_vm::types::PyComparisonOp,
                _vm: &rustpython_vm::VirtualMachine,
            ) -> rustpython_vm::PyResult<rustpython_vm::function::PyComparisonValue> {
                op.eq_only(|| match other.downcast_ref::<$py>() {
                    Some(o) => Ok(rustpython_vm::function::PyComparisonValue::Implemented(
                        zelf.0 == o.0,
                    )),
                    None => Ok(rustpython_vm::function::PyComparisonValue::NotImplemented),
                })
            }
        }

        impl rustpython_vm::types::Hashable for $py {
            fn hash(
                zelf: &rustpython_vm::Py<Self>,
                _vm: &rustpython_vm::VirtualMachine,
            ) -> rustpython_vm::PyResult<rustpython_vm::common::hash::PyHash> {
                use std::hash::{Hash, Hasher};
                let mut h = std::collections::hash_map::DefaultHasher::new();
                for c in zelf.0.to_array() {
                    c.to_bits().hash(&mut h);
                }
                Ok(h.finish() as rustpython_vm::common::hash::PyHash)
            }
        }

        impl rustpython_vm::types::AsMapping for $py {
            fn as_mapping() -> &'static rustpython_vm::protocol::PyMappingMethods {
                static M: rustpython_vm::protocol::PyMappingMethods =
                    rustpython_vm::protocol::PyMappingMethods {
                        length: Some(|_m, _vm| Ok($n)),
                        subscript: Some(|m, needle, vm| {
                            let z = <$py as rustpython_vm::types::AsMapping>::mapping_downcast(m);
                            let arr = z.0.to_array();
                            let i = <isize as rustpython_vm::TryFromObject>::try_from_object(
                                vm,
                                needle.to_owned(),
                            )?;
                            let n = arr.len() as isize;
                            let idx = if i < 0 { i + n } else { i };
                            if idx < 0 || idx >= n {
                                return Err(vm.new_index_error("index out of range".to_owned()));
                            }
                            Ok(vm.ctx.new_float(arr[idx as usize]).into())
                        }),
                        ..rustpython_vm::protocol::PyMappingMethods::NOT_IMPLEMENTED
                    };
                &M
            }
        }
    };
}

#[cfg(feature = "rustpython-backend")]
pub(crate) use impl_rp_vec_ops;

#[cfg(feature = "pyo3-backend")]
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
