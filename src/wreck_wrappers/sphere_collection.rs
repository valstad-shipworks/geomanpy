//! `SphereCollection` wrapper (structure-of-arrays sphere storage).

use wreck::soa::SpheresSoA;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(from_py_object, name = "SphereCollection")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "_geomanpy", name = "SphereCollection")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone)]
pub struct PySphereCollection(pub(crate) SpheresSoA);

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::pickle::pickle_decode;
    use crate::wreck_wrappers::PySphere;
    use pyo3::PyResult;
    use pyo3::prelude::*;
    use wreck::Sphere;

    #[pymethods]
    impl PySphereCollection {
        #[new]
        #[pyo3(signature = (*, __pickle_state__=None))]
        fn new(__pickle_state__: Option<Vec<u8>>) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<SpheresSoA>(&state)?));
            }
            Ok(Self(SpheresSoA::new()))
        }
        #[staticmethod]
        fn with_capacity(cap: usize) -> Self {
            Self(SpheresSoA::with_capacity(cap))
        }
        #[staticmethod]
        fn from_slice(spheres: Vec<PySphere>) -> Self {
            let inner: Vec<Sphere> = spheres.into_iter().map(|s| s.0).collect();
            Self(SpheresSoA::from_slice(&inner))
        }
        fn len(&self) -> usize {
            self.0.len()
        }
        fn __getitem__(&self, index: isize) -> PyResult<PySphere> {
            let n = self.0.len() as isize;
            let idx = if index < 0 { index + n } else { index };
            if idx < 0 || idx >= n {
                return Err(pyo3::exceptions::PyIndexError::new_err(
                    "SphereCollection index out of range",
                ));
            }
            Ok(PySphere(self.0.get(idx as usize)))
        }
        fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        fn push(&mut self, sphere: PySphere) {
            self.0.push(sphere.0);
        }
        fn get(&self, index: usize) -> PySphere {
            PySphere(self.0.get(index))
        }
        fn clear(&mut self) {
            self.0.clear();
        }
        fn any_collides_sphere(&self, sphere: &PySphere) -> bool {
            self.0.any_collides_sphere(&sphere.0)
        }
        fn __len__(&self) -> usize {
            self.0.len()
        }
        fn __repr__(&self) -> String {
            format!("SphereCollection(len={})", self.0.len())
        }
    }
}

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::wreck_wrappers::PySphere;
    use rustpython_vm::{
        Py, PyObjectRef, PyResult, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        protocol::PyMappingMethods,
        pyclass,
        types::{AsMapping, Constructor, Representable},
    };
    use wreck::Sphere;

    impl Constructor for PySphereCollection {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<SpheresSoA>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            Ok(Self(SpheresSoA::new()))
        }
    }
    impl Representable for PySphereCollection {
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok(format!("SphereCollection(len={})", zelf.0.len()))
        }
    }
    impl AsMapping for PySphereCollection {
        fn as_mapping() -> &'static PyMappingMethods {
            static M: PyMappingMethods = PyMappingMethods {
                length: Some(|m, _vm| Ok(PySphereCollection::mapping_downcast(m).0.len())),
                subscript: Some(|m, needle, vm| {
                    use rustpython_vm::PyPayload;
                    let z = PySphereCollection::mapping_downcast(m);
                    let n = z.0.len() as isize;
                    let i = <isize as rustpython_vm::TryFromObject>::try_from_object(
                        vm,
                        needle.to_owned(),
                    )?;
                    let idx = if i < 0 { i + n } else { i };
                    if idx < 0 || idx >= n {
                        return Err(
                            vm.new_index_error("SphereCollection index out of range".to_owned())
                        );
                    }
                    Ok(PySphere(z.0.get(idx as usize)).into_pyobject(vm))
                }),
                ..PyMappingMethods::NOT_IMPLEMENTED
            };
            &M
        }
    }
    #[pyclass(with(Constructor, Representable, AsMapping))]
    impl PySphereCollection {
        #[pymethod]
        fn len(&self) -> usize {
            self.0.len()
        }
        #[pymethod]
        fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        #[pystaticmethod]
        fn with_capacity(cap: usize) -> Self {
            Self(SpheresSoA::with_capacity(cap))
        }
        #[pystaticmethod]
        fn from_slice(spheres: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let list: Vec<PyObjectRef> = spheres.try_to_value(vm)?;
            let inner: Vec<Sphere> = list
                .iter()
                .map(|s| {
                    s.downcast_ref::<PySphere>()
                        .map(|p| p.0)
                        .ok_or_else(|| vm.new_type_error("expected Sphere".to_owned()))
                })
                .collect::<PyResult<_>>()?;
            Ok(Self(SpheresSoA::from_slice(&inner)))
        }
        /// Get the sphere at index `i`.
        #[pymethod]
        fn get(&self, i: usize, vm: &VirtualMachine) -> PyResult<PySphere> {
            if i >= self.0.len() {
                return Err(vm.new_index_error("index out of range".to_owned()));
            }
            Ok(PySphere(self.0.get(i)))
        }
        #[pymethod]
        fn any_collides_sphere(&self, sphere: PyObjectRef, vm: &VirtualMachine) -> PyResult<bool> {
            let s = sphere
                .downcast_ref::<PySphere>()
                .ok_or_else(|| vm.new_type_error("expected Sphere".to_owned()))?;
            Ok(self.0.any_collides_sphere(&s.0))
        }
        /// Append a sphere, returning the new collection (RustPython methods
        /// only borrow `&self`, so this does not mutate in place).
        #[pymethod]
        fn push(&self, sphere: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let s = sphere
                .downcast_ref::<PySphere>()
                .ok_or_else(|| vm.new_type_error("expected Sphere".to_owned()))?;
            let mut out = self.0.clone();
            out.push(s.0);
            Ok(Self(out))
        }
        /// Return a new empty collection.
        #[pymethod]
        fn clear(&self) -> Self {
            Self(SpheresSoA::new())
        }
        #[pymethod]
        fn abs_diff_eq(
            &self,
            other: PyObjectRef,
            max_abs_diff: f64,
            vm: &VirtualMachine,
        ) -> PyResult<bool> {
            let o = other
                .downcast_ref::<PySphereCollection>()
                .ok_or_else(|| vm.new_type_error("expected SphereCollection".to_owned()))?;
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
            crate::rp_serde::dataclass_fields(&[], vm)
        }
    }
}
