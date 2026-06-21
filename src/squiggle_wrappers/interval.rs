//! `Interval` wrapper — the inclusive `[min, max]` parameter range a curve spans.

use squiggle::Interval;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, skip_from_py_object, name = "Interval")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Interval")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone, Copy)]
pub struct PyInterval(pub Interval);

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::pickle::pickle_decode;
    use pyo3::PyResult;
    use pyo3::prelude::*;

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyInterval {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(*v.get());
            }
            let py = ob.py();
            let min: f64 = ob.getattr(pyo3::intern!(py, "min"))?.extract()?;
            let max: f64 = ob.getattr(pyo3::intern!(py, "max"))?.extract()?;
            Ok(Self(Interval::new(min as f32, max as f32)))
        }
    }

    #[pymethods]
    impl PyInterval {
        #[new]
        #[pyo3(signature = (min=None, max=None, *, __pickle_state__=None))]
        fn new(
            min: Option<f64>,
            max: Option<f64>,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<Interval>(&state)?));
            }
            match (min, max) {
                (Some(min), Some(max)) => Ok(Self(Interval::new(min as f32, max as f32))),
                _ => Err(pyo3::exceptions::PyValueError::new_err(
                    "Interval requires min, max arguments",
                )),
            }
        }
        #[staticmethod]
        fn unit() -> Self {
            Self(Interval::UNIT)
        }
        #[staticmethod]
        fn all() -> Self {
            Self(Interval::ALL)
        }
        #[getter]
        fn min(&self) -> f64 {
            self.0.min as f64
        }
        #[getter]
        fn max(&self) -> f64 {
            self.0.max as f64
        }
        fn span(&self) -> f64 {
            self.0.span() as f64
        }
        fn clamp(&self, t: f64) -> f64 {
            self.0.clamp(t as f32) as f64
        }
        fn lerp(&self, s: f64) -> f64 {
            self.0.lerp(s as f32) as f64
        }
        fn contains(&self, t: f64) -> bool {
            self.0.contains(t as f32)
        }
        fn is_finite(&self) -> bool {
            self.0.is_finite()
        }
        fn __repr__(&self) -> String {
            format!("Interval(min={}, max={})", self.0.min, self.0.max)
        }
    }

    crate::squiggle_wrappers::impl_approx_py!(PyInterval);
    crate::impl_getnewargs_ex!(PyInterval);
    crate::impl_dataclass_fields!(PyInterval, ["min", "max"]);
}

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use rustpython_vm::{
        Py, PyObjectRef, PyResult, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        pyclass,
        types::{Constructor, Representable},
    };

    impl Constructor for PyInterval {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<Interval>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            if args.args.len() != 2 {
                return Err(vm.new_type_error("Interval(min, max) requires 2 args".to_owned()));
            }
            let min = f64::try_from_object(vm, args.args[0].clone())?;
            let max = f64::try_from_object(vm, args.args[1].clone())?;
            Ok(Self(Interval::new(min as f32, max as f32)))
        }
    }
    impl Representable for PyInterval {
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok(format!("Interval(min={}, max={})", zelf.0.min, zelf.0.max))
        }
    }
    use rustpython_vm::TryFromObject;

    #[pyclass(with(Constructor, Representable))]
    impl PyInterval {
        #[pystaticmethod]
        fn unit() -> Self {
            Self(Interval::UNIT)
        }
        #[pystaticmethod]
        fn all() -> Self {
            Self(Interval::ALL)
        }
        #[pygetset]
        fn min(&self) -> f64 {
            self.0.min as f64
        }
        #[pygetset]
        fn max(&self) -> f64 {
            self.0.max as f64
        }
        #[pymethod]
        fn span(&self) -> f64 {
            self.0.span() as f64
        }
        #[pymethod]
        fn clamp(&self, t: f64) -> f64 {
            self.0.clamp(t as f32) as f64
        }
        #[pymethod]
        fn lerp(&self, s: f64) -> f64 {
            self.0.lerp(s as f32) as f64
        }
        #[pymethod]
        fn contains(&self, t: f64) -> bool {
            self.0.contains(t as f32)
        }
        #[pymethod]
        fn is_finite(&self) -> bool {
            self.0.is_finite()
        }
        #[pymethod]
        fn abs_diff_eq(
            &self,
            other: PyObjectRef,
            max_abs_diff: f64,
            vm: &VirtualMachine,
        ) -> PyResult<bool> {
            let o = other
                .downcast_ref::<PyInterval>()
                .ok_or_else(|| vm.new_type_error("expected Interval".to_owned()))?;
            Ok(approx::AbsDiffEq::abs_diff_eq(
                &self.0,
                &o.0,
                max_abs_diff as f32,
            ))
        }
        #[pymethod]
        fn __getnewargs_ex__(&self, vm: &VirtualMachine) -> PyResult<rustpython_vm::PyObjectRef> {
            crate::rp_serde::getnewargs_ex(&self.0, vm)
        }
        #[pygetset]
        fn __dataclass_fields__(&self, vm: &VirtualMachine) -> rustpython_vm::PyObjectRef {
            crate::rp_serde::dataclass_fields(&["min", "max"], vm)
        }
    }
}
