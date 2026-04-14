use glam::DVec4;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;

use super::{PyDVec3, extract_numpy_vector, impl_vec_constants, impl_vec_unary};

#[pyclass(skip_from_py_object, name = "Vec4")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PyDVec4(pub(crate) DVec4);

impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyDVec4 {
    type Error = pyo3::PyErr;
    fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(v) = ob.cast::<Self>() { return Ok(v.borrow().clone()); }
        let x: f64 = ob.getattr("x")?.extract()?;
        let y: f64 = ob.getattr("y")?.extract()?;
        let z: f64 = ob.getattr("z")?.extract()?;
        let w: f64 = ob.getattr("w")?.extract()?;
        Ok(Self(DVec4::new(x, y, z, w)))
    }
}

impl From<DVec4> for PyDVec4 {
    #[inline]
    fn from(v: DVec4) -> Self {
        Self(v)
    }
}

impl From<PyDVec4> for DVec4 {
    #[inline]
    fn from(v: PyDVec4) -> Self {
        v.0
    }
}

#[pymethods]
impl PyDVec4 {
    #[new]
    #[inline]
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self(DVec4::new(x, y, z, w))
    }

    #[staticmethod]
    #[inline]
    fn splat(v: f64) -> Self {
        Self(DVec4::splat(v))
    }

    #[staticmethod]
    #[inline]
    fn from_array(a: [f64; 4]) -> Self {
        Self(DVec4::from_array(a))
    }

    #[staticmethod]
    #[inline]
    fn from_numpy(array: PyReadonlyArray1<'_, f64>) -> PyResult<Self> {
        Ok(Self(DVec4::from_array(extract_numpy_vector::<4>(
            array, "Vec4",
        )?)))
    }

    #[inline]
    fn to_array(&self) -> [f64; 4] {
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
    fn truncate(&self) -> PyDVec3 {
        PyDVec3(self.0.truncate())
    }
}

#[pymethods]
impl PyDVec4 {
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
    #[setter]
    #[inline]
    fn set_x(&mut self, v: f64) {
        self.0.x = v;
    }
    #[setter]
    #[inline]
    fn set_y(&mut self, v: f64) {
        self.0.y = v;
    }
    #[setter]
    #[inline]
    fn set_z(&mut self, v: f64) {
        self.0.z = v;
    }
    #[setter]
    #[inline]
    fn set_w(&mut self, v: f64) {
        self.0.w = v;
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
    #[inline]
    fn with_w(&self, w: f64) -> Self {
        Self(self.0.with_w(w))
    }
}

#[pymethods]
impl PyDVec4 {
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
impl PyDVec4 {
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
impl PyDVec4 {
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
impl PyDVec4 {
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
impl PyDVec4 {
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
impl PyDVec4 {
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
impl PyDVec4 {
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
    #[inline]
    fn relative_eq(&self, rhs: Self, max_abs_diff: f64, max_relative: f64) -> bool {
        approx::RelativeEq::relative_eq(&self.0, &rhs.0, max_abs_diff, max_relative)
    }
}

impl_vec_unary!(
    PyDVec4,
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
    PyDVec4,
    DVec4,
    [
        (zero, ZERO, "ZERO"),
        (one, ONE, "ONE"),
        (neg_one, NEG_ONE, "NEG_ONE"),
        (unit_x, X, "X"),
        (unit_y, Y, "Y"),
        (unit_z, Z, "Z"),
        (unit_w, W, "W"),
        (neg_x, NEG_X, "NEG_X"),
        (neg_y, NEG_Y, "NEG_Y"),
        (neg_z, NEG_Z, "NEG_Z"),
        (neg_w, NEG_W, "NEG_W"),
        (infinity, INFINITY, "INFINITY"),
        (neg_infinity, NEG_INFINITY, "NEG_INFINITY"),
        (nan, NAN, "NAN"),
    ]
);

#[pymethods]
impl PyDVec4 {
    fn __repr__(&self) -> String {
        format!(
            "Vec4({}, {}, {}, {})",
            self.0.x, self.0.y, self.0.z, self.0.w
        )
    }

    fn __str__(&self) -> String {
        format!("[{}, {}, {}, {}]", self.0.x, self.0.y, self.0.z, self.0.w)
    }

    fn __len__(&self) -> usize {
        4
    }

    fn __getitem__(&self, idx: isize) -> PyResult<f64> {
        let i = if idx < 0 { 4 + idx } else { idx };
        match i {
            0 => Ok(self.0.x),
            1 => Ok(self.0.y),
            2 => Ok(self.0.z),
            3 => Ok(self.0.w),
            _ => Err(PyIndexError::new_err("index out of range")),
        }
    }

    fn __setitem__(&mut self, idx: isize, val: f64) -> PyResult<()> {
        let i = if idx < 0 { 4 + idx } else { idx };
        match i {
            0 => self.0.x = val,
            1 => self.0.y = val,
            2 => self.0.z = val,
            3 => self.0.w = val,
            _ => return Err(PyIndexError::new_err("index out of range")),
        }
        Ok(())
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
            Ok(Self(DVec4::splat(s) - self.0))
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
            Ok(Self(DVec4::splat(s) / self.0))
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

    fn __iadd__(&mut self, other: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(v) = other.extract::<Self>() {
            self.0 += v.0;
        } else if let Ok(s) = other.extract::<f64>() {
            self.0 += s;
        } else {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type for +=",
            ));
        }
        Ok(())
    }
    fn __isub__(&mut self, other: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(v) = other.extract::<Self>() {
            self.0 -= v.0;
        } else if let Ok(s) = other.extract::<f64>() {
            self.0 -= s;
        } else {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type for -=",
            ));
        }
        Ok(())
    }
    fn __imul__(&mut self, other: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(v) = other.extract::<Self>() {
            self.0 *= v.0;
        } else if let Ok(s) = other.extract::<f64>() {
            self.0 *= s;
        } else {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type for *=",
            ));
        }
        Ok(())
    }
    fn __itruediv__(&mut self, other: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(v) = other.extract::<Self>() {
            self.0 /= v.0;
        } else if let Ok(s) = other.extract::<f64>() {
            self.0 /= s;
        } else {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type for /=",
            ));
        }
        Ok(())
    }

    fn __getstate__(&self) -> [f64; 4] {
        self.0.to_array()
    }
    fn __setstate__(&mut self, state: [f64; 4]) {
        self.0 = DVec4::from_array(state);
    }

    fn __array__<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_slice(py, &self.0.to_array())
    }
}
