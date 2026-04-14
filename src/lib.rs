#[cfg(feature = "not_build_only")]
pub mod geom_api;
#[cfg(feature = "not_build_only")]
pub mod glam_wrappers;
#[cfg(feature = "not_build_only")]
pub mod wreck_wrappers;

#[cfg(feature = "not_build_only")]
use pyo3::prelude::*;

pub const PYI_CONTENTS: &str = include_str!("../py_src/geomanpy/__init__.pyi");
pub const PY_MODULE_CONTENTS: &str = include_str!("../py_src/geomanpy/__init__.py");

#[cfg(feature = "not_build_only")]
#[pymodule(name = "_geomanpy")]
fn geomanpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    glam_wrappers::register(m)?;
    wreck_wrappers::register(m)?;
    geom_api::register(m)?;
    Ok(())
}
