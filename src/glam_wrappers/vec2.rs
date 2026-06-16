//! `Vec2` (double-precision) wrapper.

use glam::DVec2;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, skip_from_py_object, name = "Vec2")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Vec2")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PyDVec2(pub DVec2);

impl From<DVec2> for PyDVec2 {
    #[inline]
    fn from(v: DVec2) -> Self {
        Self(v)
    }
}

impl From<PyDVec2> for DVec2 {
    #[inline]
    fn from(v: PyDVec2) -> Self {
        v.0
    }
}

// =============================================================================
// PyO3 backend
// =============================================================================

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::{PyDVec3, extract_numpy_vector, impl_vec_constants, impl_vec_unary};
    use crate::pickle::pickle_decode;
    use crate::{impl_dataclass_fields, impl_getnewargs_ex};
    use numpy::{AllowTypeChange, PyArray1, PyArrayLike1};
    use pyo3::exceptions::PyIndexError;
    use pyo3::prelude::*;

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyDVec2 {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(*v.get());
            }
            let x: f64 = ob.getattr("x")?.extract()?;
            let y: f64 = ob.getattr("y")?.extract()?;
            Ok(Self(DVec2::new(x, y)))
        }
    }

    #[pymethods]
    impl PyDVec2 {
        #[new]
        #[pyo3(signature = (x=0.0, y=0.0, *, __pickle_state__=None))]
        #[inline]
        fn new(x: f64, y: f64, __pickle_state__: Option<Vec<u8>>) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<DVec2>(&state)?));
            }
            Ok(Self(DVec2::new(x, y)))
        }

        #[staticmethod]
        #[inline]
        fn splat(v: f64) -> Self {
            Self(DVec2::splat(v))
        }

        #[staticmethod]
        #[inline]
        fn from_array(a: [f64; 2]) -> Self {
            Self(DVec2::from_array(a))
        }

        #[staticmethod]
        #[inline]
        fn from_numpy(array: PyArrayLike1<'_, f64, AllowTypeChange>) -> PyResult<Self> {
            Ok(Self(DVec2::from_array(extract_numpy_vector::<2>(
                array, "Vec2",
            )?)))
        }

        #[inline]
        fn to_array(&self) -> [f64; 2] {
            self.0.to_array()
        }
        #[inline]
        fn to_list(&self) -> Vec<f64> {
            self.0.to_array().to_vec()
        }
        #[inline]
        fn to_numpy<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
            PyArray1::from_slice(py, &self.0.to_array())
        }
        #[inline]
        fn extend(&self, z: f64) -> PyDVec3 {
            PyDVec3(self.0.extend(z))
        }
    }

    #[pymethods]
    impl PyDVec2 {
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
        #[inline]
        fn with_x(&self, x: f64) -> Self {
            Self(self.0.with_x(x))
        }
        #[inline]
        fn with_y(&self, y: f64) -> Self {
            Self(self.0.with_y(y))
        }
    }

    #[pymethods]
    impl PyDVec2 {
        #[inline]
        fn dot(&self, rhs: Self) -> f64 {
            self.0.dot(rhs.0)
        }
        #[inline]
        fn perp(&self) -> Self {
            Self(self.0.perp())
        }
        #[inline]
        fn perp_dot(&self, rhs: Self) -> f64 {
            self.0.perp_dot(rhs.0)
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
        fn distance(&self, rhs: Self) -> f64 {
            self.0.distance(rhs.0)
        }
        #[inline]
        fn distance_squared(&self, rhs: Self) -> f64 {
            self.0.distance_squared(rhs.0)
        }
        #[inline]
        fn normalize(&self) -> Self {
            Self(self.0.normalize())
        }
        #[inline]
        fn try_normalize(&self) -> Option<Self> {
            self.0.try_normalize().map(Self)
        }
        #[inline]
        fn normalize_or(&self, fallback: Self) -> Self {
            Self(self.0.normalize_or(fallback.0))
        }
        #[inline]
        fn normalize_and_length(&self) -> (Self, f64) {
            let (v, l) = self.0.normalize_and_length();
            (Self(v), l)
        }
        #[inline]
        fn is_normalized(&self) -> bool {
            self.0.is_normalized()
        }
        #[inline]
        fn rotate(&self, rhs: Self) -> Self {
            Self(self.0.rotate(rhs.0))
        }
    }

    #[pymethods]
    impl PyDVec2 {
        #[inline]
        fn project_onto(&self, rhs: Self) -> Self {
            Self(self.0.project_onto(rhs.0))
        }
        #[inline]
        fn reject_from(&self, rhs: Self) -> Self {
            Self(self.0.reject_from(rhs.0))
        }
        #[inline]
        fn project_onto_normalized(&self, rhs: Self) -> Self {
            Self(self.0.project_onto_normalized(rhs.0))
        }
        #[inline]
        fn reject_from_normalized(&self, rhs: Self) -> Self {
            Self(self.0.reject_from_normalized(rhs.0))
        }
        #[inline]
        fn reflect(&self, normal: Self) -> Self {
            Self(self.0.reflect(normal.0))
        }
        #[inline]
        fn refract(&self, normal: Self, eta: f64) -> Self {
            Self(self.0.refract(normal.0, eta))
        }
    }

    #[pymethods]
    impl PyDVec2 {
        #[inline]
        fn lerp(&self, rhs: Self, s: f64) -> Self {
            Self(self.0.lerp(rhs.0, s))
        }
        #[inline]
        fn move_towards(&self, rhs: Self, d: f64) -> Self {
            Self(self.0.move_towards(rhs.0, d))
        }
        #[inline]
        fn midpoint(&self, rhs: Self) -> Self {
            Self(self.0.midpoint(rhs.0))
        }
    }

    #[pymethods]
    impl PyDVec2 {
        #[inline]
        fn min(&self, rhs: Self) -> Self {
            Self(self.0.min(rhs.0))
        }
        #[inline]
        fn max(&self, rhs: Self) -> Self {
            Self(self.0.max(rhs.0))
        }
        #[inline]
        fn clamp(&self, min: Self, max: Self) -> Self {
            Self(self.0.clamp(min.0, max.0))
        }
        #[inline]
        fn min_element(&self) -> f64 {
            self.0.min_element()
        }
        #[inline]
        fn max_element(&self) -> f64 {
            self.0.max_element()
        }
        #[inline]
        fn min_position(&self) -> usize {
            self.0.min_position()
        }
        #[inline]
        fn max_position(&self) -> usize {
            self.0.max_position()
        }
        #[inline]
        fn clamp_length(&self, min: f64, max: f64) -> Self {
            Self(self.0.clamp_length(min, max))
        }
        #[inline]
        fn clamp_length_max(&self, max: f64) -> Self {
            Self(self.0.clamp_length_max(max))
        }
        #[inline]
        fn clamp_length_min(&self, min: f64) -> Self {
            Self(self.0.clamp_length_min(min))
        }
    }

    #[pymethods]
    impl PyDVec2 {
        #[inline]
        fn element_sum(&self) -> f64 {
            self.0.element_sum()
        }
        #[inline]
        fn element_product(&self) -> f64 {
            self.0.element_product()
        }
    }

    #[pymethods]
    impl PyDVec2 {
        #[inline]
        fn copysign(&self, rhs: Self) -> Self {
            Self(self.0.copysign(rhs.0))
        }
        #[inline]
        fn powf(&self, n: f64) -> Self {
            Self(self.0.powf(n))
        }
        #[inline]
        fn sin_cos(&self) -> (Self, Self) {
            let (s, c) = self.0.sin_cos();
            (Self(s), Self(c))
        }
        #[inline]
        fn mul_add(&self, a: Self, b: Self) -> Self {
            Self(self.0.mul_add(a.0, b.0))
        }
        #[inline]
        fn step(&self, rhs: Self) -> Self {
            Self(self.0.step(rhs.0))
        }
        #[inline]
        fn div_euclid(&self, rhs: Self) -> Self {
            Self(self.0.div_euclid(rhs.0))
        }
        #[inline]
        fn rem_euclid(&self, rhs: Self) -> Self {
            Self(self.0.rem_euclid(rhs.0))
        }
        #[inline]
        fn angle_to(&self, rhs: Self) -> f64 {
            self.0.angle_to(rhs.0)
        }
        #[inline]
        fn rotate_towards(&self, rhs: Self, max_angle: f64) -> Self {
            Self(self.0.rotate_towards(rhs.0, max_angle))
        }
    }

    #[pymethods]
    impl PyDVec2 {
        #[inline]
        fn is_finite(&self) -> bool {
            self.0.is_finite()
        }
        #[inline]
        fn is_nan(&self) -> bool {
            self.0.is_nan()
        }
        #[inline]
        fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f64) -> bool {
            self.0.abs_diff_eq(rhs.0, max_abs_diff)
        }
    }

    impl_vec_unary!(
        PyDVec2,
        [
            abs,
            signum,
            floor,
            ceil,
            round,
            trunc,
            fract,
            fract_gl,
            exp,
            exp2,
            ln,
            log2,
            sqrt,
            recip,
            cos,
            sin,
            normalize_or_zero,
            saturate,
        ]
    );

    impl_vec_constants!(
        PyDVec2,
        DVec2,
        [
            (zero, ZERO, "ZERO"),
            (one, ONE, "ONE"),
            (neg_one, NEG_ONE, "NEG_ONE"),
            (unit_x, X, "X"),
            (unit_y, Y, "Y"),
            (neg_x, NEG_X, "NEG_X"),
            (neg_y, NEG_Y, "NEG_Y"),
            (infinity, INFINITY, "INFINITY"),
            (neg_infinity, NEG_INFINITY, "NEG_INFINITY"),
            (nan, NAN, "NAN"),
        ]
    );

    #[pymethods]
    impl PyDVec2 {
        fn __repr__(&self) -> String {
            format!("Vec2({}, {})", self.0.x, self.0.y)
        }
        fn __str__(&self) -> String {
            format!("[{}, {}]", self.0.x, self.0.y)
        }
        fn __len__(&self) -> usize {
            2
        }
        fn __getitem__(&self, idx: isize) -> PyResult<f64> {
            let i = if idx < 0 { 2 + idx } else { idx };
            match i {
                0 => Ok(self.0.x),
                1 => Ok(self.0.y),
                _ => Err(PyIndexError::new_err("index out of range")),
            }
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
        fn __pos__(&self) -> Self {
            *self
        }

        fn __add__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
            if let Ok(v) = other.extract::<Self>() {
                Ok(Self(self.0 + v.0))
            } else if let Ok(s) = other.extract::<f64>() {
                Ok(Self(self.0 + s))
            } else {
                Err(pyo3::exceptions::PyTypeError::new_err(
                    "unsupported operand type for +",
                ))
            }
        }
        fn __radd__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
            self.__add__(other)
        }

        fn __sub__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
            if let Ok(v) = other.extract::<Self>() {
                Ok(Self(self.0 - v.0))
            } else if let Ok(s) = other.extract::<f64>() {
                Ok(Self(self.0 - s))
            } else {
                Err(pyo3::exceptions::PyTypeError::new_err(
                    "unsupported operand type for -",
                ))
            }
        }
        fn __rsub__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
            if let Ok(v) = other.extract::<Self>() {
                Ok(Self(v.0 - self.0))
            } else if let Ok(s) = other.extract::<f64>() {
                Ok(Self(DVec2::splat(s) - self.0))
            } else {
                Err(pyo3::exceptions::PyTypeError::new_err(
                    "unsupported operand type for -",
                ))
            }
        }

        fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
            if let Ok(v) = other.extract::<Self>() {
                Ok(Self(self.0 * v.0))
            } else if let Ok(s) = other.extract::<f64>() {
                Ok(Self(self.0 * s))
            } else {
                Err(pyo3::exceptions::PyTypeError::new_err(
                    "unsupported operand type for *",
                ))
            }
        }
        fn __rmul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
            self.__mul__(other)
        }

        fn __truediv__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
            if let Ok(v) = other.extract::<Self>() {
                Ok(Self(self.0 / v.0))
            } else if let Ok(s) = other.extract::<f64>() {
                Ok(Self(self.0 / s))
            } else {
                Err(pyo3::exceptions::PyTypeError::new_err(
                    "unsupported operand type for /",
                ))
            }
        }
        fn __rtruediv__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
            if let Ok(v) = other.extract::<Self>() {
                Ok(Self(v.0 / self.0))
            } else if let Ok(s) = other.extract::<f64>() {
                Ok(Self(DVec2::splat(s) / self.0))
            } else {
                Err(pyo3::exceptions::PyTypeError::new_err(
                    "unsupported operand type for /",
                ))
            }
        }

        fn __mod__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
            if let Ok(v) = other.extract::<Self>() {
                Ok(Self(self.0 % v.0))
            } else if let Ok(s) = other.extract::<f64>() {
                Ok(Self(self.0 % s))
            } else {
                Err(pyo3::exceptions::PyTypeError::new_err(
                    "unsupported operand type for %",
                ))
            }
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

    impl_getnewargs_ex!(PyDVec2);
    impl_dataclass_fields!(PyDVec2, ["x", "y"]);
}

