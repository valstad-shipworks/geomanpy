//! `Polyline` wrapper — a piecewise-linear curve through an ordered vertex sequence.

use squiggle::Polyline;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, from_py_object, name = "Polyline")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Polyline")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone)]
pub struct PyPolyline(pub Polyline);

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::PyDVec3;
    use crate::pickle::pickle_decode;
    use crate::squiggle_wrappers::{
        impl_approx_py, impl_arclength_py, impl_curve_py, impl_transform_py, impl_trim_py,
    };
    use crate::wreck_wrappers::PyLineSegment;
    use crate::wreck_wrappers::pyo3_glue::dv3;
    use pyo3::PyResult;
    use pyo3::prelude::*;

    #[pymethods]
    impl PyPolyline {
        #[new]
        #[pyo3(signature = (points=None, *, __pickle_state__=None))]
        fn new(points: Option<Vec<PyDVec3>>, __pickle_state__: Option<Vec<u8>>) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<Polyline>(&state)?));
            }
            let points = points.unwrap_or_default().into_iter().map(dv3).collect();
            Ok(Self(Polyline::new(points)))
        }
        #[getter]
        fn points(&self) -> Vec<PyDVec3> {
            crate::squiggle_wrappers::control_points(&self.0)
        }
        fn segments(&self) -> Vec<PyLineSegment> {
            self.0.segments().map(PyLineSegment).collect()
        }
        fn __repr__(&self) -> String {
            format!("Polyline(points={})", self.0.points.len())
        }
    }

    impl_curve_py!(PyPolyline);
    impl_transform_py!(PyPolyline);
    impl_trim_py!(PyPolyline);
    impl_arclength_py!(PyPolyline);
    impl_approx_py!(PyPolyline);

    crate::impl_getnewargs_ex!(PyPolyline);
    crate::impl_dataclass_fields!(PyPolyline, ["points"]);
}

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::glam_wrappers::PyDVec3;
    use crate::glam_wrappers::quat::extract_quat;
    use crate::glam_wrappers::vec3::extract_vec3;
    use crate::squiggle_wrappers::{PyInterval, PyNearest, vp};
    use crate::wreck_wrappers::PyLineSegment;
    use crate::wreck_wrappers::rustpython_glue::{dv3, extract_affine3, extract_mat3};
    use rustpython_vm::{
        Py, PyObjectRef, PyPayload, PyResult, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        pyclass,
        types::{Constructor, Representable},
    };

    impl Constructor for PyPolyline {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<Polyline>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            let points = if args.args.is_empty() {
                Vec::new()
            } else {
                vm.extract_elements_with(&args.args[0], |o| Ok(dv3(extract_vec3(&o, vm)?)))?
            };
            Ok(Self(Polyline::new(points)))
        }
    }
    impl Representable for PyPolyline {
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok(format!("Polyline(points={})", zelf.0.points.len()))
        }
    }

    #[pyclass(with(Constructor, Representable))]
    impl PyPolyline {
        #[pygetset]
        fn points(&self, vm: &VirtualMachine) -> PyObjectRef {
            let items: Vec<PyObjectRef> = self
                .0
                .points
                .iter()
                .map(|p| vp(*p).into_pyobject(vm))
                .collect();
            vm.ctx.new_list(items).into()
        }
        #[pymethod]
        fn control_points(&self, vm: &VirtualMachine) -> PyObjectRef {
            self.points(vm)
        }
        #[pymethod]
        fn segments(&self, vm: &VirtualMachine) -> PyObjectRef {
            let items: Vec<PyObjectRef> = self
                .0
                .segments()
                .map(|s| PyLineSegment(s).into_pyobject(vm))
                .collect();
            vm.ctx.new_list(items).into()
        }

        #[pymethod]
        fn domain(&self) -> PyInterval {
            crate::squiggle_wrappers::domain(&self.0)
        }
        #[pymethod]
        fn point(&self, t: f64) -> PyDVec3 {
            crate::squiggle_wrappers::point(&self.0, t as f32)
        }
        #[pymethod]
        fn velocity(&self, t: f64) -> PyDVec3 {
            crate::squiggle_wrappers::velocity(&self.0, t as f32)
        }
        #[pymethod]
        fn acceleration(&self, t: f64) -> PyDVec3 {
            crate::squiggle_wrappers::acceleration(&self.0, t as f32)
        }
        #[pymethod]
        fn tangent(&self, t: f64) -> PyDVec3 {
            crate::squiggle_wrappers::tangent(&self.0, t as f32)
        }
        #[pymethod]
        fn normal(&self, t: f64) -> PyDVec3 {
            crate::squiggle_wrappers::normal(&self.0, t as f32)
        }
        #[pymethod]
        fn binormal(&self, t: f64) -> PyDVec3 {
            crate::squiggle_wrappers::binormal(&self.0, t as f32)
        }
        #[pymethod]
        fn curvature(&self, t: f64) -> f64 {
            crate::squiggle_wrappers::curvature(&self.0, t as f32)
        }
        #[pymethod]
        fn point_clamped(&self, t: f64) -> PyDVec3 {
            crate::squiggle_wrappers::point_clamped(&self.0, t as f32)
        }
        #[pymethod]
        fn endpoints(&self) -> (PyDVec3, PyDVec3) {
            crate::squiggle_wrappers::endpoints(&self.0)
        }
        #[pymethod]
        fn length(&self) -> f64 {
            crate::squiggle_wrappers::length(&self.0)
        }
        #[pymethod]
        fn aabb(&self) -> crate::wreck_wrappers::PyCuboid {
            crate::squiggle_wrappers::aabb(&self.0)
        }
        #[pymethod]
        fn nearest(&self, query: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyNearest> {
            Ok(crate::squiggle_wrappers::nearest(
                &self.0,
                dv3(extract_vec3(&query, vm)?),
            ))
        }
        #[pymethod]
        fn distance(&self, query: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(crate::squiggle_wrappers::distance(
                &self.0,
                dv3(extract_vec3(&query, vm)?),
            ))
        }
        #[pymethod]
        fn distance_sq(&self, query: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(crate::squiggle_wrappers::distance_sq(
                &self.0,
                dv3(extract_vec3(&query, vm)?),
            ))
        }

        #[pymethod]
        fn scaled(&self, factor: f64) -> Self {
            Self(squiggle::Transform::scaled(&self.0, factor as f32))
        }
        #[pymethod]
        fn translated(&self, offset: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(squiggle::Transform::translated(
                &self.0,
                dv3(extract_vec3(&offset, vm)?),
            )))
        }
        #[pymethod]
        fn rotated_mat(&self, mat: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(squiggle::Transform::rotated_mat(
                &self.0,
                extract_mat3(&mat, vm)?.as_mat3(),
            )))
        }
        #[pymethod]
        fn rotated_quat(&self, quat: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(squiggle::Transform::rotated(
                &self.0,
                extract_quat(&quat, vm)?.as_quat(),
            )))
        }
        #[pymethod]
        fn transformed(&self, tf: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(squiggle::Transform::transformed(
                &self.0,
                glam::Affine3A::from(extract_affine3(&tf, vm)?.as_affine3()),
            )))
        }

        #[pymethod]
        fn subcurve(&self, t0: f64, t1: f64) -> Self {
            Self(squiggle::Trim::subcurve(&self.0, t0 as f32, t1 as f32))
        }
        #[pymethod]
        fn reversed(&self) -> Self {
            Self(squiggle::Trim::reversed(&self.0))
        }
        #[pymethod]
        fn truncate_start(&self, t0: f64) -> Self {
            Self(squiggle::Trim::truncate_start(&self.0, t0 as f32))
        }
        #[pymethod]
        fn truncate_end(&self, t1: f64) -> Self {
            Self(squiggle::Trim::truncate_end(&self.0, t1 as f32))
        }
        #[pymethod]
        fn split_at(&self, t: f64) -> (Self, Self) {
            let (a, b) = squiggle::Trim::split_at(&self.0, t as f32);
            (Self(a), Self(b))
        }

        #[pymethod]
        fn arc_length_to(&self, t: f64) -> f64 {
            crate::squiggle_wrappers::arc_length_to(&self.0, t as f32)
        }
        #[pymethod]
        fn t_at_distance(&self, s: f64) -> f64 {
            crate::squiggle_wrappers::t_at_distance(&self.0, s as f32)
        }
        #[pymethod]
        fn point_at_distance(&self, s: f64) -> PyDVec3 {
            crate::squiggle_wrappers::point_at_distance(&self.0, s as f32)
        }

        #[pymethod]
        fn abs_diff_eq(
            &self,
            other: PyObjectRef,
            max_abs_diff: f64,
            vm: &VirtualMachine,
        ) -> PyResult<bool> {
            let o = other
                .downcast_ref::<PyPolyline>()
                .ok_or_else(|| vm.new_type_error("expected Polyline".to_owned()))?;
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
            crate::rp_serde::dataclass_fields(&["points"], vm)
        }
    }
}
