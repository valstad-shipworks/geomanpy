//! `Plane` wrapper.

use wreck::Plane;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, from_py_object, name = "Plane")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "_geomanpy", name = "Plane")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone, Copy)]
pub struct PyPlane(pub(crate) Plane);

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::PyDVec3;
    use crate::pickle::pickle_decode;
    use crate::wreck_wrappers::PyShape;
    use crate::wreck_wrappers::pyo3_glue::{dv3, v3d};
    use pyo3::PyResult;
    use pyo3::prelude::*;
    use wreck::Stretchable;

    #[pymethods]
    impl PyPlane {
        #[new]
        #[pyo3(signature = (normal=None, d=0.0, *, __pickle_state__=None))]
        fn new(
            normal: Option<PyDVec3>,
            d: f64,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<Plane>(&state)?));
            }
            match normal {
                Some(n) => Ok(Self(Plane::new(dv3(n), d as f32))),
                None => Err(pyo3::exceptions::PyValueError::new_err(
                    "Plane requires normal argument",
                )),
            }
        }
        #[staticmethod]
        fn from_point_normal(point: PyDVec3, normal: PyDVec3) -> Self {
            Self(Plane::from_point_normal(dv3(point), dv3(normal)))
        }
        #[getter]
        fn normal(&self) -> PyDVec3 {
            v3d(self.0.normal)
        }
        #[getter]
        fn d(&self) -> f64 {
            self.0.d as f64
        }
        fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
            vec![PyShape::Plane(PyPlane(self.0.stretch(dv3(translation))))]
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
    use wreck::{Scalable, Stretchable, Transformable};

    impl Constructor for PyPlane {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<Plane>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            if args.args.len() != 2 {
                return Err(vm.new_type_error("Plane(normal, d) requires 2 args".to_owned()));
            }
            let n = dv3(extract_vec3(&args.args[0], vm)?);
            let d: f64 = args.args[1].try_float(vm)?.to_f64();
            Ok(Self(Plane::new(n, d as f32)))
        }
    }
    impl Representable for PyPlane {
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok(zelf.0.to_string())
        }
    }
    #[pyclass(with(Constructor, Representable))]
    impl PyPlane {
        #[pygetset]
        fn normal(&self) -> PyDVec3 {
            v3d(self.0.normal)
        }
        #[pygetset]
        fn d(&self) -> f64 {
            self.0.d as f64
        }
        #[pystaticmethod]
        fn from_point_normal(
            point: PyObjectRef,
            normal: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(Plane::from_point_normal(
                dv3(extract_vec3(&point, vm)?),
                dv3(extract_vec3(&normal, vm)?),
            )))
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
            let items: Vec<PyObjectRef> = vec![PyPlane(self.0.stretch(t)).into_pyobject(vm)];
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
                .downcast_ref::<PyPlane>()
                .ok_or_else(|| vm.new_type_error("expected Plane".to_owned()))?;
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
            crate::rp_serde::dataclass_fields(&["normal", "d"], vm)
        }
    }
}
