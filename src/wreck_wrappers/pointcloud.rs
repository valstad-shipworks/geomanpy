//! `Pointcloud` wrapper.

use wreck::Pointcloud;

/// `r_range` is the `(min, max)` bound on the radii of the query balls this
/// cloud will be tested against; queries with radii outside the range may
/// falsely report non-collision. Defaults to `(0.0, inf)`.
#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(module = "geomanpy", frozen, skip_from_py_object, name = "Pointcloud")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Pointcloud")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone)]
pub struct PyPointcloud(pub Pointcloud);

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::pickle::pickle_decode;
    use pyo3::PyResult;
    use pyo3::prelude::*;

    #[pymethods]
    impl PyPointcloud {
        #[new]
        #[pyo3(signature = (points=None, r_range=(0.0, f32::INFINITY), point_radius=0.0, *, __pickle_state__=None))]
        fn new(
            points: Option<Vec<[f32; 3]>>,
            r_range: (f32, f32),
            point_radius: f32,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<Pointcloud>(&state)?));
            }
            match points {
                Some(pts) => Ok(Self(Pointcloud::new(&pts, r_range, point_radius))),
                None => Err(pyo3::exceptions::PyValueError::new_err(
                    "Pointcloud requires points argument",
                )),
            }
        }

        #[staticmethod]
        #[pyo3(signature = (points, point_radius = 0.033, r_range = (0.0, f32::INFINITY)))]
        fn from_numpy(
            points: numpy::PyArrayLike2<'_, f64, numpy::AllowTypeChange>,
            point_radius: f32,
            r_range: (f32, f32),
        ) -> PyResult<Self> {
            let view = points.as_array();
            if view.shape()[1] != 3 {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "points must be (N, 3)",
                ));
            }
            let n = view.shape()[0];
            let mut pts = Vec::with_capacity(n);
            for i in 0..n {
                pts.push([
                    view[(i, 0)] as f32,
                    view[(i, 1)] as f32,
                    view[(i, 2)] as f32,
                ]);
            }
            Ok(Self(Pointcloud::new(&pts, r_range, point_radius)))
        }

        #[staticmethod]
        #[pyo3(signature = (points, point_radius = 0.033, r_range = (0.0, f32::INFINITY)))]
        fn from_list(
            points: Vec<[f64; 3]>,
            point_radius: f64,
            r_range: (f32, f32),
        ) -> PyResult<Self> {
            let pts: Vec<[f32; 3]> = points
                .iter()
                .map(|p| [p[0] as f32, p[1] as f32, p[2] as f32])
                .collect();
            Ok(Self(Pointcloud::new(&pts, r_range, point_radius as f32)))
        }

        fn __repr__(&self) -> String {
            "Pointcloud(...)".to_string()
        }
    }

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyPointcloud {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(v.get().clone());
            }
            let py = ob.py();
            let newargs = ob.call_method0(pyo3::intern!(py, "__getnewargs_ex__"))?;
            let (_, kwargs): (
                pyo3::Bound<'py, pyo3::PyAny>,
                pyo3::Bound<'py, pyo3::types::PyDict>,
            ) = newargs.extract()?;
            let state: Vec<u8> = kwargs
                .get_item(pyo3::intern!(py, "__pickle_state__"))?
                .ok_or_else(|| pyo3::exceptions::PyTypeError::new_err("expected Pointcloud"))?
                .extract()?;
            Ok(Self(pickle_decode::<Pointcloud>(&state)?))
        }
    }
}

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::glam_wrappers::quat::extract_quat;
    use crate::glam_wrappers::vec3::extract_vec3;
    use crate::wreck_wrappers::rustpython_glue::{extract_affine3, extract_mat3, shape_collides};
    use crate::wreck_wrappers::{PyCuboid, PySphere};
    use rustpython_vm::{
        Py, PyObjectRef, PyRef, PyResult, VirtualMachine,
        builtins::PyType,
        function::{FuncArgs, OptionalArg},
        pyclass,
        types::{Constructor, Representable},
    };
    use wreck::{Scalable, Transformable};

    fn extract_points(points: &PyObjectRef, vm: &VirtualMachine) -> PyResult<Vec<[f32; 3]>> {
        let pts: Vec<Vec<f64>> = points.try_to_value(vm)?;
        let mut out = Vec::with_capacity(pts.len());
        for p in &pts {
            if p.len() != 3 {
                return Err(vm.new_value_error("each point must be a 3-tuple".to_owned()));
            }
            out.push([p[0] as f32, p[1] as f32, p[2] as f32]);
        }
        Ok(out)
    }

    fn extract_r_range(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<(f32, f32)> {
        let vals: Vec<f64> = obj.try_to_value(vm)?;
        if vals.len() != 2 {
            return Err(vm.new_value_error("r_range must be a (min, max) pair".to_owned()));
        }
        Ok((vals[0] as f32, vals[1] as f32))
    }

    impl Constructor for PyPointcloud {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, mut args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<Pointcloud>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            if args.args.len() > 3 {
                return Err(vm.new_type_error(format!(
                    "Pointcloud() takes 3 positional arguments but {} were given",
                    args.args.len()
                )));
            }
            let mut values: [Option<PyObjectRef>; 3] = [None, None, None];
            for (slot, val) in values.iter_mut().zip(args.args.drain(..)) {
                *slot = Some(val);
            }
            for (slot, name) in values.iter_mut().zip(["points", "r_range", "point_radius"]) {
                if let Some(val) = args.kwargs.swap_remove(name) {
                    if slot.is_some() {
                        return Err(vm.new_type_error(format!(
                            "Pointcloud() got multiple values for argument '{name}'"
                        )));
                    }
                    *slot = Some(val);
                }
            }
            if let Some(name) = args.kwargs.keys().next() {
                return Err(vm.new_type_error(format!(
                    "Pointcloud() got an unexpected keyword argument '{name}'"
                )));
            }
            let [points, r_range, point_radius] = values;
            let Some(points) = points else {
                return Err(vm.new_value_error("Pointcloud requires points argument".to_owned()));
            };
            let pts = extract_points(&points, vm)?;
            let r_range = match r_range {
                Some(obj) => extract_r_range(&obj, vm)?,
                None => (0.0, f32::INFINITY),
            };
            let point_radius = match point_radius {
                Some(obj) => obj.try_float(vm)?.to_f64() as f32,
                None => 0.0,
            };
            Ok(Self(Pointcloud::new(&pts, r_range, point_radius)))
        }
    }
    impl Representable for PyPointcloud {
        fn repr_str(_zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok("Pointcloud(...)".to_owned())
        }
    }
    impl PyPointcloud {
        pub(crate) const DATACLASS_FIELDS: &'static [&'static str] = &[];
    }

    #[pyclass(with(Constructor, Representable))]
    impl PyPointcloud {
        #[pystaticmethod]
        fn from_numpy(
            points: PyObjectRef,
            point_radius: OptionalArg<f64>,
            r_range: OptionalArg<PyObjectRef>,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            let arr: ndarray::ArrayD<f64> = rumpy::convert::obj_to_typed::<f64>(&points, vm)?;
            if arr.ndim() != 2 || arr.shape()[1] != 3 {
                return Err(vm.new_value_error("points must be (N, 3)".to_owned()));
            }
            let n = arr.shape()[0];
            let mut pts = Vec::with_capacity(n);
            for i in 0..n {
                pts.push([arr[[i, 0]] as f32, arr[[i, 1]] as f32, arr[[i, 2]] as f32]);
            }
            let point_radius = point_radius.unwrap_or(0.033) as f32;
            let r_range = match r_range {
                OptionalArg::Present(obj) => extract_r_range(&obj, vm)?,
                OptionalArg::Missing => (0.0, f32::INFINITY),
            };
            Ok(Self(Pointcloud::new(&pts, r_range, point_radius)))
        }

        #[pystaticmethod]
        fn from_list(
            points: PyObjectRef,
            point_radius: OptionalArg<f64>,
            r_range: OptionalArg<PyObjectRef>,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            let pts = extract_points(&points, vm)?;
            let point_radius = point_radius.unwrap_or(0.033) as f32;
            let r_range = match r_range {
                OptionalArg::Present(obj) => extract_r_range(&obj, vm)?,
                OptionalArg::Missing => (0.0, f32::INFINITY),
            };
            Ok(Self(Pointcloud::new(&pts, r_range, point_radius)))
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
        fn abs_diff_eq(
            &self,
            other: PyObjectRef,
            max_abs_diff: f64,
            vm: &VirtualMachine,
        ) -> PyResult<bool> {
            let o = other
                .downcast_ref::<PyPointcloud>()
                .ok_or_else(|| vm.new_type_error("expected Pointcloud".to_owned()))?;
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
        fn __dict__(zelf: PyRef<Self>, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            crate::rp_serde::dataclass_dict(zelf.into(), Self::DATACLASS_FIELDS, vm)
        }
    }
}
