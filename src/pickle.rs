//! Helpers for pickle serialization via serde_pickle.
//!
//! All wrapped types implement `__getnewargs_ex__` to support Python's pickle
//! protocol. The returned kwargs contain a `__pickle_state__` entry with the
//! serde-pickle encoded bytes of the inner Rust value. Each `__new__` accepts
//! the same `__pickle_state__` kwarg and, when present, reconstructs the inner
//! value by decoding the bytes instead of using the normal positional args.

use pyo3::Bound;
use pyo3::PyResult;
use pyo3::Python;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyBytes, PyDict, PyDictMethods, PyTuple};

#[inline]
pub fn pickle_encode<T: serde::Serialize>(value: &T) -> PyResult<Vec<u8>> {
    serde_pickle::to_vec(value, serde_pickle::SerOptions::new())
        .map_err(|e| PyValueError::new_err(format!("pickle encode failed: {e}")))
}

#[inline]
pub fn pickle_decode<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> PyResult<T> {
    serde_pickle::from_slice(bytes, serde_pickle::DeOptions::new())
        .map_err(|e| PyValueError::new_err(format!("pickle decode failed: {e}")))
}

/// Build the `(args, kwargs)` tuple returned by `__getnewargs_ex__`:
/// empty positional args plus a `__pickle_state__` kwarg carrying the
/// serde-pickle encoded bytes of `value`.
#[inline]
pub fn make_getnewargs_ex<'py, T: serde::Serialize>(
    py: Python<'py>,
    value: &T,
) -> PyResult<(Bound<'py, PyTuple>, Bound<'py, PyDict>)> {
    let bytes = pickle_encode(value)?;
    let args = PyTuple::empty(py);
    let kwargs = PyDict::new(py);
    kwargs.set_item("__pickle_state__", PyBytes::new(py, &bytes))?;
    Ok((args, kwargs))
}

/// Generate `__getnewargs_ex__` for a wrapper type whose inner value
/// implements `serde::Serialize`. Intended for pyclasses that wrap a single
/// inner field accessed via `self.0`.
#[macro_export]
macro_rules! impl_getnewargs_ex {
    ($py_type:ty) => {
        #[pyo3::pymethods]
        impl $py_type {
            fn __getnewargs_ex__<'py>(
                &self,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<(
                pyo3::Bound<'py, pyo3::types::PyTuple>,
                pyo3::Bound<'py, pyo3::types::PyDict>,
            )> {
                $crate::pickle::make_getnewargs_ex(py, &self.0)
            }
        }
    };
}
