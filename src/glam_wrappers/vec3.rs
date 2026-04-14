use super::{PyDMat3, PyDVec2, PyDVec4, extract_numpy_vector, impl_serde_methods, impl_vec_constants, impl_vec_unary};
use glam::{DMat3, DVec3};
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::{PyIndexError, PyValueError};
use pyo3::prelude::*;

#[pyclass(skip_from_py_object, name = "Vec3")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PyDVec3(pub(crate) DVec3);

impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyDVec3 {
    type Error = pyo3::PyErr;
    fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(v) = ob.cast::<Self>() {
            return Ok(v.borrow().clone());
        }
        let x: f64 = ob.getattr("x")?.extract()?;
        let y: f64 = ob.getattr("y")?.extract()?;
        let z: f64 = ob.getattr("z")?.extract()?;
        Ok(Self(DVec3::new(x, y, z)))
    }
}

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

#[pymethods]
impl PyDVec3 {
    #[new]
    #[inline]
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self(DVec3::new(x, y, z))
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
    fn from_numpy(array: PyReadonlyArray1<'_, f64>) -> PyResult<Self> {
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
    #[inline]
    fn relative_eq(&self, rhs: Self, max_abs_diff: f64, max_relative: f64) -> bool {
        approx::RelativeEq::relative_eq(&self.0, &rhs.0, max_abs_diff, max_relative)
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

    fn __setitem__(&mut self, idx: isize, val: f64) -> PyResult<()> {
        let i = if idx < 0 { 3 + idx } else { idx };
        match i {
            0 => self.0.x = val,
            1 => self.0.y = val,
            2 => self.0.z = val,
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

    fn __getstate__(&self) -> [f64; 3] {
        self.0.to_array()
    }

    fn __setstate__(&mut self, state: [f64; 3]) {
        self.0 = DVec3::from_array(state);
    }

    fn __array__<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_slice(py, &self.0.to_array())
    }
}

/// Additional helper methods ported from valstad geom.py.
#[pymethods]
impl PyDVec3 {
    /// Create a Vec3 from a rotation matrix (direction) and radius.
    ///
    /// Equivalent to `rotation * Vec3(1, 0, 0) * radius`.
    #[staticmethod]
    fn from_spherical(angle: PyDMat3, radius: f64) -> Self {
        let ref_axis = DVec3::X;
        let rotated = angle.0 * ref_axis;
        Self(rotated * radius)
    }

    /// Compute the rotation (Mat3) that rotates `ref_axis` (default X+) to align with `self`.
    ///
    /// Returns Mat3.identity() when `self` is parallel to `ref_axis`, and a 180°
    /// rotation around a perpendicular axis when anti-parallel.
    #[pyo3(signature = (ref_axis=None))]
    fn to_rotation(&self, ref_axis: Option<PyDVec3>) -> PyResult<PyDMat3> {
        let v = self.0;
        let r = ref_axis.map(|r| r.0).unwrap_or(DVec3::X);

        let v_len = v.length();
        let r_len = r.length();
        if v_len < 1e-12 || r_len < 1e-12 {
            return Err(PyValueError::new_err(
                "Rotation undefined for zero-length vector.",
            ));
        }

        let v_hat = v / v_len;
        let r_hat = r / r_len;

        let cross = r_hat.cross(v_hat);
        let cross_len = cross.length();

        if cross_len < 1e-12 {
            let dot = r_hat.dot(v_hat);
            if dot > 0.0 {
                return Ok(PyDMat3(DMat3::IDENTITY));
            } else {
                // Anti-parallel: rotate 180° around a perpendicular axis
                let perp = if r_hat.x.abs() < 0.9 {
                    DVec3::X
                } else {
                    DVec3::Y
                };
                let axis = r_hat.cross(perp).normalize();
                return Ok(PyDMat3(DMat3::from_axis_angle(
                    axis,
                    std::f64::consts::PI,
                )));
            }
        }

        let axis = cross / cross_len;
        let angle = cross_len.atan2(r_hat.dot(v_hat));
        Ok(PyDMat3(DMat3::from_axis_angle(axis, angle)))
    }

    /// Rotate this vector around a pivot point by the given rotation matrix.
    ///
    /// Equivalent to `rotation * (self - point) + point`.
    fn rotate_around(&self, point: PyDVec3, rotation: PyDMat3) -> Self {
        let rel = self.0 - point.0;
        let rotated = rotation.0 * rel;
        Self(rotated + point.0)
    }
}

impl_serde_methods!(PyDVec3, DVec3);
