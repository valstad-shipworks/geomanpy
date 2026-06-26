use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::ffi::CString;

const PY_FILE: &str = "tests/glam.py";

fn main() {
    Python::attach(|py| {
        let code = std::fs::read_to_string(PY_FILE).unwrap();
        let code = CString::new(code).unwrap();
        let module = PyModule::from_code(py, &code, c_str!("glam.py"), c_str!("glam")).unwrap();
        geomanpy::glam_wrappers::register(&module).unwrap();
        geomanpy::wreck_wrappers::register(&module).unwrap();
        geomanpy::squiggle_wrappers::register(&module).unwrap();
        py.run(c_str!("main()"), None, Some(&module.dict()))
            .unwrap();
    })
}
