pub mod glam_wrappers;
pub mod wreck_wrappers;

use pyo3::prelude::*;

#[pymodule(name = "_geomanpy")]
fn geomanpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    glam_wrappers::register(m)?;
    wreck_wrappers::register(m)?;
    Ok(())
}
