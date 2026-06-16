//! `Quat` (double-precision) wrapper.

use glam::DQuat;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, skip_from_py_object, name = "Quat")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Quat")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PyDQuat(pub DQuat);

impl From<DQuat> for PyDQuat {
    #[inline]
    fn from(q: DQuat) -> Self {
        Self(q)
    }
}

impl From<PyDQuat> for DQuat {
    #[inline]
    fn from(q: PyDQuat) -> Self {
        q.0
    }
}

// =============================================================================
// PyO3 backend
// =============================================================================

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::{
        PyDAffine3, PyDMat3, PyDMat4, PyDVec2, PyDVec3, PyEulerRot, extract_numpy_vector,
        impl_serde_methods,
    };
    use crate::pickle::pickle_decode;
    use crate::{impl_dataclass_fields, impl_getnewargs_ex};
    use numpy::{AllowTypeChange, PyArray1, PyArrayLike1};
    use pyo3::prelude::*;

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyDQuat {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(*v.get());
            }
            if let Ok(xs) = ob.extract::<[f64; 4]>() {
                return Ok(Self(DQuat::from_xyzw(xs[0], xs[1], xs[2], xs[3])));
            }
            let py = ob.py();
            let x: f64 = ob.getattr(pyo3::intern!(py, "x"))?.extract()?;
            let y: f64 = ob.getattr(pyo3::intern!(py, "y"))?.extract()?;
            let z: f64 = ob.getattr(pyo3::intern!(py, "z"))?.extract()?;
            let w: f64 = ob.getattr(pyo3::intern!(py, "w"))?.extract()?;
            Ok(Self(DQuat::from_xyzw(x, y, z, w)))
        }
    }

    #[pymethods]
    impl PyDQuat {
        #[new]
        #[pyo3(signature = (x=0.0, y=0.0, z=0.0, w=1.0, *, __pickle_state__=None))]
        #[inline]
        fn new(
            x: f64,
            y: f64,
            z: f64,
            w: f64,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<DQuat>(&state)?));
            }
            Ok(Self(DQuat::from_xyzw(x, y, z, w)))
        }

        #[staticmethod]
        #[inline]
        fn from_xyzw(x: f64, y: f64, z: f64, w: f64) -> Self {
            Self(DQuat::from_xyzw(x, y, z, w))
        }
        #[staticmethod]
        #[inline]
        fn from_array(a: [f64; 4]) -> Self {
            Self(DQuat::from_array(a))
        }
        #[staticmethod]
        #[inline]
        fn from_numpy(array: PyArrayLike1<'_, f64, AllowTypeChange>) -> PyResult<Self> {
            Ok(Self(DQuat::from_array(extract_numpy_vector::<4>(
                array, "Quat",
            )?)))
        }
        #[staticmethod]
        #[inline]
        fn from_axis_angle(axis: PyDVec3, angle: f64) -> Self {
            Self(DQuat::from_axis_angle(axis.0, angle))
        }
        #[staticmethod]
        #[inline]
        fn from_scaled_axis(v: PyDVec3) -> Self {
            Self(DQuat::from_scaled_axis(v.0))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_x(angle: f64) -> Self {
            Self(DQuat::from_rotation_x(angle))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_y(angle: f64) -> Self {
            Self(DQuat::from_rotation_y(angle))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_z(angle: f64) -> Self {
            Self(DQuat::from_rotation_z(angle))
        }
        #[staticmethod]
        #[inline]
        fn from_euler(order: PyEulerRot, a: f64, b: f64, c: f64) -> Self {
            Self(DQuat::from_euler(order.into(), a, b, c))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_axes(x_axis: PyDVec3, y_axis: PyDVec3, z_axis: PyDVec3) -> Self {
            Self(DQuat::from_rotation_axes(x_axis.0, y_axis.0, z_axis.0))
        }
        #[staticmethod]
        #[inline]
        fn from_mat3(mat: PyDMat3) -> Self {
            Self(DQuat::from_mat3(&mat.0))
        }
        #[staticmethod]
        #[inline]
        fn from_mat4(mat: PyDMat4) -> Self {
            Self(DQuat::from_mat4(&mat.0))
        }
        #[staticmethod]
        #[inline]
        fn from_affine3(a: PyDAffine3) -> Self {
            Self(DQuat::from_affine3(&a.0))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_arc(from: PyDVec3, to: PyDVec3) -> Self {
            Self(DQuat::from_rotation_arc(from.0, to.0))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_arc_colinear(from: PyDVec3, to: PyDVec3) -> Self {
            Self(DQuat::from_rotation_arc_colinear(from.0, to.0))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_arc_2d(from: PyDVec2, to: PyDVec2) -> Self {
            Self(DQuat::from_rotation_arc_2d(from.0, to.0))
        }
        #[staticmethod]
        #[inline]
        fn look_to_lh(dir: PyDVec3, up: PyDVec3) -> Self {
            Self(DQuat::look_to_lh(dir.0, up.0))
        }
        #[staticmethod]
        #[inline]
        fn look_to_rh(dir: PyDVec3, up: PyDVec3) -> Self {
            Self(DQuat::look_to_rh(dir.0, up.0))
        }
        #[staticmethod]
        #[inline]
        fn look_at_lh(eye: PyDVec3, center: PyDVec3, up: PyDVec3) -> Self {
            Self(DQuat::look_at_lh(eye.0, center.0, up.0))
        }
        #[staticmethod]
        #[inline]
        fn look_at_rh(eye: PyDVec3, center: PyDVec3, up: PyDVec3) -> Self {
            Self(DQuat::look_at_rh(eye.0, center.0, up.0))
        }
    }

    #[pymethods]
    impl PyDQuat {
        #[getter]
        #[inline]
        fn x(&self) -> f64 {
            self.0.x
        }
        #[getter]
        #[inline]
        fn y(&self) -> f64 {
            self.0.y
        }
        #[getter]
        #[inline]
        fn z(&self) -> f64 {
            self.0.z
        }
        #[getter]
        #[inline]
        fn w(&self) -> f64 {
            self.0.w
        }
    }

    #[pymethods]
    impl PyDQuat {
        #[inline]
        fn conjugate(&self) -> Self {
            Self(self.0.conjugate())
        }
        #[inline]
        fn inverse(&self) -> Self {
            Self(self.0.inverse())
        }
        #[inline]
        fn dot(&self, rhs: Self) -> f64 {
            self.0.dot(rhs.0)
        }
        #[inline]
        fn length(&self) -> f64 {
            self.0.length()
        }
        #[inline]
        fn length_squared(&self) -> f64 {
            self.0.length_squared()
        }
        #[inline]
        fn length_recip(&self) -> f64 {
            self.0.length_recip()
        }
        #[inline]
        fn normalize(&self) -> Self {
            Self(self.0.normalize())
        }
        #[inline]
        fn mul_vec3(&self, rhs: PyDVec3) -> PyDVec3 {
            PyDVec3(self.0.mul_vec3(rhs.0))
        }
        #[inline]
        fn mul_quat(&self, rhs: Self) -> Self {
            Self(self.0.mul_quat(rhs.0))
        }
    }

    #[pymethods]
    impl PyDQuat {
        #[inline]
        fn lerp(&self, end: Self, s: f64) -> Self {
            Self(self.0.lerp(end.0, s))
        }
        #[inline]
        fn slerp(&self, end: Self, s: f64) -> Self {
            Self(self.0.slerp(end.0, s))
        }
        #[inline]
        fn angle_between(&self, rhs: Self) -> f64 {
            self.0.angle_between(rhs.0)
        }
        #[inline]
        fn rotate_towards(&self, rhs: Self, max_angle: f64) -> Self {
            Self(self.0.rotate_towards(rhs.0, max_angle))
        }
    }

    #[pymethods]
    impl PyDQuat {
        #[inline]
        fn to_axis_angle(&self) -> (PyDVec3, f64) {
            let (axis, angle) = self.0.to_axis_angle();
            (PyDVec3(axis), angle)
        }
        #[inline]
        fn to_scaled_axis(&self) -> PyDVec3 {
            PyDVec3(self.0.to_scaled_axis())
        }
        #[inline]
        fn to_euler(&self, order: PyEulerRot) -> (f64, f64, f64) {
            self.0.to_euler(order.into())
        }
        #[inline]
        fn to_array(&self) -> [f64; 4] {
            self.0.to_array()
        }
        #[inline]
        fn to_numpy<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
            PyArray1::from_slice(py, &self.0.to_array())
        }
        #[inline]
        fn xyz(&self) -> PyDVec3 {
            PyDVec3(self.0.xyz())
        }
    }

    #[pymethods]
    impl PyDQuat {
        #[inline]
        fn is_finite(&self) -> bool {
            self.0.is_finite()
        }
        #[inline]
        fn is_nan(&self) -> bool {
            self.0.is_nan()
        }
        #[inline]
        fn is_normalized(&self) -> bool {
            self.0.is_normalized()
        }
        #[inline]
        fn is_near_identity(&self) -> bool {
            self.0.is_near_identity()
        }
        #[inline]
        fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f64) -> bool {
            self.0.abs_diff_eq(rhs.0, max_abs_diff)
        }
    }

    #[pymethods]
    impl PyDQuat {
        #[classattr]
        #[pyo3(name = "IDENTITY")]
        fn identity_const() -> Self {
            Self(DQuat::IDENTITY)
        }
        #[classattr]
        #[pyo3(name = "NAN")]
        fn nan_const() -> Self {
            Self(DQuat::NAN)
        }
    }

    #[pymethods]
    impl PyDQuat {
        fn __repr__(&self) -> String {
            format!(
                "Quat({}, {}, {}, {})",
                self.0.x, self.0.y, self.0.z, self.0.w
            )
        }
        fn __str__(&self) -> String {
            format!("[{}, {}, {}, {}]", self.0.x, self.0.y, self.0.z, self.0.w)
        }
        fn __eq__(&self, other: Self) -> bool {
            self.0 == other.0
        }
        fn __ne__(&self, other: Self) -> bool {
            self.0 != other.0
        }
        fn __neg__(&self) -> Self {
            Self(-self.0)
        }

        fn __mul__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
            let py = other.py();
            if let Ok(q) = other.extract::<Self>() {
                Ok(Self(self.0.mul_quat(q.0))
                    .into_pyobject(py)?
                    .into_any()
                    .unbind())
            } else if let Ok(v) = other.extract::<PyDVec3>() {
                Ok(PyDVec3(self.0.mul_vec3(v.0))
                    .into_pyobject(py)?
                    .into_any()
                    .unbind())
            } else if let Ok(s) = other.extract::<f64>() {
                Ok(Self(self.0 * s).into_pyobject(py)?.into_any().unbind())
            } else {
                Err(pyo3::exceptions::PyTypeError::new_err(
                    "unsupported operand type for *",
                ))
            }
        }

        fn __rmul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
            if let Ok(s) = other.extract::<f64>() {
                Ok(Self(self.0 * s))
            } else {
                Err(pyo3::exceptions::PyTypeError::new_err(
                    "unsupported operand type for *",
                ))
            }
        }

        fn __add__(&self, other: Self) -> Self {
            Self(self.0 + other.0)
        }
        fn __sub__(&self, other: Self) -> Self {
            Self(self.0 - other.0)
        }
        fn __truediv__(&self, other: f64) -> Self {
            Self(self.0 / other)
        }

        fn __array__<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
            PyArray1::from_slice(py, &self.0.to_array())
        }

        fn __hash__(&self) -> u64 {
            use std::hash::{Hash, Hasher};
            let mut h = std::collections::hash_map::DefaultHasher::new();
            for c in self.0.to_array() {
                c.to_bits().hash(&mut h);
            }
            h.finish()
        }
    }

    impl_serde_methods!(PyDQuat, DQuat);
    impl_getnewargs_ex!(PyDQuat);
    impl_dataclass_fields!(PyDQuat, ["x", "y", "z", "w"]);
}

