//! `ConvexPolygon` wrapper.

use wreck::ConvexPolygon;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, from_py_object, name = "ConvexPolygon")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "ConvexPolygon")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone)]
pub struct PyConvexPolygon(pub(crate) ConvexPolygon);

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::PyDVec3;
    use crate::pickle::pickle_decode;
    use crate::wreck_wrappers::pyo3_glue::{dv3, v3d};
    use crate::wreck_wrappers::{PyConvexPolytope, PyShape};
    use pyo3::PyResult;
    use pyo3::prelude::*;
    use wreck::Stretchable;
    use wreck::stretched::ConvexPolygonStretch;

    #[pymethods]
    impl PyConvexPolygon {
        #[new]
        #[pyo3(signature = (center=None, normal=None, vertices_2d=None, *, __pickle_state__=None))]
        fn new(
            center: Option<PyDVec3>,
            normal: Option<PyDVec3>,
            vertices_2d: Option<Vec<[f64; 2]>>,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<ConvexPolygon>(&state)?));
            }
            match (center, normal, vertices_2d) {
                (Some(center), Some(normal), Some(vertices_2d)) => {
                    let verts: Vec<[f32; 2]> = vertices_2d
                        .into_iter()
                        .map(|v| [v[0] as f32, v[1] as f32])
                        .collect();
                    Ok(Self(ConvexPolygon::new(dv3(center), dv3(normal), verts)))
                }
                _ => Err(pyo3::exceptions::PyValueError::new_err(
                    "ConvexPolygon requires center, normal, vertices_2d arguments",
                )),
            }
        }
        #[staticmethod]
        fn with_axes(
            center: PyDVec3,
            normal: PyDVec3,
            u_axis: PyDVec3,
            v_axis: PyDVec3,
            vertices_2d: Vec<[f64; 2]>,
        ) -> Self {
            let verts: Vec<[f32; 2]> = vertices_2d
                .into_iter()
                .map(|v| [v[0] as f32, v[1] as f32])
                .collect();
            Self(ConvexPolygon::with_axes(
                dv3(center),
                dv3(normal),
                dv3(u_axis),
                dv3(v_axis),
                verts,
            ))
        }
        #[getter]
        fn center(&self) -> PyDVec3 {
            v3d(self.0.center)
        }
        #[getter]
        fn normal(&self) -> PyDVec3 {
            v3d(self.0.normal)
        }
        #[getter]
        fn u_axis(&self) -> PyDVec3 {
            v3d(self.0.u_axis)
        }
        #[getter]
        fn v_axis(&self) -> PyDVec3 {
            v3d(self.0.v_axis)
        }
        #[getter]
        fn vertices_2d(&self) -> Vec<[f64; 2]> {
            self.0
                .vertices_2d
                .iter()
                .map(|v| [v[0] as f64, v[1] as f64])
                .collect()
        }
        fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
            match self.0.stretch(dv3(translation)) {
                ConvexPolygonStretch::InPlane(p) => {
                    vec![PyShape::ConvexPolygon(PyConvexPolygon(p))]
                }
                ConvexPolygonStretch::OutOfPlane(p) => {
                    vec![PyShape::ConvexPolytope(PyConvexPolytope(p))]
                }
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
    use crate::wreck_wrappers::rustpython_glue::{
        dv3, extract_affine3, extract_mat3, shape_collides, v3d,
    };
    use crate::wreck_wrappers::{PyConvexPolytope, PyCuboid, PySphere};
    use glam::Vec3;
    use rustpython_vm::{
        Py, PyObjectRef, PyPayload, PyResult, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        pyclass,
        types::{Constructor, Representable},
    };
    use wreck::stretched::ConvexPolygonStretch;
    use wreck::{Scalable, Stretchable, Transformable};

    impl Constructor for PyConvexPolygon {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<ConvexPolygon>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            Ok(Self(ConvexPolygon::new(Vec3::ZERO, Vec3::Z, Vec::new())))
        }
    }
    impl Representable for PyConvexPolygon {
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok(zelf.0.to_string())
        }
    }
    #[pyclass(with(Constructor, Representable))]
    impl PyConvexPolygon {
        #[pygetset]
        fn center(&self) -> PyDVec3 {
            v3d(self.0.center)
        }
        #[pygetset]
        fn normal(&self) -> PyDVec3 {
            v3d(self.0.normal)
        }
        #[pygetset]
        fn u_axis(&self) -> PyDVec3 {
            v3d(self.0.u_axis)
        }
        #[pygetset]
        fn v_axis(&self) -> PyDVec3 {
            v3d(self.0.v_axis)
        }
        #[pymethod]
        fn vertices_2d(&self, vm: &VirtualMachine) -> PyObjectRef {
            let items: Vec<PyObjectRef> = self
                .0
                .vertices_2d
                .iter()
                .map(|v| {
                    vm.ctx
                        .new_tuple(vec![
                            vm.ctx.new_float(v[0] as f64).into(),
                            vm.ctx.new_float(v[1] as f64).into(),
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
        fn obb(&self) -> PyCuboid {
            PyCuboid(self.0.obb())
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
            let t = dv3(extract_vec3(&translation, vm)?);
            let items: Vec<PyObjectRef> = match self.0.stretch(t) {
                ConvexPolygonStretch::InPlane(p) => vec![PyConvexPolygon(p).into_pyobject(vm)],
                ConvexPolygonStretch::OutOfPlane(p) => vec![PyConvexPolytope(p).into_pyobject(vm)],
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
                .downcast_ref::<PyConvexPolygon>()
                .ok_or_else(|| vm.new_type_error("expected ConvexPolygon".to_owned()))?;
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
            crate::rp_serde::dataclass_fields(
                &["center", "normal", "u_axis", "v_axis", "vertices_2d"],
                vm,
            )
        }
    }
}
