//! Bézier curve wrappers — `QuadraticBezier` (3 control points) and
//! `CubicBezier` (4 control points). squiggle's `Bezier<N>` is const-generic;
//! Python gets one concrete class per degree.

use squiggle::{CubicBezier, QuadraticBezier};

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, skip_from_py_object, name = "QuadraticBezier")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "QuadraticBezier")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone, Copy)]
pub struct PyQuadraticBezier(pub QuadraticBezier);

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, skip_from_py_object, name = "CubicBezier")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "CubicBezier")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone, Copy)]
pub struct PyCubicBezier(pub CubicBezier);

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::PyDVec3;
    use crate::pickle::pickle_decode;
    use crate::squiggle_wrappers::{
        impl_approx_py, impl_arclength_py, impl_curve_py, impl_transform_py, impl_trim_py,
    };
    use crate::wreck_wrappers::pyo3_glue::dv3;
    use pyo3::PyResult;
    use pyo3::prelude::*;

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyQuadraticBezier {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(*v.get());
            }
            let py = ob.py();
            let pts: Vec<PyDVec3> = ob.getattr(pyo3::intern!(py, "points"))?.extract()?;
            if pts.len() != 3 {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "QuadraticBezier requires exactly 3 control points",
                ));
            }
            Ok(Self(QuadraticBezier::new([
                dv3(pts[0]),
                dv3(pts[1]),
                dv3(pts[2]),
            ])))
        }
    }

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyCubicBezier {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(*v.get());
            }
            let py = ob.py();
            let pts: Vec<PyDVec3> = ob.getattr(pyo3::intern!(py, "points"))?.extract()?;
            if pts.len() != 4 {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "CubicBezier requires exactly 4 control points",
                ));
            }
            Ok(Self(CubicBezier::new([
                dv3(pts[0]),
                dv3(pts[1]),
                dv3(pts[2]),
                dv3(pts[3]),
            ])))
        }
    }

    #[pymethods]
    impl PyQuadraticBezier {
        #[new]
        #[pyo3(signature = (p0=None, p1=None, p2=None, *, __pickle_state__=None))]
        fn new(
            p0: Option<PyDVec3>,
            p1: Option<PyDVec3>,
            p2: Option<PyDVec3>,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<QuadraticBezier>(&state)?));
            }
            match (p0, p1, p2) {
                (Some(p0), Some(p1), Some(p2)) => {
                    Ok(Self(QuadraticBezier::new([dv3(p0), dv3(p1), dv3(p2)])))
                }
                _ => Err(pyo3::exceptions::PyValueError::new_err(
                    "QuadraticBezier requires p0, p1, p2 arguments",
                )),
            }
        }
        #[getter]
        fn points(&self) -> Vec<PyDVec3> {
            crate::squiggle_wrappers::control_points(&self.0)
        }
        fn split(&self, t: f64) -> (Self, Self) {
            let (a, b) = self.0.split(t as f32);
            (Self(a), Self(b))
        }
        fn __repr__(&self) -> String {
            "QuadraticBezier(3 points)".to_owned()
        }
    }

    #[pymethods]
    impl PyCubicBezier {
        #[new]
        #[pyo3(signature = (p0=None, p1=None, p2=None, p3=None, *, __pickle_state__=None))]
        fn new(
            p0: Option<PyDVec3>,
            p1: Option<PyDVec3>,
            p2: Option<PyDVec3>,
            p3: Option<PyDVec3>,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<CubicBezier>(&state)?));
            }
            match (p0, p1, p2, p3) {
                (Some(p0), Some(p1), Some(p2), Some(p3)) => Ok(Self(CubicBezier::new([
                    dv3(p0),
                    dv3(p1),
                    dv3(p2),
                    dv3(p3),
                ]))),
                _ => Err(pyo3::exceptions::PyValueError::new_err(
                    "CubicBezier requires p0, p1, p2, p3 arguments",
                )),
            }
        }
        #[getter]
        fn points(&self) -> Vec<PyDVec3> {
            crate::squiggle_wrappers::control_points(&self.0)
        }
        fn split(&self, t: f64) -> (Self, Self) {
            let (a, b) = self.0.split(t as f32);
            (Self(a), Self(b))
        }
        fn __repr__(&self) -> String {
            "CubicBezier(4 points)".to_owned()
        }
    }

    impl_curve_py!(PyQuadraticBezier);
    impl_transform_py!(PyQuadraticBezier);
    impl_trim_py!(PyQuadraticBezier);
    impl_arclength_py!(PyQuadraticBezier);
    impl_approx_py!(PyQuadraticBezier);
    crate::impl_getnewargs_ex!(PyQuadraticBezier);
    crate::impl_dataclass_fields!(PyQuadraticBezier, ["points"]);

    impl_curve_py!(PyCubicBezier);
    impl_transform_py!(PyCubicBezier);
    impl_trim_py!(PyCubicBezier);
    impl_arclength_py!(PyCubicBezier);
    impl_approx_py!(PyCubicBezier);
    crate::impl_getnewargs_ex!(PyCubicBezier);
    crate::impl_dataclass_fields!(PyCubicBezier, ["points"]);
}

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::glam_wrappers::PyDVec3;
    use crate::glam_wrappers::quat::extract_quat;
    use crate::glam_wrappers::vec3::extract_vec3;
    use crate::squiggle_wrappers::{PyInterval, PyNearest, vp};
    use crate::wreck_wrappers::rustpython_glue::{dv3, extract_affine3, extract_mat3};
    use rustpython_vm::{
        Py, PyObjectRef, PyPayload, PyResult, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        pyclass,
        types::{Constructor, Representable},
    };

    impl Constructor for PyQuadraticBezier {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<QuadraticBezier>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            if args.args.len() != 3 {
                return Err(
                    vm.new_type_error("QuadraticBezier(p0, p1, p2) requires 3 args".to_owned())
                );
            }
            Ok(Self(QuadraticBezier::new([
                dv3(extract_vec3(&args.args[0], vm)?),
                dv3(extract_vec3(&args.args[1], vm)?),
                dv3(extract_vec3(&args.args[2], vm)?),
            ])))
        }
    }
    impl Representable for PyQuadraticBezier {
        fn repr_str(_zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok("QuadraticBezier(3 points)".to_owned())
        }
    }

    impl Constructor for PyCubicBezier {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<CubicBezier>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            if args.args.len() != 4 {
                return Err(
                    vm.new_type_error("CubicBezier(p0, p1, p2, p3) requires 4 args".to_owned())
                );
            }
            Ok(Self(CubicBezier::new([
                dv3(extract_vec3(&args.args[0], vm)?),
                dv3(extract_vec3(&args.args[1], vm)?),
                dv3(extract_vec3(&args.args[2], vm)?),
                dv3(extract_vec3(&args.args[3], vm)?),
            ])))
        }
    }
    impl Representable for PyCubicBezier {
        fn repr_str(_zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok("CubicBezier(4 points)".to_owned())
        }
    }

    #[pyclass(with(Constructor, Representable))]
    impl PyQuadraticBezier {
        #[pygetset]
        fn points(&self, vm: &VirtualMachine) -> PyObjectRef {
            let items: Vec<PyObjectRef> = self.0.points.iter().map(|p| vp(*p).into_pyobject(vm)).collect();
            vm.ctx.new_list(items).into()
        }
        #[pymethod]
        fn control_points(&self, vm: &VirtualMachine) -> PyObjectRef {
            self.points(vm)
        }
        #[pymethod]
        fn split(&self, t: f64) -> (Self, Self) {
            let (a, b) = self.0.split(t as f32);
            (Self(a), Self(b))
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
            Ok(crate::squiggle_wrappers::nearest(&self.0, dv3(extract_vec3(&query, vm)?)))
        }
        #[pymethod]
        fn distance(&self, query: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(crate::squiggle_wrappers::distance(&self.0, dv3(extract_vec3(&query, vm)?)))
        }
        #[pymethod]
        fn distance_sq(&self, query: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(crate::squiggle_wrappers::distance_sq(&self.0, dv3(extract_vec3(&query, vm)?)))
        }

        #[pymethod]
        fn scaled(&self, factor: f64) -> Self {
            Self(squiggle::Transform::scaled(&self.0, factor as f32))
        }
        #[pymethod]
        fn translated(&self, offset: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(squiggle::Transform::translated(&self.0, dv3(extract_vec3(&offset, vm)?))))
        }
        #[pymethod]
        fn rotated_mat(&self, mat: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(squiggle::Transform::rotated_mat(&self.0, extract_mat3(&mat, vm)?.as_mat3())))
        }
        #[pymethod]
        fn rotated_quat(&self, quat: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(squiggle::Transform::rotated(&self.0, extract_quat(&quat, vm)?.as_quat())))
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
                .downcast_ref::<PyQuadraticBezier>()
                .ok_or_else(|| vm.new_type_error("expected QuadraticBezier".to_owned()))?;
            Ok(approx::AbsDiffEq::abs_diff_eq(&self.0, &o.0, max_abs_diff as f32))
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

    #[pyclass(with(Constructor, Representable))]
    impl PyCubicBezier {
        #[pygetset]
        fn points(&self, vm: &VirtualMachine) -> PyObjectRef {
            let items: Vec<PyObjectRef> = self.0.points.iter().map(|p| vp(*p).into_pyobject(vm)).collect();
            vm.ctx.new_list(items).into()
        }
        #[pymethod]
        fn control_points(&self, vm: &VirtualMachine) -> PyObjectRef {
            self.points(vm)
        }
        #[pymethod]
        fn split(&self, t: f64) -> (Self, Self) {
            let (a, b) = self.0.split(t as f32);
            (Self(a), Self(b))
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
            Ok(crate::squiggle_wrappers::nearest(&self.0, dv3(extract_vec3(&query, vm)?)))
        }
        #[pymethod]
        fn distance(&self, query: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(crate::squiggle_wrappers::distance(&self.0, dv3(extract_vec3(&query, vm)?)))
        }
        #[pymethod]
        fn distance_sq(&self, query: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(crate::squiggle_wrappers::distance_sq(&self.0, dv3(extract_vec3(&query, vm)?)))
        }

        #[pymethod]
        fn scaled(&self, factor: f64) -> Self {
            Self(squiggle::Transform::scaled(&self.0, factor as f32))
        }
        #[pymethod]
        fn translated(&self, offset: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(squiggle::Transform::translated(&self.0, dv3(extract_vec3(&offset, vm)?))))
        }
        #[pymethod]
        fn rotated_mat(&self, mat: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(squiggle::Transform::rotated_mat(&self.0, extract_mat3(&mat, vm)?.as_mat3())))
        }
        #[pymethod]
        fn rotated_quat(&self, quat: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(squiggle::Transform::rotated(&self.0, extract_quat(&quat, vm)?.as_quat())))
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
                .downcast_ref::<PyCubicBezier>()
                .ok_or_else(|| vm.new_type_error("expected CubicBezier".to_owned()))?;
            Ok(approx::AbsDiffEq::abs_diff_eq(&self.0, &o.0, max_abs_diff as f32))
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
