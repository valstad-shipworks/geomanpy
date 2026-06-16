//! `Ray` wrapper.

use wreck::Ray;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, from_py_object, name = "Ray")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Ray")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone, Copy)]
pub struct PyRay(pub Ray);

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::PyDVec3;
    use crate::pickle::pickle_decode;
    use crate::wreck_wrappers::pyo3_glue::{dv3, v3d};
    use crate::wreck_wrappers::{PyConvexPolygon, PyShape};
    use pyo3::PyResult;
    use pyo3::prelude::*;
    use wreck::Stretchable;
    use wreck::stretched::RayStretch;

    #[pymethods]
    impl PyRay {
        #[new]
        #[pyo3(signature = (origin=None, dir=None, *, __pickle_state__=None))]
        fn new(
            origin: Option<PyDVec3>,
            dir: Option<PyDVec3>,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<Ray>(&state)?));
            }
            match (origin, dir) {
                (Some(o), Some(d)) => Ok(Self(Ray::new(dv3(o), dv3(d)))),
                _ => Err(pyo3::exceptions::PyValueError::new_err(
                    "Ray requires origin, dir arguments",
                )),
            }
        }
        #[getter]
        fn origin(&self) -> PyDVec3 {
            v3d(self.0.origin)
        }
        #[getter]
        fn dir(&self) -> PyDVec3 {
            v3d(self.0.dir)
        }
        fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
            match self.0.stretch(dv3(translation)) {
                RayStretch::Parallel(r) => vec![PyShape::Ray(PyRay(r))],
                RayStretch::Polygon(p) => vec![PyShape::ConvexPolygon(PyConvexPolygon(p))],
            }
        }
        fn __repr__(&self) -> String {
            self.0.to_string()
        }
    }
}

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::glam_wrappers::PyDVec3;
    use crate::glam_wrappers::quat::extract_quat;
    use crate::glam_wrappers::vec3::extract_vec3;
    use crate::wreck_wrappers::PyConvexPolygon;
    use crate::wreck_wrappers::rustpython_glue::{
        dv3, extract_affine3, extract_mat3, shape_collides, v3d,
    };
    use rustpython_vm::{
        Py, PyObjectRef, PyPayload, PyResult, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        pyclass,
        types::{Constructor, Representable},
    };
    use wreck::stretched::RayStretch;
    use wreck::{Scalable, Stretchable, Transformable};

    impl Constructor for PyRay {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<Ray>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            if args.args.len() != 2 {
                return Err(vm.new_type_error("Ray(origin, dir) requires 2 args".to_owned()));
            }
            let o = dv3(extract_vec3(&args.args[0], vm)?);
            let d = dv3(extract_vec3(&args.args[1], vm)?);
            Ok(Self(Ray::new(o, d)))
        }
    }
    impl Representable for PyRay {
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok(zelf.0.to_string())
        }
    }
    #[pyclass(with(Constructor, Representable))]
    impl PyRay {
        #[pygetset]
        fn origin(&self) -> PyDVec3 {
            v3d(self.0.origin)
        }
        #[pygetset]
        fn dir(&self) -> PyDVec3 {
            v3d(self.0.dir)
        }

        #[pymethod]
        fn scaled(&self, factor: f64) -> Self {
            Self(self.0.scaled_d(factor))
        }
        #[pymethod]
        fn translated(&self, offset: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.translated_d(extract_vec3(&offset, vm)?)))
        }
        #[pymethod]
        fn rotated_mat(&self, mat: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.rotated_mat_d(extract_mat3(&mat, vm)?)))
        }
        #[pymethod]
        fn rotated_quat(&self, quat: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.rotated_quat_d(extract_quat(&quat, vm)?)))
        }
        #[pymethod]
        fn transformed(&self, tf: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.transformed_d(extract_affine3(&tf, vm)?)))
        }

        #[pymethod]
        fn collides(&self, other: PyObjectRef, vm: &VirtualMachine) -> PyResult<bool> {
            shape_collides(&self.0, &other, vm)
        }
        #[pymethod]
        fn stretch(&self, translation: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            let t = dv3(extract_vec3(&translation, vm)?);
            let items: Vec<PyObjectRef> = match self.0.stretch(t) {
                RayStretch::Parallel(r) => vec![PyRay(r).into_pyobject(vm)],
                RayStretch::Polygon(p) => vec![PyConvexPolygon(p).into_pyobject(vm)],
            };
            Ok(vm.ctx.new_list(items).into())
        }
        #[pymethod]
        fn abs_diff_eq(
            &self,
            other: PyObjectRef,
            max_abs_diff: f64,
            vm: &VirtualMachine,
        ) -> PyResult<bool> {
            let o = other
                .downcast_ref::<PyRay>()
                .ok_or_else(|| vm.new_type_error("expected Ray".to_owned()))?;
            Ok(approx::AbsDiffEq::abs_diff_eq(
                &self.0,
                &o.0,
                max_abs_diff as f32,
            ))
        }
        #[pymethod]
        fn __getnewargs_ex__(&self, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            crate::rp_serde::getnewargs_ex(&self.0, vm)
        }
        #[pygetset]
        fn __dataclass_fields__(&self, vm: &VirtualMachine) -> PyObjectRef {
            crate::rp_serde::dataclass_fields(&["origin", "dir"], vm)
        }
    }
}