// =============================================================================
// RustPython backend
// =============================================================================

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::glam_wrappers::vec2::extract_vec2;
    use crate::glam_wrappers::vec3::extract_vec3;
    use crate::glam_wrappers::{PyDAffine3, PyDMat3, PyDMat4, PyDVec3, PyEulerRot};
    use rustpython_vm::{
        Py, PyObject, PyObjectRef, PyPayload, PyResult, TryFromObject, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        pyclass,
        types::{AsNumber, Comparable, Constructor, Hashable, PyComparisonOp, Representable},
    };

    pub(crate) fn extract(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<DQuat> {
        if let Some(v) = obj.downcast_ref::<PyDQuat>() {
            return Ok(v.0);
        }
        if let Ok(xs) = obj.try_to_value::<Vec<f64>>(vm)
            && xs.len() == 4
        {
            return Ok(DQuat::from_xyzw(xs[0], xs[1], xs[2], xs[3]));
        }
        let x: f64 = obj.get_attr("x", vm)?.try_float(vm)?.to_f64();
        let y: f64 = obj.get_attr("y", vm)?.try_float(vm)?.to_f64();
        let z: f64 = obj.get_attr("z", vm)?.try_float(vm)?.to_f64();
        let w: f64 = obj.get_attr("w", vm)?.try_float(vm)?.to_f64();
        Ok(DQuat::from_xyzw(x, y, z, w))
    }

    fn extract_euler(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<glam::EulerRot> {
        match obj.downcast_ref::<PyEulerRot>() {
            Some(e) => Ok((**e).into()),
            None => Err(vm.new_type_error("expected an EulerRot".to_owned())),
        }
    }

    fn extract_mat3(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<glam::DMat3> {
        match obj.downcast_ref::<PyDMat3>() {
            Some(m) => Ok(m.0),
            None => Err(vm.new_type_error("expected a Mat3".to_owned())),
        }
    }

    fn extract_mat4(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<glam::DMat4> {
        match obj.downcast_ref::<PyDMat4>() {
            Some(m) => Ok(m.0),
            None => Err(vm.new_type_error("expected a Mat4".to_owned())),
        }
    }

    fn extract_affine3(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<glam::DAffine3> {
        match obj.downcast_ref::<PyDAffine3>() {
            Some(a) => Ok(a.0),
            None => Err(vm.new_type_error("expected an Affine3".to_owned())),
        }
    }

    impl Constructor for PyDQuat {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<DQuat>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            let (x, y, z, w) = match args.args.len() {
                0 => (0.0, 0.0, 0.0, 1.0),
                4 => (
                    args.args[0].try_float(vm)?.to_f64(),
                    args.args[1].try_float(vm)?.to_f64(),
                    args.args[2].try_float(vm)?.to_f64(),
                    args.args[3].try_float(vm)?.to_f64(),
                ),
                n => {
                    return Err(vm.new_type_error(format!(
                        "Quat() takes 0 or 4 positional arguments, got {n}"
                    )));
                }
            };
            Ok(PyDQuat(DQuat::from_xyzw(x, y, z, w)))
        }
    }

    impl Representable for PyDQuat {
        #[inline]
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            let q = zelf.0;
            Ok(format!("Quat({}, {}, {}, {})", q.x, q.y, q.z, q.w))
        }
    }

    impl Comparable for PyDQuat {
        fn cmp(
            zelf: &Py<Self>,
            other: &PyObject,
            op: PyComparisonOp,
            _vm: &VirtualMachine,
        ) -> PyResult<rustpython_vm::function::PyComparisonValue> {
            op.eq_only(|| match other.downcast_ref::<PyDQuat>() {
                Some(o) => Ok(rustpython_vm::function::PyComparisonValue::Implemented(
                    zelf.0 == o.0,
                )),
                None => Ok(rustpython_vm::function::PyComparisonValue::NotImplemented),
            })
        }
    }

    impl Hashable for PyDQuat {
        fn hash(
            zelf: &Py<Self>,
            _vm: &VirtualMachine,
        ) -> PyResult<rustpython_vm::common::hash::PyHash> {
            use std::hash::{Hash, Hasher};
            let mut h = std::collections::hash_map::DefaultHasher::new();
            for c in zelf.0.to_array() {
                c.to_bits().hash(&mut h);
            }
            Ok(h.finish() as rustpython_vm::common::hash::PyHash)
        }
    }

    fn quat_mul(a: &PyObject, b: &PyObject, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        if let (Some(x), Some(y)) = (a.downcast_ref::<PyDQuat>(), b.downcast_ref::<PyDQuat>()) {
            return Ok(PyDQuat(x.0 * y.0).into_pyobject(vm));
        }
        if let (Some(x), Some(v)) = (a.downcast_ref::<PyDQuat>(), b.downcast_ref::<PyDVec3>()) {
            return Ok(PyDVec3(x.0 * v.0).into_pyobject(vm));
        }
        if let Some(x) = a.downcast_ref::<PyDQuat>()
            && let Ok(s) = f64::try_from_object(vm, b.to_owned())
        {
            return Ok(PyDQuat(x.0 * s).into_pyobject(vm));
        }
        if let Some(x) = b.downcast_ref::<PyDQuat>()
            && let Ok(s) = f64::try_from_object(vm, a.to_owned())
        {
            return Ok(PyDQuat(x.0 * s).into_pyobject(vm));
        }
        Ok(vm.ctx.not_implemented())
    }

    fn quat_add(a: &PyObject, b: &PyObject, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        if let (Some(x), Some(y)) = (a.downcast_ref::<PyDQuat>(), b.downcast_ref::<PyDQuat>()) {
            return Ok(PyDQuat(x.0 + y.0).into_pyobject(vm));
        }
        Ok(vm.ctx.not_implemented())
    }

    fn quat_sub(a: &PyObject, b: &PyObject, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        if let (Some(x), Some(y)) = (a.downcast_ref::<PyDQuat>(), b.downcast_ref::<PyDQuat>()) {
            return Ok(PyDQuat(x.0 - y.0).into_pyobject(vm));
        }
        Ok(vm.ctx.not_implemented())
    }

    fn quat_div(a: &PyObject, b: &PyObject, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        if let Some(x) = a.downcast_ref::<PyDQuat>()
            && let Ok(s) = f64::try_from_object(vm, b.to_owned())
        {
            return Ok(PyDQuat(x.0 / s).into_pyobject(vm));
        }
        Ok(vm.ctx.not_implemented())
    }

    impl AsNumber for PyDQuat {
        fn as_number() -> &'static rustpython_vm::protocol::PyNumberMethods {
            static N: rustpython_vm::protocol::PyNumberMethods =
                rustpython_vm::protocol::PyNumberMethods {
                    add: Some(quat_add),
                    subtract: Some(quat_sub),
                    multiply: Some(quat_mul),
                    true_divide: Some(quat_div),
                    negative: Some(|num, vm| {
                        let z = <PyDQuat as AsNumber>::number_downcast(num);
                        Ok(PyDQuat(-z.0).into_pyobject(vm))
                    }),
                    ..rustpython_vm::protocol::PyNumberMethods::NOT_IMPLEMENTED
                };
            &N
        }
    }

    #[pyclass(with(Constructor, Representable, AsNumber, Comparable, Hashable))]
    impl PyDQuat {
        #[pygetset]
        fn x(&self) -> f64 {
            self.0.x
        }
        #[pygetset]
        fn y(&self) -> f64 {
            self.0.y
        }
        #[pygetset]
        fn z(&self) -> f64 {
            self.0.z
        }
        #[pygetset]
        fn w(&self) -> f64 {
            self.0.w
        }

        #[pystaticmethod]
        fn from_xyzw(x: f64, y: f64, z: f64, w: f64) -> Self {
            Self(DQuat::from_xyzw(x, y, z, w))
        }
        #[pystaticmethod]
        fn from_array(obj: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let xs: Vec<f64> = obj.try_to_value(vm)?;
            if xs.len() != 4 {
                return Err(vm.new_value_error("Quat.from_array expects 4 elements".to_owned()));
            }
            Ok(Self(DQuat::from_array([xs[0], xs[1], xs[2], xs[3]])))
        }
        #[pystaticmethod]
        fn from_numpy(obj: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DQuat::from_array(
                crate::glam_wrappers::extract_numpy_vector_rp::<4>(&obj, "Quat", vm)?,
            )))
        }
        #[pymethod]
        fn to_numpy(&self, vm: &VirtualMachine) -> PyObjectRef {
            crate::glam_wrappers::pyndarray_from_slice(&self.0.to_array(), vm)
        }
        #[pymethod]
        fn __array__(&self, vm: &VirtualMachine) -> PyObjectRef {
            crate::glam_wrappers::pyndarray_from_slice(&self.0.to_array(), vm)
        }
        #[pystaticmethod]
        fn from_rotation_x(angle: f64) -> Self {
            Self(DQuat::from_rotation_x(angle))
        }
        #[pystaticmethod]
        fn from_rotation_y(angle: f64) -> Self {
            Self(DQuat::from_rotation_y(angle))
        }
        #[pystaticmethod]
        fn from_rotation_z(angle: f64) -> Self {
            Self(DQuat::from_rotation_z(angle))
        }
        #[pystaticmethod]
        fn from_euler(
            order: PyObjectRef,
            a: f64,
            b: f64,
            c: f64,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DQuat::from_euler(extract_euler(&order, vm)?, a, b, c)))
        }
        #[pystaticmethod]
        fn from_axis_angle(axis: PyObjectRef, angle: f64, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DQuat::from_axis_angle(
                extract_vec3(&axis, vm)?,
                angle,
            )))
        }
        #[pystaticmethod]
        fn from_scaled_axis(v: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DQuat::from_scaled_axis(extract_vec3(&v, vm)?)))
        }
        #[pystaticmethod]
        fn from_rotation_axes(
            x_axis: PyObjectRef,
            y_axis: PyObjectRef,
            z_axis: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DQuat::from_rotation_axes(
                extract_vec3(&x_axis, vm)?,
                extract_vec3(&y_axis, vm)?,
                extract_vec3(&z_axis, vm)?,
            )))
        }
        #[pystaticmethod]
        fn from_mat3(mat: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DQuat::from_mat3(&extract_mat3(&mat, vm)?)))
        }
        #[pystaticmethod]
        fn from_mat4(mat: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DQuat::from_mat4(&extract_mat4(&mat, vm)?)))
        }
        #[pystaticmethod]
        fn from_affine3(a: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DQuat::from_affine3(&extract_affine3(&a, vm)?)))
        }
        #[pystaticmethod]
        fn from_rotation_arc(
            from: PyObjectRef,
            to: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DQuat::from_rotation_arc(
                extract_vec3(&from, vm)?,
                extract_vec3(&to, vm)?,
            )))
        }
        #[pystaticmethod]
        fn from_rotation_arc_colinear(
            from: PyObjectRef,
            to: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DQuat::from_rotation_arc_colinear(
                extract_vec3(&from, vm)?,
                extract_vec3(&to, vm)?,
            )))
        }
        #[pystaticmethod]
        fn from_rotation_arc_2d(
            from: PyObjectRef,
            to: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DQuat::from_rotation_arc_2d(
                extract_vec2(&from, vm)?,
                extract_vec2(&to, vm)?,
            )))
        }
        #[pystaticmethod]
        fn look_to_lh(dir: PyObjectRef, up: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DQuat::look_to_lh(
                extract_vec3(&dir, vm)?,
                extract_vec3(&up, vm)?,
            )))
        }
        #[pystaticmethod]
        fn look_to_rh(dir: PyObjectRef, up: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DQuat::look_to_rh(
                extract_vec3(&dir, vm)?,
                extract_vec3(&up, vm)?,
            )))
        }
        #[pystaticmethod]
        fn look_at_lh(
            eye: PyObjectRef,
            center: PyObjectRef,
            up: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DQuat::look_at_lh(
                extract_vec3(&eye, vm)?,
                extract_vec3(&center, vm)?,
                extract_vec3(&up, vm)?,
            )))
        }
        #[pystaticmethod]
        fn look_at_rh(
            eye: PyObjectRef,
            center: PyObjectRef,
            up: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DQuat::look_at_rh(
                extract_vec3(&eye, vm)?,
                extract_vec3(&center, vm)?,
                extract_vec3(&up, vm)?,
            )))
        }

        #[pymethod]
        fn conjugate(&self) -> Self {
            Self(self.0.conjugate())
        }
        #[pymethod]
        fn inverse(&self) -> Self {
            Self(self.0.inverse())
        }
        #[pymethod]
        fn dot(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(self.0.dot(extract(&rhs, vm)?))
        }
        #[pymethod]
        fn length(&self) -> f64 {
            self.0.length()
        }
        #[pymethod]
        fn length_squared(&self) -> f64 {
            self.0.length_squared()
        }
        #[pymethod]
        fn length_recip(&self) -> f64 {
            self.0.length_recip()
        }
        #[pymethod]
        fn normalize(&self) -> Self {
            Self(self.0.normalize())
        }
        #[pymethod]
        fn mul_vec3(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyDVec3> {
            Ok(PyDVec3(self.0.mul_vec3(extract_vec3(&rhs, vm)?)))
        }
        #[pymethod]
        fn mul_quat(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.mul_quat(extract(&rhs, vm)?)))
        }
        #[pymethod]
        fn lerp(&self, end: PyObjectRef, s: f64, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.lerp(extract(&end, vm)?, s)))
        }
        #[pymethod]
        fn slerp(&self, end: PyObjectRef, s: f64, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.slerp(extract(&end, vm)?, s)))
        }
        #[pymethod]
        fn angle_between(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(self.0.angle_between(extract(&rhs, vm)?))
        }
        #[pymethod]
        fn rotate_towards(
            &self,
            rhs: PyObjectRef,
            max_angle: f64,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(self.0.rotate_towards(extract(&rhs, vm)?, max_angle)))
        }
        #[pymethod]
        fn to_axis_angle(&self, vm: &VirtualMachine) -> PyObjectRef {
            let (axis, angle) = self.0.to_axis_angle();
            vm.ctx
                .new_tuple(vec![
                    PyDVec3(axis).into_pyobject(vm),
                    vm.ctx.new_float(angle).into(),
                ])
                .into()
        }
        #[pymethod]
        fn to_scaled_axis(&self) -> PyDVec3 {
            PyDVec3(self.0.to_scaled_axis())
        }
        #[pymethod]
        fn to_euler(&self, order: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            let (a, b, c) = self.0.to_euler(extract_euler(&order, vm)?);
            Ok(vm
                .ctx
                .new_tuple(vec![
                    vm.ctx.new_float(a).into(),
                    vm.ctx.new_float(b).into(),
                    vm.ctx.new_float(c).into(),
                ])
                .into())
        }
        #[pymethod]
        fn to_array(&self, vm: &VirtualMachine) -> PyObjectRef {
            let a = self.0.to_array();
            vm.ctx
                .new_tuple(vec![
                    vm.ctx.new_float(a[0]).into(),
                    vm.ctx.new_float(a[1]).into(),
                    vm.ctx.new_float(a[2]).into(),
                    vm.ctx.new_float(a[3]).into(),
                ])
                .into()
        }
        #[pymethod]
        fn xyz(&self) -> PyDVec3 {
            PyDVec3(self.0.xyz())
        }
        #[pymethod]
        fn is_finite(&self) -> bool {
            self.0.is_finite()
        }
        #[pymethod]
        fn is_nan(&self) -> bool {
            self.0.is_nan()
        }
        #[pymethod]
        fn is_normalized(&self) -> bool {
            self.0.is_normalized()
        }
        #[pymethod]
        fn is_near_identity(&self) -> bool {
            self.0.is_near_identity()
        }
        #[pymethod]
        fn abs_diff_eq(
            &self,
            rhs: PyObjectRef,
            max_abs_diff: f64,
            vm: &VirtualMachine,
        ) -> PyResult<bool> {
            Ok(self.0.abs_diff_eq(extract(&rhs, vm)?, max_abs_diff))
        }

        #[pymethod(name = "__str__")]
        fn str(&self) -> String {
            format!("[{}, {}, {}, {}]", self.0.x, self.0.y, self.0.z, self.0.w)
        }

        #[pymethod]
        fn to_json(&self, vm: &VirtualMachine) -> PyResult<String> {
            crate::rp_serde::to_json(&self.0, vm)
        }
        #[pystaticmethod]
        fn from_json(s: String, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(crate::rp_serde::from_json::<DQuat>(&s, vm)?))
        }
        #[pystaticmethod]
        fn try_from_json(s: String) -> Option<Self> {
            crate::rp_serde::try_from_json::<DQuat>(&s).map(Self)
        }
        #[pymethod]
        fn to_dict(&self, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            crate::rp_serde::to_dict(&self.0, vm)
        }
        #[pystaticmethod]
        fn from_dict(obj: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(crate::rp_serde::from_dict::<DQuat>(&obj, vm)?))
        }
        #[pystaticmethod]
        fn try_from_dict(obj: PyObjectRef, vm: &VirtualMachine) -> Option<Self> {
            crate::rp_serde::try_from_dict::<DQuat>(&obj, vm).map(Self)
        }

        #[pymethod]
        fn __getnewargs_ex__(&self, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            crate::rp_serde::getnewargs_ex(&self.0, vm)
        }

        #[pygetset]
        fn __dataclass_fields__(&self, vm: &VirtualMachine) -> PyObjectRef {
            crate::rp_serde::dataclass_fields(&["x", "y", "z", "w"], vm)
        }
    }

    pub(crate) fn install_constants(typ: &rustpython_vm::builtins::PyTypeRef, vm: &VirtualMachine) {
        let set = |name: &str, v: DQuat| {
            typ.set_attr(vm.ctx.intern_str(name), PyDQuat(v).into_pyobject(vm));
        };
        set("IDENTITY", DQuat::IDENTITY);
        set("NAN", DQuat::NAN);
    }
}

#[cfg(feature = "rustpython-backend")]
pub(crate) use rustpython_impl::extract as extract_quat;

#[cfg(feature = "rustpython-backend")]
pub(crate) use rustpython_impl::install_constants;
