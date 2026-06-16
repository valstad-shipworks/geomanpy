//! `Cylinder` wrapper.

use wreck::Cylinder;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, skip_from_py_object, name = "Cylinder")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Cylinder")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone, Copy)]
pub struct PyCylinder(pub Cylinder);

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::{PyDMat3, PyDVec3};
    use crate::pickle::pickle_decode;
    use crate::wreck_wrappers::pyo3_glue::{dv3, v3d};
    use crate::wreck_wrappers::{PyCapsule, PyConvexPolytope, PyShape};
    use pyo3::PyResult;
    use pyo3::prelude::*;
    use wreck::Stretchable;
    use wreck::stretched::CylinderStretch;

    #[pymethods]
    impl PyCylinder {
        #[new]
        #[pyo3(signature = (p1=None, p2=None, radius=0.0, *, __pickle_state__=None))]
        fn new(
            p1: Option<PyDVec3>,
            p2: Option<PyDVec3>,
            radius: f64,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<Cylinder>(&state)?));
            }
            match (p1, p2) {
                (Some(a), Some(b)) => Ok(Self(Cylinder::new(dv3(a), dv3(b), radius as f32))),
                _ => Err(pyo3::exceptions::PyValueError::new_err(
                    "Cylinder requires p1, p2 arguments",
                )),
            }
        }
        #[getter]
        fn p1(&self) -> PyDVec3 {
            v3d(self.0.p1)
        }
        #[getter]
        fn p2(&self) -> PyDVec3 {
            v3d(self.0.p2())
        }
        #[getter]
        fn radius(&self) -> f64 {
            self.0.radius as f64
        }
        fn point_dist_sq(&self, point: PyDVec3) -> f64 {
            self.0.point_dist_sq(dv3(point)) as f64
        }
        fn contains_point(&self, point: PyDVec3) -> bool {
            self.0.contains_point(dv3(point))
        }
        #[staticmethod]
        fn from_center_orientation(
            center: PyDVec3,
            orientation: PyDMat3,
            length: f64,
            radius: f64,
        ) -> Self {
            Self(Cylinder::from_center_orientation(
                center.0,
                orientation.0,
                length,
                radius,
            ))
        }
        fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
            match self.0.stretch(dv3(translation)) {
                CylinderStretch::Aligned(c) => vec![PyShape::Cylinder(PyCylinder(c))],
                CylinderStretch::Unaligned(edges, poly) => {
                    let mut out: Vec<PyShape> = edges
                        .into_iter()
                        .map(|c| PyShape::Capsule(PyCapsule(c)))
                        .collect();
                    out.push(PyShape::ConvexPolytope(PyConvexPolytope(poly)));
                    out
                }
            }
        }
        fn __repr__(&self) -> String {
            self.0.to_string()
        }
    }

    #[pymethods]
    impl PyCylinder {
        fn length(&self) -> f64 {
            self.0.length() as f64
        }
        fn bounding_sphere(&self) -> (PyDVec3, f64) {
            let (c, r) = self.0.bounding_sphere();
            (v3d(c), r as f64)
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
    use crate::wreck_wrappers::{PyCapsule, PyConvexPolytope, PyCuboid, PySphere};
    use rustpython_vm::{
        Py, PyObjectRef, PyPayload, PyResult, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        pyclass,
        types::{Constructor, Representable},
    };
    use wreck::stretched::CylinderStretch;
    use wreck::{Scalable, Stretchable, Transformable};

    impl Constructor for PyCylinder {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<Cylinder>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            if args.args.len() != 3 {
                return Err(
                    vm.new_type_error("Cylinder(p1, p2, radius) requires 3 args".to_owned())
                );
            }
            let p1 = dv3(extract_vec3(&args.args[0], vm)?);
            let p2 = dv3(extract_vec3(&args.args[1], vm)?);
            let r: f64 = args.args[2].try_float(vm)?.to_f64();
            Ok(Self(Cylinder::new(p1, p2, r as f32)))
        }
    }
    impl Representable for PyCylinder {
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok(zelf.0.to_string())
        }
    }
    #[pyclass(with(Constructor, Representable))]
    impl PyCylinder {
        #[pygetset]
        fn p1(&self) -> PyDVec3 {
            v3d(self.0.p1)
        }
        #[pygetset]
        fn p2(&self) -> PyDVec3 {
            v3d(self.0.p2())
        }
        #[pygetset]
        fn radius(&self) -> f64 {
            self.0.radius as f64
        }
        #[pymethod]
        fn length(&self) -> f64 {
            self.0.length() as f64
        }
        #[pymethod]
        fn contains_point(&self, point: PyObjectRef, vm: &VirtualMachine) -> PyResult<bool> {
            Ok(self.0.contains_point(dv3(extract_vec3(&point, vm)?)))
        }
        #[pymethod]
        fn point_dist_sq(&self, point: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(self.0.point_dist_sq(dv3(extract_vec3(&point, vm)?)) as f64)
        }
        #[pymethod]
        fn bounding_sphere(&self) -> (PyDVec3, f64) {
            let (c, r) = self.0.bounding_sphere();
            (v3d(c), r as f64)
        }
        #[pystaticmethod]
        fn from_center_orientation(
            center: PyObjectRef,
            orientation: PyObjectRef,
            length: f64,
            radius: f64,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            let c = extract_vec3(&center, vm)?;
            let o = extract_mat3(&orientation, vm)?;
            Ok(Self(Cylinder::from_center_orientation(
                c, o, length, radius,
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
                CylinderStretch::Aligned(c) => vec![PyCylinder(c).into_pyobject(vm)],
                CylinderStretch::Unaligned(edges, poly) => {
                    let mut out: Vec<PyObjectRef> = edges
                        .into_iter()
                        .map(|c| PyCapsule(c).into_pyobject(vm))
                        .collect();
                    out.push(PyConvexPolytope(poly).into_pyobject(vm));
                    out
                }
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
                .downcast_ref::<PyCylinder>()
                .ok_or_else(|| vm.new_type_error("expected Cylinder".to_owned()))?;
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
            crate::rp_serde::dataclass_fields(&["p1", "p2", "radius"], vm)
        }
    }
}
