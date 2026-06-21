//! `Nearest` wrapper — the result of a closest-point query against a curve.

use squiggle::Nearest;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, skip_from_py_object, name = "Nearest")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Nearest")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone, Copy)]
pub struct PyNearest(pub Nearest);

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::PyDVec3;
    use crate::pickle::pickle_decode;
    use crate::squiggle_wrappers::vp;
    use crate::wreck_wrappers::pyo3_glue::dv3;
    use pyo3::PyResult;
    use pyo3::prelude::*;

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyNearest {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(*v.get());
            }
            let py = ob.py();
            let t: f64 = ob.getattr(pyo3::intern!(py, "t"))?.extract()?;
            let point = dv3(ob
                .getattr(pyo3::intern!(py, "point"))?
                .extract::<PyDVec3>()?);
            let dist_sq: f64 = ob.getattr(pyo3::intern!(py, "dist_sq"))?.extract()?;
            Ok(Self(Nearest {
                t: t as f32,
                point,
                dist_sq: dist_sq as f32,
            }))
        }
    }

    #[pymethods]
    impl PyNearest {
        #[new]
        #[pyo3(signature = (t=None, point=None, dist_sq=None, *, __pickle_state__=None))]
        fn new(
            t: Option<f64>,
            point: Option<PyDVec3>,
            dist_sq: Option<f64>,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<Nearest>(&state)?));
            }
            match (t, point, dist_sq) {
                (Some(t), Some(point), Some(dist_sq)) => Ok(Self(Nearest {
                    t: t as f32,
                    point: dv3(point),
                    dist_sq: dist_sq as f32,
                })),
                _ => Err(pyo3::exceptions::PyValueError::new_err(
                    "Nearest requires t, point, dist_sq arguments",
                )),
            }
        }
        #[getter]
        fn t(&self) -> f64 {
            self.0.t as f64
        }
        #[getter]
        fn point(&self) -> PyDVec3 {
            vp(self.0.point)
        }
        #[getter]
        fn dist_sq(&self) -> f64 {
            self.0.dist_sq as f64
        }
        fn distance(&self) -> f64 {
            (self.0.dist_sq as f64).sqrt()
        }
        fn __repr__(&self) -> String {
            let p = self.0.point;
            format!(
                "Nearest(t={}, point=[{}, {}, {}], dist_sq={})",
                self.0.t, p.x, p.y, p.z, self.0.dist_sq
            )
        }
    }

    crate::squiggle_wrappers::impl_approx_py!(PyNearest);
    crate::impl_getnewargs_ex!(PyNearest);
    crate::impl_dataclass_fields!(PyNearest, ["t", "point", "dist_sq"]);
}

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::glam_wrappers::PyDVec3;
    use crate::glam_wrappers::vec3::extract_vec3;
    use crate::squiggle_wrappers::vp;
    use crate::wreck_wrappers::rustpython_glue::dv3;
    use rustpython_vm::{
        Py, PyObjectRef, PyResult, TryFromObject, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        pyclass,
        types::{Constructor, Representable},
    };

    impl Constructor for PyNearest {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<Nearest>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            if args.args.len() != 3 {
                return Err(
                    vm.new_type_error("Nearest(t, point, dist_sq) requires 3 args".to_owned())
                );
            }
            let t = f64::try_from_object(vm, args.args[0].clone())?;
            let point = dv3(extract_vec3(&args.args[1], vm)?);
            let dist_sq = f64::try_from_object(vm, args.args[2].clone())?;
            Ok(Self(Nearest {
                t: t as f32,
                point,
                dist_sq: dist_sq as f32,
            }))
        }
    }
    impl Representable for PyNearest {
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            let p = zelf.0.point;
            Ok(format!(
                "Nearest(t={}, point=[{}, {}, {}], dist_sq={})",
                zelf.0.t, p.x, p.y, p.z, zelf.0.dist_sq
            ))
        }
    }

    #[pyclass(with(Constructor, Representable))]
    impl PyNearest {
        #[pygetset]
        fn t(&self) -> f64 {
            self.0.t as f64
        }
        #[pygetset]
        fn point(&self) -> PyDVec3 {
            vp(self.0.point)
        }
        #[pygetset]
        fn dist_sq(&self) -> f64 {
            self.0.dist_sq as f64
        }
        #[pymethod]
        fn distance(&self) -> f64 {
            (self.0.dist_sq as f64).sqrt()
        }
        #[pymethod]
        fn abs_diff_eq(
            &self,
            other: PyObjectRef,
            max_abs_diff: f64,
            vm: &VirtualMachine,
        ) -> PyResult<bool> {
            let o = other
                .downcast_ref::<PyNearest>()
                .ok_or_else(|| vm.new_type_error("expected Nearest".to_owned()))?;
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
            crate::rp_serde::dataclass_fields(&["t", "point", "dist_sq"], vm)
        }
    }
}