// =============================================================================
// RustPython backend
// =============================================================================

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::glam_wrappers::PyDVec3;
    use crate::glam_wrappers::impl_rp_vec_ops;
    use rustpython_vm::{
        Py, PyObjectRef, PyPayload, PyResult, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        pyclass,
        types::{AsMapping, AsNumber, Comparable, Constructor, Hashable, Representable},
    };

    pub(crate) fn extract(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<DVec2> {
        if let Some(v) = obj.downcast_ref::<PyDVec2>() {
            return Ok(v.0);
        }
        if let Ok(xs) = obj.try_to_value::<Vec<f64>>(vm)
            && xs.len() == 2
        {
            return Ok(DVec2::new(xs[0], xs[1]));
        }
        let x: f64 = obj.get_attr("x", vm)?.try_float(vm)?.to_f64();
        let y: f64 = obj.get_attr("y", vm)?.try_float(vm)?.to_f64();
        Ok(DVec2::new(x, y))
    }

    impl Constructor for PyDVec2 {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<DVec2>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            let (x, y) = match args.args.len() {
                0 => (0.0, 0.0),
                2 => (
                    args.args[0].try_float(vm)?.to_f64(),
                    args.args[1].try_float(vm)?.to_f64(),
                ),
                n => {
                    return Err(vm.new_type_error(format!(
                        "Vec2() takes 0 or 2 positional arguments, got {n}"
                    )));
                }
            };
            Ok(PyDVec2(DVec2::new(x, y)))
        }
    }

    impl Representable for PyDVec2 {
        #[inline]
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            let v = zelf.0;
            Ok(format!("Vec2({}, {})", v.x, v.y))
        }
    }

    #[pyclass(with(Constructor, Representable, AsNumber, Comparable, Hashable, AsMapping))]
    impl PyDVec2 {
        #[pygetset]
        fn x(&self) -> f64 {
            self.0.x
        }
        #[pygetset]
        fn y(&self) -> f64 {
            self.0.y
        }

        #[pymethod]
        fn with_x(&self, x: f64) -> Self {
            Self(self.0.with_x(x))
        }
        #[pymethod]
        fn with_y(&self, y: f64) -> Self {
            Self(self.0.with_y(y))
        }

        #[pystaticmethod]
        fn splat(v: f64) -> Self {
            Self(DVec2::splat(v))
        }
        #[pystaticmethod]
        fn from_array(obj: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let xs: Vec<f64> = obj.try_to_value(vm)?;
            if xs.len() != 2 {
                return Err(vm.new_value_error("Vec2.from_array expects 2 elements".to_owned()));
            }
            Ok(Self(DVec2::new(xs[0], xs[1])))
        }
        #[pystaticmethod]
        fn from_numpy(obj: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DVec2::from_array(
                crate::glam_wrappers::extract_numpy_vector_rp::<2>(&obj, "Vec2", vm)?,
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

        #[pymethod]
        fn to_array(&self, vm: &VirtualMachine) -> PyObjectRef {
            vm.ctx
                .new_tuple(vec![
                    vm.ctx.new_float(self.0.x).into(),
                    vm.ctx.new_float(self.0.y).into(),
                ])
                .into()
        }
        #[pymethod]
        fn to_list(&self, vm: &VirtualMachine) -> PyObjectRef {
            vm.ctx
                .new_list(vec![
                    vm.ctx.new_float(self.0.x).into(),
                    vm.ctx.new_float(self.0.y).into(),
                ])
                .into()
        }
        #[pymethod]
        fn extend(&self, z: f64) -> PyDVec3 {
            PyDVec3(self.0.extend(z))
        }

        #[pymethod]
        fn dot(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(self.0.dot(extract(&rhs, vm)?))
        }
        #[pymethod]
        fn perp(&self) -> Self {
            Self(self.0.perp())
        }
        #[pymethod]
        fn perp_dot(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(self.0.perp_dot(extract(&rhs, vm)?))
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
        fn distance(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(self.0.distance(extract(&rhs, vm)?))
        }
        #[pymethod]
        fn distance_squared(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(self.0.distance_squared(extract(&rhs, vm)?))
        }
        #[pymethod]
        fn normalize(&self) -> Self {
            Self(self.0.normalize())
        }
        #[pymethod]
        fn normalize_or_zero(&self) -> Self {
            Self(self.0.normalize_or_zero())
        }
        #[pymethod]
        fn normalize_or(&self, fallback: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.normalize_or(extract(&fallback, vm)?)))
        }
        #[pymethod]
        fn try_normalize(&self) -> Option<Self> {
            self.0.try_normalize().map(Self)
        }
        #[pymethod]
        fn normalize_and_length(&self) -> (Self, f64) {
            let (v, l) = self.0.normalize_and_length();
            (Self(v), l)
        }
        #[pymethod]
        fn is_normalized(&self) -> bool {
            self.0.is_normalized()
        }
        #[pymethod]
        fn rotate(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.rotate(extract(&rhs, vm)?)))
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
        fn project_onto(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.project_onto(extract(&rhs, vm)?)))
        }
        #[pymethod]
        fn project_onto_normalized(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.project_onto_normalized(extract(&rhs, vm)?)))
        }
        #[pymethod]
        fn reject_from(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.reject_from(extract(&rhs, vm)?)))
        }
        #[pymethod]
        fn reject_from_normalized(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.reject_from_normalized(extract(&rhs, vm)?)))
        }
        #[pymethod]
        fn reflect(&self, normal: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.reflect(extract(&normal, vm)?)))
        }
        #[pymethod]
        fn refract(&self, normal: PyObjectRef, eta: f64, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.refract(extract(&normal, vm)?, eta)))
        }
        #[pymethod]
        fn lerp(&self, rhs: PyObjectRef, s: f64, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.lerp(extract(&rhs, vm)?, s)))
        }
        #[pymethod]
        fn move_towards(&self, rhs: PyObjectRef, d: f64, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.move_towards(extract(&rhs, vm)?, d)))
        }
        #[pymethod]
        fn midpoint(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.midpoint(extract(&rhs, vm)?)))
        }
        #[pymethod]
        fn min(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.min(extract(&rhs, vm)?)))
        }
        #[pymethod]
        fn max(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.max(extract(&rhs, vm)?)))
        }
        #[pymethod]
        fn clamp(&self, min: PyObjectRef, max: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.clamp(extract(&min, vm)?, extract(&max, vm)?)))
        }
        #[pymethod]
        fn min_element(&self) -> f64 {
            self.0.min_element()
        }
        #[pymethod]
        fn max_element(&self) -> f64 {
            self.0.max_element()
        }
        #[pymethod]
        fn min_position(&self) -> usize {
            self.0.min_position()
        }
        #[pymethod]
        fn max_position(&self) -> usize {
            self.0.max_position()
        }
        #[pymethod]
        fn clamp_length(&self, min: f64, max: f64) -> Self {
            Self(self.0.clamp_length(min, max))
        }
        #[pymethod]
        fn clamp_length_max(&self, max: f64) -> Self {
            Self(self.0.clamp_length_max(max))
        }
        #[pymethod]
        fn clamp_length_min(&self, min: f64) -> Self {
            Self(self.0.clamp_length_min(min))
        }
        #[pymethod]
        fn element_sum(&self) -> f64 {
            self.0.element_sum()
        }
        #[pymethod]
        fn element_product(&self) -> f64 {
            self.0.element_product()
        }
        #[pymethod]
        fn copysign(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.copysign(extract(&rhs, vm)?)))
        }
        #[pymethod]
        fn mul_add(&self, a: PyObjectRef, b: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.mul_add(extract(&a, vm)?, extract(&b, vm)?)))
        }
        #[pymethod]
        fn step(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.step(extract(&rhs, vm)?)))
        }
        #[pymethod]
        fn div_euclid(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.div_euclid(extract(&rhs, vm)?)))
        }
        #[pymethod]
        fn rem_euclid(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.rem_euclid(extract(&rhs, vm)?)))
        }
        #[pymethod]
        fn sin_cos(&self) -> (Self, Self) {
            let (s, c) = self.0.sin_cos();
            (Self(s), Self(c))
        }
        #[pymethod]
        fn angle_to(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(self.0.angle_to(extract(&rhs, vm)?))
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
        fn abs_diff_eq(
            &self,
            rhs: PyObjectRef,
            max_abs_diff: f64,
            vm: &VirtualMachine,
        ) -> PyResult<bool> {
            Ok(self.0.abs_diff_eq(extract(&rhs, vm)?, max_abs_diff))
        }
        #[pymethod]
        fn abs(&self) -> Self {
            Self(self.0.abs())
        }
        #[pymethod]
        fn signum(&self) -> Self {
            Self(self.0.signum())
        }
        #[pymethod]
        fn floor(&self) -> Self {
            Self(self.0.floor())
        }
        #[pymethod]
        fn ceil(&self) -> Self {
            Self(self.0.ceil())
        }
        #[pymethod]
        fn round(&self) -> Self {
            Self(self.0.round())
        }
        #[pymethod]
        fn trunc(&self) -> Self {
            Self(self.0.trunc())
        }
        #[pymethod]
        fn fract(&self) -> Self {
            Self(self.0.fract())
        }
        #[pymethod]
        fn fract_gl(&self) -> Self {
            Self(self.0.fract_gl())
        }
        #[pymethod]
        fn exp(&self) -> Self {
            Self(self.0.exp())
        }
        #[pymethod]
        fn exp2(&self) -> Self {
            Self(self.0.exp2())
        }
        #[pymethod]
        fn ln(&self) -> Self {
            Self(self.0.ln())
        }
        #[pymethod]
        fn log2(&self) -> Self {
            Self(self.0.log2())
        }
        #[pymethod]
        fn powf(&self, n: f64) -> Self {
            Self(self.0.powf(n))
        }
        #[pymethod]
        fn recip(&self) -> Self {
            Self(self.0.recip())
        }
        #[pymethod]
        fn sqrt(&self) -> Self {
            Self(self.0.sqrt())
        }
        #[pymethod]
        fn cos(&self) -> Self {
            Self(self.0.cos())
        }
        #[pymethod]
        fn sin(&self) -> Self {
            Self(self.0.sin())
        }
        #[pymethod]
        fn saturate(&self) -> Self {
            Self(self.0.saturate())
        }

        #[pymethod(name = "__str__")]
        fn str(&self) -> String {
            format!("[{}, {}]", self.0.x, self.0.y)
        }
        #[pymethod]
        fn __getnewargs_ex__(&self, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            crate::rp_serde::getnewargs_ex(&self.0, vm)
        }

        #[pygetset]
        fn __dataclass_fields__(&self, vm: &VirtualMachine) -> PyObjectRef {
            crate::rp_serde::dataclass_fields(&["x", "y"], vm)
        }
    }

    /// Install class-level vector constants as attributes on the (already
    /// initialized) static type. Done post-creation because a constant whose
    /// value is an instance of the class can't be built during the class's own
    /// initialization.
    pub(crate) fn install_constants(typ: &rustpython_vm::builtins::PyTypeRef, vm: &VirtualMachine) {
        let set = |name: &str, v: DVec2| {
            typ.set_attr(vm.ctx.intern_str(name), PyDVec2(v).into_pyobject(vm));
        };
        set("ZERO", DVec2::ZERO);
        set("ONE", DVec2::ONE);
        set("NEG_ONE", DVec2::NEG_ONE);
        set("X", DVec2::X);
        set("Y", DVec2::Y);
        set("NEG_X", DVec2::NEG_X);
        set("NEG_Y", DVec2::NEG_Y);
        set("INFINITY", DVec2::INFINITY);
        set("NEG_INFINITY", DVec2::NEG_INFINITY);
        set("NAN", DVec2::NAN);
    }

    impl_rp_vec_ops!(PyDVec2, DVec2, 2);
}

#[cfg(feature = "rustpython-backend")]
pub(crate) use rustpython_impl::install_constants;

#[cfg(feature = "rustpython-backend")]
pub(crate) use rustpython_impl::extract as extract_vec2;
