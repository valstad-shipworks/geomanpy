//! `ConvexPolytope` wrapper.

use wreck::ConvexPolytope;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, from_py_object, name = "ConvexPolytope")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "ConvexPolytope")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone)]
pub struct PyConvexPolytope(pub(crate) ConvexPolytope);

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::PyDVec3;
    use crate::pickle::pickle_decode;
    use crate::wreck_wrappers::pyo3_glue::dv3;
    use crate::wreck_wrappers::{PyCuboid, PyShape};
    use glam::Vec3;
    use pyo3::PyResult;
    use pyo3::prelude::*;
    use wreck::Stretchable;

    #[pymethods]
    impl PyConvexPolytope {
        #[new]
        #[pyo3(signature = (planes=None, vertices=None, *, __pickle_state__=None))]
        fn new(
            planes: Option<Vec<([f64; 3], f64)>>,
            vertices: Option<Vec<[f64; 3]>>,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<ConvexPolytope>(&state)?));
            }
            match (planes, vertices) {
                (Some(planes), Some(vertices)) => {
                    let planes: Vec<(Vec3, f32)> = planes
                        .into_iter()
                        .map(|(n, d)| (Vec3::new(n[0] as f32, n[1] as f32, n[2] as f32), d as f32))
                        .collect();
                    let vertices: Vec<Vec3> = vertices
                        .into_iter()
                        .map(|v| Vec3::new(v[0] as f32, v[1] as f32, v[2] as f32))
                        .collect();
                    Ok(Self(ConvexPolytope::new(planes, vertices)))
                }
                _ => Err(pyo3::exceptions::PyValueError::new_err(
                    "ConvexPolytope requires planes, vertices arguments",
                )),
            }
        }
        #[staticmethod]
        fn with_obb(planes: Vec<([f64; 3], f64)>, vertices: Vec<[f64; 3]>, obb: PyCuboid) -> Self {
            let planes: Vec<(Vec3, f32)> = planes
                .into_iter()
                .map(|(n, d)| (Vec3::new(n[0] as f32, n[1] as f32, n[2] as f32), d as f32))
                .collect();
            let vertices: Vec<Vec3> = vertices
                .into_iter()
                .map(|v| Vec3::new(v[0] as f32, v[1] as f32, v[2] as f32))
                .collect();
            Self(ConvexPolytope::with_obb(planes, vertices, obb.0))
        }
        #[getter]
        fn planes(&self) -> Vec<([f64; 3], f64)> {
            self.0
                .planes
                .iter()
                .map(|(n, d)| ([n.x as f64, n.y as f64, n.z as f64], *d as f64))
                .collect()
        }
        #[getter]
        fn vertices(&self) -> Vec<[f64; 3]> {
            self.0
                .vertices
                .iter()
                .map(|v| [v.x as f64, v.y as f64, v.z as f64])
                .collect()
        }
        #[getter]
        fn get_obb(&self) -> PyCuboid {
            PyCuboid(self.0.obb)
        }
        fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
            vec![PyShape::ConvexPolytope(PyConvexPolytope(
                self.0.stretch(dv3(translation)),
            ))]
        }
        fn __repr__(&self) -> String {
            self.0.to_string()
        }
    }
}

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::glam_wrappers::quat::extract_quat;
    use crate::glam_wrappers::vec3::extract_vec3;
    use crate::wreck_wrappers::rustpython_glue::{
        dv3, extract_affine3, extract_mat3, shape_collides,
    };
    use crate::wreck_wrappers::{PyCuboid, PySphere};
    use rustpython_vm::{
        Py, PyObjectRef, PyPayload, PyResult, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        pyclass,
        types::{Constructor, Representable},
    };
    use wreck::{Scalable, Stretchable, Transformable};

    impl Constructor for PyConvexPolytope {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<ConvexPolytope>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            Ok(Self(ConvexPolytope::new(Vec::new(), Vec::new())))
        }
    }
    impl Representable for PyConvexPolytope {
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok(zelf.0.to_string())
        }
    }
    #[pyclass(with(Constructor, Representable))]
    impl PyConvexPolytope {
        #[pygetset]
        fn obb(&self) -> PyCuboid {
            PyCuboid(self.0.obb)
        }
        #[pymethod]
        fn vertices(&self, vm: &VirtualMachine) -> PyObjectRef {
            let items: Vec<PyObjectRef> = self
                .0
                .vertices
                .iter()
                .map(|v| {
                    vm.ctx
                        .new_tuple(vec![
                            vm.ctx.new_float(v.x as f64).into(),
                            vm.ctx.new_float(v.y as f64).into(),
                            vm.ctx.new_float(v.z as f64).into(),
                        ])
                        .into()
                })
                .collect();
            vm.ctx.new_list(items).into()
        }
        #[pymethod]
        fn planes(&self, vm: &VirtualMachine) -> PyObjectRef {
            let items: Vec<PyObjectRef> = self
                .0
                .planes
                .iter()
                .map(|(n, d)| {
                    vm.ctx
                        .new_tuple(vec![
                            vm.ctx
                                .new_tuple(vec![
                                    vm.ctx.new_float(n.x as f64).into(),
                                    vm.ctx.new_float(n.y as f64).into(),
                                    vm.ctx.new_float(n.z as f64).into(),
                                ])
                                .into(),
                            vm.ctx.new_float(*d as f64).into(),
                        ])
                        .into()
                })
                .collect();
            vm.ctx.new_list(items).into()
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
        fn broadphase(&self) -> PySphere {
            PySphere(self.0.broadphase())
        }
        #[pymethod]
        fn aabb(&self) -> PyCuboid {
            PyCuboid(self.0.aabb())
        }

        #[pymethod]
        fn collides(&self, other: PyObjectRef, vm: &VirtualMachine) -> PyResult<bool> {
            shape_collides(&self.0, &other, vm)
        }
        #[pymethod]
        fn stretch(&self, translation: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            let items: Vec<PyObjectRef> = vec![
                PyConvexPolytope(self.0.stretch(dv3(extract_vec3(&translation, vm)?)))
                    .into_pyobject(vm),
            ];
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
                .downcast_ref::<PyConvexPolytope>()
                .ok_or_else(|| vm.new_type_error("expected ConvexPolytope".to_owned()))?;
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
            crate::rp_serde::dataclass_fields(&["planes", "vertices"], vm)
        }
    }
}
