//! `Sphere` wrapper.

use wreck::Sphere;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, skip_from_py_object, name = "Sphere")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Sphere")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone, Copy)]
pub struct PySphere(pub Sphere);

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::PyDVec3;
    use crate::pickle::pickle_decode;
    use crate::wreck_wrappers::pyo3_glue::{dv3, v3d};
    use crate::wreck_wrappers::{PyCapsule, PyShape};
    use pyo3::PyResult;
    use pyo3::prelude::*;
    use wreck::Stretchable;
    use wreck::stretched::SphereStretch;

    #[pymethods]
    impl PySphere {
        #[new]
        #[pyo3(signature = (center=None, radius=0.0, *, __pickle_state__=None))]
        fn new(
            center: Option<PyDVec3>,
            radius: f64,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<Sphere>(&state)?));
            }
            match center {
                Some(c) => Ok(Self(Sphere::new_d(c.0, radius))),
                None => Err(pyo3::exceptions::PyValueError::new_err(
                    "Sphere requires center argument",
                )),
            }
        }
        #[getter]
        fn center(&self) -> PyDVec3 {
            v3d(self.0.center)
        }
        #[getter]
        fn radius(&self) -> f64 {
            self.0.radius as f64
        }
        fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
            match self.0.stretch(dv3(translation)) {
                SphereStretch::NoStretch(s) => vec![PyShape::Sphere(PySphere(s))],
                SphereStretch::Stretch(c) => vec![PyShape::Capsule(PyCapsule(c))],
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
    use crate::wreck_wrappers::{PyCapsule, PyCuboid};
    use glam::Vec3;
    use rustpython_vm::{
        Py, PyObjectRef, PyPayload, PyResult, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        pyclass,
        types::{Constructor, Representable},
    };
    use wreck::stretched::SphereStretch;
    use wreck::{Scalable, Stretchable, Transformable};

    impl Constructor for PySphere {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<Sphere>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            if args.args.is_empty() {
                return Ok(Self(Sphere::new(Vec3::ZERO, 0.0)));
            }
            if args.args.len() == 2 {
                let center = extract_vec3(&args.args[0], vm)?;
                let radius: f64 = args.args[1].try_float(vm)?.to_f64();
                return Ok(Self(Sphere::new(dv3(center), radius as f32)));
            }
            Err(vm.new_type_error("Sphere() takes 0 or 2 positional arguments".to_owned()))
        }
    }
    impl Representable for PySphere {
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok(zelf.0.to_string())
        }
    }
    #[pyclass(with(Constructor, Representable))]
    impl PySphere {
        #[pygetset]
        fn center(&self) -> PyDVec3 {
            v3d(self.0.center)
        }
        #[pygetset]
        fn radius(&self) -> f64 {
            self.0.radius as f64
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
                SphereStretch::NoStretch(s) => vec![PySphere(s).into_pyobject(vm)],
                SphereStretch::Stretch(c) => vec![PyCapsule(c).into_pyobject(vm)],
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
                .downcast_ref::<PySphere>()
                .ok_or_else(|| vm.new_type_error("expected Sphere".to_owned()))?;
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
            crate::rp_serde::dataclass_fields(&["center", "radius"], vm)
        }
    }
}
