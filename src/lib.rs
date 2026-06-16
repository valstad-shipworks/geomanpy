// Rustpython class constants are exposed as classmethods whose Rust name
// matches their Python name (UPPER_CASE) — silence the snake_case lint for
// those, scoped to this crate. Standard Rust methods are still checked.
#![cfg_attr(feature = "rustpython-backend", allow(non_snake_case))]
// `to_*` wrappers must take `&self` to be callable from Python, and the
// extract/return signatures of the binding glue are inherently verbose.
#![allow(clippy::wrong_self_convention, clippy::type_complexity)]

//! geomanpy — Python bindings for `glam` and `wreck`.
//!
//! Backend is selected by feature flag:
//! - `pyo3-backend` (default): produces a CPython extension via PyO3.
//! - `rustpython-backend`: registers a `geomanpy` module inside an embedded
//!   RustPython VM. The original wrapper structs (`PyDVec3`, `PySphere`, …)
//!   are reused — they're decorated with both pyclass macros via `cfg_attr`
//!   and carry separate impl blocks for each backend's method conventions.
//!
//! Exactly one backend must be active. The optional `safe-locks` feature
//! switches mutable wrappers from `UnsafeCell` to rustpython's `PyRwLock`
//! (no effect on the immutable Copy wrappers).

// Mutual-exclusion guard.
#[cfg(all(feature = "pyo3-backend", feature = "rustpython-backend"))]
compile_error!(
    "features `pyo3-backend` and `rustpython-backend` are mutually exclusive — \
     pick exactly one (default is `pyo3-backend`)."
);
#[cfg(all(
    feature = "not_build_only",
    not(any(feature = "pyo3-backend", feature = "rustpython-backend"))
))]
compile_error!(
    "no Python backend selected — enable either `pyo3-backend` (default) \
     or `rustpython-backend`."
);

// Backend-agnostic constants — embedded source of the Python facade.
pub const PYI_CONTENTS: &str = include_str!("../py_src/geomanpy/__init__.pyi");
pub const PY_MODULE_CONTENTS: &str = include_str!("../py_src/geomanpy/__init__.py");

// Shared modules — compile under either backend. Each contains struct
// definitions plus cfg-gated impl blocks for the active backend.
#[cfg(feature = "not_build_only")]
pub mod glam_wrappers;
#[cfg(feature = "not_build_only")]
pub mod wreck_wrappers;

// Serialization helpers. `pickle` exposes backend-agnostic serde-pickle
// encode/decode plus pyo3-gated `__getnewargs_ex__` glue; `dataclass` is
// pyo3-only; `rp_serde` is the rustpython counterpart.
#[cfg(all(feature = "not_build_only", feature = "pyo3-backend"))]
pub mod dataclass;
#[cfg(feature = "not_build_only")]
pub mod pickle;
#[cfg(all(feature = "not_build_only", feature = "rustpython-backend"))]
pub mod rp_serde;

// PyO3 entry point.
#[cfg(all(feature = "not_build_only", feature = "pyo3-backend"))]
use pyo3::prelude::*;

#[cfg(all(feature = "not_build_only", feature = "pyo3-backend"))]
#[pymodule(name = "_geomanpy")]
fn geomanpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    glam_wrappers::register(m)?;
    wreck_wrappers::register(m)?;
    Ok(())
}

// RustPython entry point.
#[cfg(all(feature = "not_build_only", feature = "rustpython-backend"))]
pub mod rustpython_bindings;

/// Return the `geomanpy` module definition for embedding in a
/// [`rustpython_vm::Interpreter`].
#[cfg(all(feature = "not_build_only", feature = "rustpython-backend"))]
pub use rustpython_bindings::make_module;
