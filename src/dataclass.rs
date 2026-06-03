//! Helpers for exposing `__dataclass_fields__` on wrapper types.
//!
//! Frameworks like `orjson`, `msgspec`, and `pydantic` detect a class as a
//! dataclass by checking for `__dataclass_fields__`. This module builds a real
//! `dict[str, dataclasses.Field]` via `dataclasses.make_dataclass`, which each
//! pyclass exposes as a `#[classattr]` so it appears on the class.

use pyo3::Bound;
use pyo3::PyResult;
use pyo3::Python;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString};

/// Build a `__dataclass_fields__` dict for a type with the given field names
/// by constructing an ad-hoc dataclass and stealing its `__dataclass_fields__`.
/// Each field is typed as `typing.Any` with no default.
pub fn make_dataclass_fields<'py>(
    py: Python<'py>,
    names: &[&str],
) -> PyResult<Bound<'py, PyDict>> {
    let dataclasses = py.import("dataclasses")?;
    let make_dc = dataclasses.getattr("make_dataclass")?;

    let fields_list = PyList::empty(py);
    for name in names {
        fields_list.append(PyString::new(py, name))?;
    }

    let dummy = make_dc.call1(("_GeomanpyDataclassShape", fields_list))?;
    let fields = dummy.getattr("__dataclass_fields__")?.cast_into::<PyDict>()
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!(
            "make_dataclass did not expose __dataclass_fields__: {e}"
        )))?;
    Ok(fields)
}

#[macro_export]
macro_rules! impl_dataclass_fields {
    ($py_type:ty, [$($name:literal),* $(,)?]) => {
        #[pyo3::pymethods]
        impl $py_type {
            #[classattr]
            #[allow(non_snake_case)]
            fn __dataclass_fields__(
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
                $crate::dataclass::make_dataclass_fields(py, &[$($name),*])
                    .map(|d| d.unbind())
            }

            // Populate `__dict__` with live field values so serializers that
            // iterate the instance dict (e.g. orjson's dataclass fast path)
            // see stable references held by the returned dict, rather than
            // fresh single-refcount objects returned by our getters.
            #[getter]
            #[allow(non_snake_case, unused_imports)]
            fn __dict__<'py>(
                slf: &pyo3::Bound<'py, Self>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::types::PyDict>> {
                use pyo3::types::PyAnyMethods;
                let py = slf.py();
                let d = pyo3::types::PyDict::new(py);
                $(
                    let key = pyo3::intern!(py, $name);
                    let val = slf.as_any().getattr(key)?;
                    d.set_item(key, val)?;
                )*
                Ok(d)
            }
        }
    };
}
