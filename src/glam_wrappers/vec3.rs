//! `Vec3` (double-precision) wrapper.
//!
//! The struct is shared across both backends via `cfg_attr`; method impls are
//! cfg-gated per backend.

use glam::DVec3;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, skip_from_py_object, name = "Vec3")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Vec3")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PyDVec3(pub(crate) DVec3);

impl From<DVec3> for PyDVec3 {
    #[inline]
    fn from(v: DVec3) -> Self {
        Self(v)
    }
}

impl From<PyDVec3> for DVec3 {
    #[inline]
    fn from(v: PyDVec3) -> Self {
        v.0
    }
}

// =============================================================================
// PyO3 backend
// =============================================================================

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::{
        PyDVec2, PyDVec4, extract_numpy_vector, impl_serde_methods, impl_vec_constants,
        impl_vec_unary,
    };
    use crate::pickle::pickle_decode;
    use crate::{impl_dataclass_fields, impl_getnewargs_ex};
    use numpy::{AllowTypeChange, PyArray1, PyArrayLike1};
    use pyo3::exceptions::PyIndexError;
    use pyo3::prelude::*;

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyDVec3 {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(*v.get());
            }
            if let Ok(xs) = ob.extract::<[f64; 3]>() {
                return Ok(Self(DVec3::new(xs[0], xs[1], xs[2])));
            }
            let py = ob.py();
            let x: f64 = ob.getattr(pyo3::intern!(py, "x"))?.extract()?;
            let y: f64 = ob.getattr(pyo3::intern!(py, "y"))?.extract()?;
            let z: f64 = ob.getattr(pyo3::intern!(py, "z"))?.extract()?;
            Ok(Self(DVec3::new(x, y, z)))
        }
    }

    #[pymethods]
    impl PyDVec3 {
        #[new]
        #[pyo3(signature = (x=0.0, y=0.0, z=0.0, *, __pickle_state__=None))]
        #[inline]
        fn new(x: f64, y: f64, z: f64, __pickle_state__: Option<Vec<u8>>) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<DVec3>(&state)?));
            }
            Ok(Self(DVec3::new(x, y, z)))
        }

        #[staticmethod]
        #[inline]
        fn splat(v: f64) -> Self {
            Self(DVec3::splat(v))
        }

        #[staticmethod]
        #[inline]
        fn from_array(a: [f64; 3]) -> Self {
            Self(DVec3::from_array(a))
        }

        #[staticmethod]
        #[inline]
        fn from_numpy(array: PyArrayLike1<'_, f64, AllowTypeChange>) -> PyResult<Self> {
            Ok(Self(DVec3::from_array(extract_numpy_vector::<3>(
                array, "Vec3",
            )?)))
        }

        #[inline]
        fn to_array(&self) -> [f64; 3] {
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
        fn extend(&self, w: f64) -> PyDVec4 {
            PyDVec4(self.0.extend(w))
        }

        #[inline]
        fn truncate(&self) -> PyDVec2 {
            PyDVec2(self.0.truncate())
        }

        #[inline]
        fn to_homogeneous(&self) -> PyDVec4 {
            PyDVec4(self.0.to_homogeneous())
        }

        #[staticmethod]
        #[inline]
        fn from_homogeneous(v: PyDVec4) -> Self {
            Self(DVec3::from_homogeneous(v.0))
        }
    }

    #[pymethods]
    impl PyDVec3 {
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
        #[inline]
        fn with_x(&self, x: f64) -> Self {
            Self(self.0.with_x(x))
        }
        #[inline]
        fn with_y(&self, y: f64) -> Self {
            Self(self.0.with_y(y))
        }
        #[inline]
        fn with_z(&self, z: f64) -> Self {
            Self(self.0.with_z(z))
        }
    }

    #[pymethods]
    impl PyDVec3 {
        #[inline]
        fn dot(&self, rhs: Self) -> f64 {
            self.0.dot(rhs.0)
        }
        #[inline]
        fn cross(&self, rhs: Self) -> Self {
            Self(self.0.cross(rhs.0))
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
    }

    #[pymethods]
    impl PyDVec3 {
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
    impl PyDVec3 {
        #[inline]
        fn lerp(&self, rhs: Self, s: f64) -> Self {
            Self(self.0.lerp(rhs.0, s))
        }
        #[inline]
        fn slerp(&self, rhs: Self, s: f64) -> Self {
            Self(self.0.slerp(rhs.0, s))
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
    impl PyDVec3 {
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
    impl PyDVec3 {
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
    impl PyDVec3 {
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
    }

    #[pymethods]
    impl PyDVec3 {
        #[inline]
        fn angle_between(&self, rhs: Self) -> f64 {
            self.0.angle_between(rhs.0)
        }
        #[inline]
        fn rotate_x(&self, angle: f64) -> Self {
            Self(self.0.rotate_x(angle))
        }
        #[inline]
        fn rotate_y(&self, angle: f64) -> Self {
            Self(self.0.rotate_y(angle))
        }
        #[inline]
        fn rotate_z(&self, angle: f64) -> Self {
            Self(self.0.rotate_z(angle))
        }
        #[inline]
        fn rotate_axis(&self, axis: Self, angle: f64) -> Self {
            Self(self.0.rotate_axis(axis.0, angle))
        }
        #[inline]
        fn rotate_towards(&self, rhs: Self, max_angle: f64) -> Self {
            Self(self.0.rotate_towards(rhs.0, max_angle))
        }
    }

    #[pymethods]
    impl PyDVec3 {
        #[inline]
        fn any_orthogonal_vector(&self) -> Self {
            Self(self.0.any_orthogonal_vector())
        }
        #[inline]
        fn any_orthonormal_vector(&self) -> Self {
            Self(self.0.any_orthonormal_vector())
        }
        #[inline]
        fn any_orthonormal_pair(&self) -> (Self, Self) {
            let (a, b) = self.0.any_orthonormal_pair();
            (Self(a), Self(b))
        }
    }

    #[pymethods]
    impl PyDVec3 {
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
        PyDVec3,
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
        PyDVec3,
        DVec3,
        [
            (zero, ZERO, "ZERO"),
            (one, ONE, "ONE"),
            (neg_one, NEG_ONE, "NEG_ONE"),
            (unit_x, X, "X"),
            (unit_y, Y, "Y"),
            (unit_z, Z, "Z"),
            (neg_x, NEG_X, "NEG_X"),
            (neg_y, NEG_Y, "NEG_Y"),
            (neg_z, NEG_Z, "NEG_Z"),
            (infinity, INFINITY, "INFINITY"),
            (neg_infinity, NEG_INFINITY, "NEG_INFINITY"),
            (nan, NAN, "NAN"),
        ]
    );

    #[pymethods]
    impl PyDVec3 {
        fn __repr__(&self) -> String {
            format!("Vec3({}, {}, {})", self.0.x, self.0.y, self.0.z)
        }

        fn __str__(&self) -> String {
            format!("[{}, {}, {}]", self.0.x, self.0.y, self.0.z)
        }

        fn __len__(&self) -> usize {
            3
        }

        fn __getitem__(&self, idx: isize) -> PyResult<f64> {
            let i = if idx < 0 { 3 + idx } else { idx };
            match i {
                0 => Ok(self.0.x),
                1 => Ok(self.0.y),
                2 => Ok(self.0.z),
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
                Ok(Self(DVec3::splat(s) - self.0))
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
                Ok(Self(DVec3::splat(s) / self.0))
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

    impl_serde_methods!(PyDVec3, DVec3);
    impl_getnewargs_ex!(PyDVec3);
    impl_dataclass_fields!(PyDVec3, ["x", "y", "z"]);
}

// =============================================================================
// RustPython backend
// =============================================================================

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::glam_wrappers::impl_rp_vec_ops;
    use crate::glam_wrappers::vec4::extract_vec4;
    use crate::glam_wrappers::{PyDVec2, PyDVec4};
    use rustpython_vm::{
        Py, PyObjectRef, PyPayload, PyResult, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        pyclass,
        types::{AsMapping, AsNumber, Comparable, Constructor, Hashable, Representable},
    };

    /// Pull a `DVec3` out of any Python object: another `Vec3`, a 3-tuple/list
    /// of floats, or anything with `x`/`y`/`z` attributes.
    pub(crate) fn extract(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<DVec3> {
        if let Some(v) = obj.downcast_ref::<PyDVec3>() {
            return Ok(v.0);
        }
        if let Ok(xs) = obj.try_to_value::<Vec<f64>>(vm)
            && xs.len() == 3
        {
            return Ok(DVec3::new(xs[0], xs[1], xs[2]));
        }
        let x: f64 = obj.get_attr("x", vm)?.try_float(vm)?.to_f64();
        let y: f64 = obj.get_attr("y", vm)?.try_float(vm)?.to_f64();
        let z: f64 = obj.get_attr("z", vm)?.try_float(vm)?.to_f64();
        Ok(DVec3::new(x, y, z))
    }

    impl Constructor for PyDVec3 {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<DVec3>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            let (x, y, z) = match args.args.len() {
                0 => (0.0, 0.0, 0.0),
                3 => (
                    args.args[0].try_float(vm)?.to_f64(),
                    args.args[1].try_float(vm)?.to_f64(),
                    args.args[2].try_float(vm)?.to_f64(),
                ),
                n => {
                    return Err(vm.new_type_error(format!(
                        "Vec3() takes 0 or 3 positional arguments, got {n}"
                    )));
                }
            };
            Ok(PyDVec3(DVec3::new(x, y, z)))
        }
    }

    impl Representable for PyDVec3 {
        #[inline]
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            let v = zelf.0;
            Ok(format!("Vec3({}, {}, {})", v.x, v.y, v.z))
        }
    }

    #[pyclass(with(Constructor, Representable, AsNumber, Comparable, Hashable, AsMapping))]
    impl PyDVec3 {
        // Components
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

        #[pymethod]
        fn with_x(&self, x: f64) -> Self {
            Self(self.0.with_x(x))
        }
        #[pymethod]
        fn with_y(&self, y: f64) -> Self {
            Self(self.0.with_y(y))
        }
        #[pymethod]
        fn with_z(&self, z: f64) -> Self {
            Self(self.0.with_z(z))
        }

        // Constructors
        #[pystaticmethod]
        fn splat(v: f64) -> Self {
            Self(DVec3::splat(v))
        }
        #[pystaticmethod]
        fn from_array(a: Vec<f64>, vm: &VirtualMachine) -> PyResult<Self> {
            if a.len() != 3 {
                return Err(vm.new_value_error(format!("expected 3 elements, got {}", a.len())));
            }
            Ok(Self(DVec3::new(a[0], a[1], a[2])))
        }
        #[pystaticmethod]
        fn from_numpy(obj: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DVec3::from_array(
                crate::glam_wrappers::extract_numpy_vector_rp::<3>(&obj, "Vec3", vm)?,
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

        // Array conversion
        #[pymethod]
        fn to_array(&self, vm: &VirtualMachine) -> PyObjectRef {
            vm.ctx
                .new_tuple(vec![
                    vm.ctx.new_float(self.0.x).into(),
                    vm.ctx.new_float(self.0.y).into(),
                    vm.ctx.new_float(self.0.z).into(),
                ])
                .into()
        }
        #[pymethod]
        fn to_list(&self, vm: &VirtualMachine) -> PyObjectRef {
            vm.ctx
                .new_list(vec![
                    vm.ctx.new_float(self.0.x).into(),
                    vm.ctx.new_float(self.0.y).into(),
                    vm.ctx.new_float(self.0.z).into(),
                ])
                .into()
        }
        #[pymethod]
        fn extend(&self, w: f64) -> PyDVec4 {
            PyDVec4(self.0.extend(w))
        }
        #[pymethod]
        fn truncate(&self) -> PyDVec2 {
            PyDVec2(self.0.truncate())
        }
        #[pymethod]
        fn to_homogeneous(&self) -> PyDVec4 {
            PyDVec4(self.0.to_homogeneous())
        }
        #[pystaticmethod]
        fn from_homogeneous(v: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let w = extract_vec4(&v, vm)?;
            Ok(Self(DVec3::from_homogeneous(w)))
        }

        // Core glam methods
        #[pymethod]
        fn dot(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(self.0.dot(extract(&rhs, vm)?))
        }
        #[pymethod]
        fn cross(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.cross(extract(&rhs, vm)?)))
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
        fn is_normalized(&self) -> bool {
            self.0.is_normalized()
        }
        #[pymethod]
        fn try_normalize(&self) -> Option<Self> {
            self.0.try_normalize().map(Self)
        }
        #[pymethod]
        fn normalize_or(&self, fallback: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.normalize_or(extract(&fallback, vm)?)))
        }

        // Projection / reflection
        #[pymethod]
        fn project_onto(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.project_onto(extract(&rhs, vm)?)))
        }
        #[pymethod]
        fn reject_from(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.reject_from(extract(&rhs, vm)?)))
        }
        #[pymethod]
        fn project_onto_normalized(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.project_onto_normalized(extract(&rhs, vm)?)))
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

        // Interpolation
        #[pymethod]
        fn lerp(&self, rhs: PyObjectRef, s: f64, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.lerp(extract(&rhs, vm)?, s)))
        }
        #[pymethod]
        fn slerp(&self, rhs: PyObjectRef, s: f64, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.slerp(extract(&rhs, vm)?, s)))
        }
        #[pymethod]
        fn move_towards(&self, rhs: PyObjectRef, d: f64, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.move_towards(extract(&rhs, vm)?, d)))
        }
        #[pymethod]
        fn midpoint(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.midpoint(extract(&rhs, vm)?)))
        }

        // Element ops
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

        // Rotation
        #[pymethod]
        fn angle_between(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(self.0.angle_between(extract(&rhs, vm)?))
        }
        #[pymethod]
        fn rotate_x(&self, angle: f64) -> Self {
            Self(self.0.rotate_x(angle))
        }
        #[pymethod]
        fn rotate_y(&self, angle: f64) -> Self {
            Self(self.0.rotate_y(angle))
        }
        #[pymethod]
        fn rotate_z(&self, angle: f64) -> Self {
            Self(self.0.rotate_z(angle))
        }
        #[pymethod]
        fn rotate_axis(
            &self,
            axis: PyObjectRef,
            angle: f64,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(self.0.rotate_axis(extract(&axis, vm)?, angle)))
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
        fn any_orthogonal_vector(&self) -> Self {
            Self(self.0.any_orthogonal_vector())
        }
        #[pymethod]
        fn any_orthonormal_vector(&self) -> Self {
            Self(self.0.any_orthonormal_vector())
        }
        #[pymethod]
        fn any_orthonormal_pair(&self) -> (Self, Self) {
            let (a, b) = self.0.any_orthonormal_pair();
            (Self(a), Self(b))
        }

        // Predicates
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

        // Unary
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
            format!("[{}, {}, {}]", self.0.x, self.0.y, self.0.z)
        }

        #[pymethod]
        fn to_json(&self, vm: &VirtualMachine) -> PyResult<String> {
            crate::rp_serde::to_json(&self.0, vm)
        }
        #[pystaticmethod]
        fn from_json(s: String, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(crate::rp_serde::from_json::<DVec3>(&s, vm)?))
        }
        #[pystaticmethod]
        fn try_from_json(s: String) -> Option<Self> {
            crate::rp_serde::try_from_json::<DVec3>(&s).map(Self)
        }
        #[pymethod]
        fn to_dict(&self, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            crate::rp_serde::to_dict(&self.0, vm)
        }
        #[pystaticmethod]
        fn from_dict(obj: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(crate::rp_serde::from_dict::<DVec3>(&obj, vm)?))
        }
        #[pystaticmethod]
        fn try_from_dict(obj: PyObjectRef, vm: &VirtualMachine) -> Option<Self> {
            crate::rp_serde::try_from_dict::<DVec3>(&obj, vm).map(Self)
        }

        #[pymethod]
        fn __getnewargs_ex__(&self, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            crate::rp_serde::getnewargs_ex(&self.0, vm)
        }

        #[pygetset]
        fn __dataclass_fields__(&self, vm: &VirtualMachine) -> PyObjectRef {
            crate::rp_serde::dataclass_fields(&["x", "y", "z"], vm)
        }
    }

    pub(crate) fn install_constants(typ: &rustpython_vm::builtins::PyTypeRef, vm: &VirtualMachine) {
        let set = |name: &str, v: DVec3| {
            typ.set_attr(vm.ctx.intern_str(name), PyDVec3(v).into_pyobject(vm));
        };
        set("ZERO", DVec3::ZERO);
        set("ONE", DVec3::ONE);
        set("NEG_ONE", DVec3::NEG_ONE);
        set("X", DVec3::X);
        set("Y", DVec3::Y);
        set("Z", DVec3::Z);
        set("NEG_X", DVec3::NEG_X);
        set("NEG_Y", DVec3::NEG_Y);
        set("NEG_Z", DVec3::NEG_Z);
        set("INFINITY", DVec3::INFINITY);
        set("NEG_INFINITY", DVec3::NEG_INFINITY);
        set("NAN", DVec3::NAN);
    }

    impl_rp_vec_ops!(PyDVec3, DVec3, 3);
}

#[cfg(feature = "rustpython-backend")]
pub(crate) use rustpython_impl::install_constants;

#[cfg(feature = "rustpython-backend")]
pub(crate) use rustpython_impl::extract as extract_vec3;
