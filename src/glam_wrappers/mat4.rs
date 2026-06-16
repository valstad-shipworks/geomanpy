//! `Mat4` (double-precision) wrapper.

use glam::DMat4;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, skip_from_py_object, name = "Mat4")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Mat4")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PyDMat4(pub(crate) DMat4);

impl From<DMat4> for PyDMat4 {
    #[inline]
    fn from(m: DMat4) -> Self {
        Self(m)
    }
}

impl From<PyDMat4> for DMat4 {
    #[inline]
    fn from(m: PyDMat4) -> Self {
        m.0
    }
}

// =============================================================================
// PyO3 backend
// =============================================================================

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::{
        PyDMat3, PyDQuat, PyDVec3, PyDVec4, PyEulerRot, array2_from_rows, extract_numpy_matrix,
        transpose_array2,
    };
    use crate::pickle::pickle_decode;
    use crate::{impl_dataclass_fields, impl_getnewargs_ex};
    use numpy::{AllowTypeChange, PyArray2, PyArrayLike2};
    use pyo3::exceptions::PyIndexError;
    use pyo3::prelude::*;

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyDMat4 {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(*v.get());
            }
            let py = ob.py();
            let cols: (
                (f64, f64, f64, f64),
                (f64, f64, f64, f64),
                (f64, f64, f64, f64),
                (f64, f64, f64, f64),
            ) = ob
                .call_method0(pyo3::intern!(py, "to_cols_array_2d"))?
                .extract()?;
            Ok(Self(glam::DMat4::from_cols(
                glam::DVec4::new(cols.0.0, cols.0.1, cols.0.2, cols.0.3),
                glam::DVec4::new(cols.1.0, cols.1.1, cols.1.2, cols.1.3),
                glam::DVec4::new(cols.2.0, cols.2.1, cols.2.2, cols.2.3),
                glam::DVec4::new(cols.3.0, cols.3.1, cols.3.2, cols.3.3),
            )))
        }
    }

    #[pymethods]
    impl PyDMat4 {
        #[new]
        #[pyo3(signature = (x_axis=None, y_axis=None, z_axis=None, w_axis=None, *, __pickle_state__=None))]
        #[inline]
        fn new(
            x_axis: Option<PyDVec4>,
            y_axis: Option<PyDVec4>,
            z_axis: Option<PyDVec4>,
            w_axis: Option<PyDVec4>,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<DMat4>(&state)?));
            }
            match (x_axis, y_axis, z_axis, w_axis) {
                (Some(x), Some(y), Some(z), Some(w)) => {
                    Ok(Self(DMat4::from_cols(x.0, y.0, z.0, w.0)))
                }
                _ => Err(pyo3::exceptions::PyValueError::new_err(
                    "Mat4 requires x_axis, y_axis, z_axis, w_axis arguments",
                )),
            }
        }

        #[staticmethod]
        #[inline]
        fn from_cols(x_axis: PyDVec4, y_axis: PyDVec4, z_axis: PyDVec4, w_axis: PyDVec4) -> Self {
            Self(DMat4::from_cols(x_axis.0, y_axis.0, z_axis.0, w_axis.0))
        }
        #[staticmethod]
        #[inline]
        fn from_cols_array(m: [f64; 16]) -> Self {
            Self(DMat4::from_cols_array(&m))
        }
        #[staticmethod]
        #[inline]
        fn from_cols_array_2d(m: [[f64; 4]; 4]) -> Self {
            Self(DMat4::from_cols_array_2d(&m))
        }
        #[staticmethod]
        #[inline]
        fn from_numpy(array: PyArrayLike2<'_, f64, AllowTypeChange>) -> PyResult<Self> {
            let rows = extract_numpy_matrix::<4, 4>(array, "Mat4")?;
            Ok(Self(DMat4::from_cols_array_2d(&transpose_array2(rows))))
        }
        #[staticmethod]
        #[inline]
        fn from_diagonal(diagonal: PyDVec4) -> Self {
            Self(DMat4::from_diagonal(diagonal.0))
        }
        #[staticmethod]
        #[inline]
        fn from_scale_rotation_translation(
            scale: PyDVec3,
            rotation: PyDQuat,
            translation: PyDVec3,
        ) -> Self {
            Self(DMat4::from_scale_rotation_translation(
                scale.0,
                rotation.0,
                translation.0,
            ))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_translation(rotation: PyDQuat, translation: PyDVec3) -> Self {
            Self(DMat4::from_rotation_translation(rotation.0, translation.0))
        }
        #[staticmethod]
        #[inline]
        fn from_quat(rotation: PyDQuat) -> Self {
            Self(DMat4::from_quat(rotation.0))
        }
        #[staticmethod]
        #[inline]
        fn from_mat3(m: PyDMat3) -> Self {
            Self(DMat4::from_mat3(m.0))
        }
        #[staticmethod]
        #[inline]
        fn from_mat3_translation(mat3: PyDMat3, translation: PyDVec3) -> Self {
            Self(DMat4::from_mat3_translation(mat3.0, translation.0))
        }
        #[staticmethod]
        #[inline]
        fn from_translation(translation: PyDVec3) -> Self {
            Self(DMat4::from_translation(translation.0))
        }
        #[staticmethod]
        #[inline]
        fn from_axis_angle(axis: PyDVec3, angle: f64) -> Self {
            Self(DMat4::from_axis_angle(axis.0, angle))
        }
        #[staticmethod]
        #[inline]
        fn from_euler(order: PyEulerRot, a: f64, b: f64, c: f64) -> Self {
            Self(DMat4::from_euler(order.into(), a, b, c))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_x(angle: f64) -> Self {
            Self(DMat4::from_rotation_x(angle))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_y(angle: f64) -> Self {
            Self(DMat4::from_rotation_y(angle))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_z(angle: f64) -> Self {
            Self(DMat4::from_rotation_z(angle))
        }
        #[staticmethod]
        #[inline]
        fn from_scale(scale: PyDVec3) -> Self {
            Self(DMat4::from_scale(scale.0))
        }
    }

    #[pymethods]
    impl PyDMat4 {
        #[staticmethod]
        fn perspective_rh_gl(
            fov_y_radians: f64,
            aspect_ratio: f64,
            z_near: f64,
            z_far: f64,
        ) -> Self {
            Self(DMat4::perspective_rh_gl(
                fov_y_radians,
                aspect_ratio,
                z_near,
                z_far,
            ))
        }
        #[staticmethod]
        fn perspective_lh(fov_y_radians: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Self {
            Self(DMat4::perspective_lh(
                fov_y_radians,
                aspect_ratio,
                z_near,
                z_far,
            ))
        }
        #[staticmethod]
        fn perspective_rh(fov_y_radians: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Self {
            Self(DMat4::perspective_rh(
                fov_y_radians,
                aspect_ratio,
                z_near,
                z_far,
            ))
        }
        #[staticmethod]
        fn perspective_infinite_lh(fov_y_radians: f64, aspect_ratio: f64, z_near: f64) -> Self {
            Self(DMat4::perspective_infinite_lh(
                fov_y_radians,
                aspect_ratio,
                z_near,
            ))
        }
        #[staticmethod]
        fn perspective_infinite_rh(fov_y_radians: f64, aspect_ratio: f64, z_near: f64) -> Self {
            Self(DMat4::perspective_infinite_rh(
                fov_y_radians,
                aspect_ratio,
                z_near,
            ))
        }
        #[staticmethod]
        fn perspective_infinite_reverse_lh(
            fov_y_radians: f64,
            aspect_ratio: f64,
            z_near: f64,
        ) -> Self {
            Self(DMat4::perspective_infinite_reverse_lh(
                fov_y_radians,
                aspect_ratio,
                z_near,
            ))
        }
        #[staticmethod]
        fn perspective_infinite_reverse_rh(
            fov_y_radians: f64,
            aspect_ratio: f64,
            z_near: f64,
        ) -> Self {
            Self(DMat4::perspective_infinite_reverse_rh(
                fov_y_radians,
                aspect_ratio,
                z_near,
            ))
        }
        #[staticmethod]
        fn orthographic_rh_gl(
            left: f64,
            right: f64,
            bottom: f64,
            top: f64,
            near: f64,
            far: f64,
        ) -> Self {
            Self(DMat4::orthographic_rh_gl(
                left, right, bottom, top, near, far,
            ))
        }
        #[staticmethod]
        fn orthographic_lh(
            left: f64,
            right: f64,
            bottom: f64,
            top: f64,
            near: f64,
            far: f64,
        ) -> Self {
            Self(DMat4::orthographic_lh(left, right, bottom, top, near, far))
        }
        #[staticmethod]
        fn orthographic_rh(
            left: f64,
            right: f64,
            bottom: f64,
            top: f64,
            near: f64,
            far: f64,
        ) -> Self {
            Self(DMat4::orthographic_rh(left, right, bottom, top, near, far))
        }
        #[staticmethod]
        fn frustum_rh_gl(
            left: f64,
            right: f64,
            bottom: f64,
            top: f64,
            z_near: f64,
            z_far: f64,
        ) -> Self {
            Self(DMat4::frustum_rh_gl(
                left, right, bottom, top, z_near, z_far,
            ))
        }
        #[staticmethod]
        fn frustum_lh(
            left: f64,
            right: f64,
            bottom: f64,
            top: f64,
            z_near: f64,
            z_far: f64,
        ) -> Self {
            Self(DMat4::frustum_lh(left, right, bottom, top, z_near, z_far))
        }
        #[staticmethod]
        fn frustum_rh(
            left: f64,
            right: f64,
            bottom: f64,
            top: f64,
            z_near: f64,
            z_far: f64,
        ) -> Self {
            Self(DMat4::frustum_rh(left, right, bottom, top, z_near, z_far))
        }
    }

    #[pymethods]
    impl PyDMat4 {
        #[staticmethod]
        fn look_to_lh(eye: PyDVec3, dir: PyDVec3, up: PyDVec3) -> Self {
            Self(DMat4::look_to_lh(eye.0, dir.0, up.0))
        }
        #[staticmethod]
        fn look_to_rh(eye: PyDVec3, dir: PyDVec3, up: PyDVec3) -> Self {
            Self(DMat4::look_to_rh(eye.0, dir.0, up.0))
        }
        #[staticmethod]
        fn look_at_lh(eye: PyDVec3, center: PyDVec3, up: PyDVec3) -> Self {
            Self(DMat4::look_at_lh(eye.0, center.0, up.0))
        }
        #[staticmethod]
        fn look_at_rh(eye: PyDVec3, center: PyDVec3, up: PyDVec3) -> Self {
            Self(DMat4::look_at_rh(eye.0, center.0, up.0))
        }
    }

    #[pymethods]
    impl PyDMat4 {
        #[getter]
        #[inline]
        fn x_axis(&self) -> PyDVec4 {
            PyDVec4(self.0.x_axis)
        }
        #[getter]
        #[inline]
        fn y_axis(&self) -> PyDVec4 {
            PyDVec4(self.0.y_axis)
        }
        #[getter]
        #[inline]
        fn z_axis(&self) -> PyDVec4 {
            PyDVec4(self.0.z_axis)
        }
        #[getter]
        #[inline]
        fn w_axis(&self) -> PyDVec4 {
            PyDVec4(self.0.w_axis)
        }

        #[inline]
        fn col(&self, index: usize) -> PyResult<PyDVec4> {
            if index < 4 {
                Ok(PyDVec4(self.0.col(index)))
            } else {
                Err(PyIndexError::new_err("column index out of range"))
            }
        }
        #[inline]
        fn row(&self, index: usize) -> PyResult<PyDVec4> {
            if index < 4 {
                Ok(PyDVec4(self.0.row(index)))
            } else {
                Err(PyIndexError::new_err("row index out of range"))
            }
        }
    }

    #[pymethods]
    impl PyDMat4 {
        #[inline]
        fn to_cols_array(&self) -> [f64; 16] {
            self.0.to_cols_array()
        }
        #[inline]
        fn to_cols_array_2d(&self) -> [[f64; 4]; 4] {
            self.0.to_cols_array_2d()
        }
        #[inline]
        fn to_numpy<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
            array2_from_rows(py, transpose_array2(self.0.to_cols_array_2d()))
        }
    }

    #[pymethods]
    impl PyDMat4 {
        #[inline]
        fn diagonal(&self) -> PyDVec4 {
            PyDVec4(self.0.diagonal())
        }
        #[inline]
        fn determinant(&self) -> f64 {
            self.0.determinant()
        }
        #[inline]
        fn transpose(&self) -> Self {
            Self(self.0.transpose())
        }
        #[inline]
        fn inverse(&self) -> Self {
            Self(self.0.inverse())
        }
        #[inline]
        fn try_inverse(&self) -> Option<Self> {
            self.0.try_inverse().map(Self)
        }
        #[inline]
        fn inverse_or_zero(&self) -> Self {
            Self(self.0.inverse_or_zero())
        }
        #[inline]
        fn to_scale_rotation_translation(&self) -> (PyDVec3, PyDQuat, PyDVec3) {
            let (s, r, t) = self.0.to_scale_rotation_translation();
            (PyDVec3(s), PyDQuat(r), PyDVec3(t))
        }
        #[inline]
        fn to_euler(&self, order: PyEulerRot) -> (f64, f64, f64) {
            self.0.to_euler(order.into())
        }
    }

    #[pymethods]
    impl PyDMat4 {
        #[inline]
        fn mul_vec4(&self, rhs: PyDVec4) -> PyDVec4 {
            PyDVec4(self.0.mul_vec4(rhs.0))
        }
        #[inline]
        fn mul_transpose_vec4(&self, rhs: PyDVec4) -> PyDVec4 {
            PyDVec4(self.0.mul_transpose_vec4(rhs.0))
        }
        #[inline]
        fn project_point3(&self, rhs: PyDVec3) -> PyDVec3 {
            PyDVec3(self.0.project_point3(rhs.0))
        }
        #[inline]
        fn transform_point3(&self, rhs: PyDVec3) -> PyDVec3 {
            PyDVec3(self.0.transform_point3(rhs.0))
        }
        #[inline]
        fn transform_vector3(&self, rhs: PyDVec3) -> PyDVec3 {
            PyDVec3(self.0.transform_vector3(rhs.0))
        }
    }

    #[pymethods]
    impl PyDMat4 {
        #[inline]
        fn mul_mat4(&self, rhs: Self) -> Self {
            Self(self.0.mul_mat4(&rhs.0))
        }
        #[inline]
        fn add_mat4(&self, rhs: Self) -> Self {
            Self(self.0.add_mat4(&rhs.0))
        }
        #[inline]
        fn sub_mat4(&self, rhs: Self) -> Self {
            Self(self.0.sub_mat4(&rhs.0))
        }
        #[inline]
        fn mul_scalar(&self, rhs: f64) -> Self {
            Self(self.0.mul_scalar(rhs))
        }
        #[inline]
        fn div_scalar(&self, rhs: f64) -> Self {
            Self(self.0.div_scalar(rhs))
        }
        #[inline]
        fn mul_diagonal_scale(&self, scale: PyDVec4) -> Self {
            Self(self.0.mul_diagonal_scale(scale.0))
        }
    }

    #[pymethods]
    impl PyDMat4 {
        #[inline]
        fn abs(&self) -> Self {
            Self(self.0.abs())
        }
        #[inline]
        fn recip(&self) -> Self {
            Self(self.0.recip())
        }
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

    #[pymethods]
    impl PyDMat4 {
        #[classattr]
        #[pyo3(name = "IDENTITY")]
        fn identity_const() -> Self {
            Self(DMat4::IDENTITY)
        }
        #[classattr]
        #[pyo3(name = "ZERO")]
        fn zero_const() -> Self {
            Self(DMat4::ZERO)
        }
        #[classattr]
        #[pyo3(name = "NAN")]
        fn nan_const() -> Self {
            Self(DMat4::NAN)
        }
    }

    #[pymethods]
    impl PyDMat4 {
        fn __repr__(&self) -> String {
            let c = self.0.to_cols_array_2d();
            format!(
                "Mat4([{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}])",
                c[0][0],
                c[0][1],
                c[0][2],
                c[0][3],
                c[1][0],
                c[1][1],
                c[1][2],
                c[1][3],
                c[2][0],
                c[2][1],
                c[2][2],
                c[2][3],
                c[3][0],
                c[3][1],
                c[3][2],
                c[3][3],
            )
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
            if let Ok(m) = other.extract::<Self>() {
                Ok(Self(self.0 * m.0).into_pyobject(py)?.into_any().unbind())
            } else if let Ok(v) = other.extract::<PyDVec4>() {
                Ok(PyDVec4(self.0 * v.0).into_pyobject(py)?.into_any().unbind())
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
                Ok(Self(s * self.0))
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

        fn __array__<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
            self.to_numpy(py)
        }

        fn __hash__(&self) -> u64 {
            use std::hash::{Hash, Hasher};
            let mut h = std::collections::hash_map::DefaultHasher::new();
            for c in self.0.to_cols_array() {
                c.to_bits().hash(&mut h);
            }
            h.finish()
        }
    }

    impl_getnewargs_ex!(PyDMat4);
    impl_dataclass_fields!(PyDMat4, ["x_axis", "y_axis", "z_axis", "w_axis"]);
}

// =============================================================================
// RustPython backend
// =============================================================================

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::glam_wrappers::{
        PyDMat3, PyDQuat, PyDVec3, PyDVec4, PyEulerRot, quat::extract_quat, vec3::extract_vec3,
        vec4::extract_vec4,
    };

    fn extract_mat3(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<glam::DMat3> {
        obj.downcast_ref::<PyDMat3>()
            .map(|m| m.0)
            .ok_or_else(|| vm.new_type_error("expected Mat3".to_owned()))
    }

    fn extract_euler(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<glam::EulerRot> {
        match obj.downcast_ref::<PyEulerRot>() {
            Some(e) => Ok((**e).into()),
            None => Err(vm.new_type_error("expected an EulerRot".to_owned())),
        }
    }
    use rustpython_vm::{
        Py, PyObject, PyObjectRef, PyPayload, PyResult, TryFromObject, VirtualMachine,
        builtins::PyType,
        function::{FuncArgs, PyComparisonValue},
        protocol::PyNumberMethods,
        pyclass,
        types::{AsNumber, Comparable, Constructor, Hashable, PyComparisonOp, Representable},
    };

    impl Constructor for PyDMat4 {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<DMat4>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            if args.args.is_empty() {
                return Ok(Self(DMat4::IDENTITY));
            }
            if args.args.len() == 4 {
                let x = extract_vec4(&args.args[0], vm)?;
                let y = extract_vec4(&args.args[1], vm)?;
                let z = extract_vec4(&args.args[2], vm)?;
                let w = extract_vec4(&args.args[3], vm)?;
                return Ok(Self(DMat4::from_cols(x, y, z, w)));
            }
            Err(vm.new_type_error(format!(
                "Mat4() takes 0 or 4 positional arguments, got {}",
                args.args.len()
            )))
        }
    }

    impl Representable for PyDMat4 {
        #[inline]
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            let c = zelf.0.to_cols_array_2d();
            Ok(format!(
                "Mat4([{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}])",
                c[0][0],
                c[0][1],
                c[0][2],
                c[0][3],
                c[1][0],
                c[1][1],
                c[1][2],
                c[1][3],
                c[2][0],
                c[2][1],
                c[2][2],
                c[2][3],
                c[3][0],
                c[3][1],
                c[3][2],
                c[3][3],
            ))
        }
    }

    #[pyclass(with(Constructor, Representable, AsNumber, Comparable, Hashable))]
    impl PyDMat4 {
        #[pygetset]
        fn x_axis(&self) -> PyDVec4 {
            PyDVec4(self.0.x_axis)
        }
        #[pygetset]
        fn y_axis(&self) -> PyDVec4 {
            PyDVec4(self.0.y_axis)
        }
        #[pygetset]
        fn z_axis(&self) -> PyDVec4 {
            PyDVec4(self.0.z_axis)
        }
        #[pygetset]
        fn w_axis(&self) -> PyDVec4 {
            PyDVec4(self.0.w_axis)
        }

        #[pystaticmethod]
        fn from_cols(
            x_axis: PyObjectRef,
            y_axis: PyObjectRef,
            z_axis: PyObjectRef,
            w_axis: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DMat4::from_cols(
                extract_vec4(&x_axis, vm)?,
                extract_vec4(&y_axis, vm)?,
                extract_vec4(&z_axis, vm)?,
                extract_vec4(&w_axis, vm)?,
            )))
        }
        #[pystaticmethod]
        fn from_cols_array(m: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let xs: Vec<f64> = m.try_to_value(vm)?;
            if xs.len() != 16 {
                return Err(
                    vm.new_value_error("Mat4.from_cols_array expects 16 elements".to_owned())
                );
            }
            let mut arr = [0.0; 16];
            arr.copy_from_slice(&xs);
            Ok(Self(DMat4::from_cols_array(&arr)))
        }
        #[pystaticmethod]
        fn from_cols_array_2d(m: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let rows = crate::glam_wrappers::extract_numpy_matrix_rp::<4, 4>(&m, "Mat4", vm)?;
            Ok(Self(DMat4::from_cols_array_2d(&rows)))
        }
        #[pystaticmethod]
        fn from_numpy(obj: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let rows = crate::glam_wrappers::extract_numpy_matrix_rp::<4, 4>(&obj, "Mat4", vm)?;
            Ok(Self(DMat4::from_cols_array_2d(
                &crate::glam_wrappers::transpose_array2_rp(rows),
            )))
        }
        #[pystaticmethod]
        fn from_diagonal(diagonal: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DMat4::from_diagonal(extract_vec4(&diagonal, vm)?)))
        }
        #[pystaticmethod]
        fn from_scale_rotation_translation(
            scale: PyObjectRef,
            rotation: PyObjectRef,
            translation: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DMat4::from_scale_rotation_translation(
                extract_vec3(&scale, vm)?,
                extract_quat(&rotation, vm)?,
                extract_vec3(&translation, vm)?,
            )))
        }
        #[pystaticmethod]
        fn from_rotation_translation(
            rotation: PyObjectRef,
            translation: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DMat4::from_rotation_translation(
                extract_quat(&rotation, vm)?,
                extract_vec3(&translation, vm)?,
            )))
        }
        #[pystaticmethod]
        fn from_quat(rotation: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DMat4::from_quat(extract_quat(&rotation, vm)?)))
        }
        #[pystaticmethod]
        fn from_mat3(m: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DMat4::from_mat3(extract_mat3(&m, vm)?)))
        }
        #[pystaticmethod]
        fn from_mat3_translation(
            mat3: PyObjectRef,
            translation: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DMat4::from_mat3_translation(
                extract_mat3(&mat3, vm)?,
                extract_vec3(&translation, vm)?,
            )))
        }
        #[pystaticmethod]
        fn from_translation(translation: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DMat4::from_translation(extract_vec3(
                &translation,
                vm,
            )?)))
        }
        #[pystaticmethod]
        fn from_axis_angle(axis: PyObjectRef, angle: f64, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DMat4::from_axis_angle(
                extract_vec3(&axis, vm)?,
                angle,
            )))
        }
        #[pystaticmethod]
        fn from_euler(
            order: PyObjectRef,
            a: f64,
            b: f64,
            c: f64,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DMat4::from_euler(extract_euler(&order, vm)?, a, b, c)))
        }
        #[pystaticmethod]
        fn from_rotation_x(angle: f64) -> Self {
            Self(DMat4::from_rotation_x(angle))
        }
        #[pystaticmethod]
        fn from_rotation_y(angle: f64) -> Self {
            Self(DMat4::from_rotation_y(angle))
        }
        #[pystaticmethod]
        fn from_rotation_z(angle: f64) -> Self {
            Self(DMat4::from_rotation_z(angle))
        }
        #[pystaticmethod]
        fn from_scale(scale: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DMat4::from_scale(extract_vec3(&scale, vm)?)))
        }

        #[pystaticmethod]
        fn perspective_rh_gl(
            fov_y_radians: f64,
            aspect_ratio: f64,
            z_near: f64,
            z_far: f64,
        ) -> Self {
            Self(DMat4::perspective_rh_gl(
                fov_y_radians,
                aspect_ratio,
                z_near,
                z_far,
            ))
        }
        #[pystaticmethod]
        fn perspective_lh(fov_y_radians: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Self {
            Self(DMat4::perspective_lh(
                fov_y_radians,
                aspect_ratio,
                z_near,
                z_far,
            ))
        }
        #[pystaticmethod]
        fn perspective_rh(fov_y_radians: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Self {
            Self(DMat4::perspective_rh(
                fov_y_radians,
                aspect_ratio,
                z_near,
                z_far,
            ))
        }
        #[pystaticmethod]
        fn perspective_infinite_lh(fov_y_radians: f64, aspect_ratio: f64, z_near: f64) -> Self {
            Self(DMat4::perspective_infinite_lh(
                fov_y_radians,
                aspect_ratio,
                z_near,
            ))
        }
        #[pystaticmethod]
        fn perspective_infinite_rh(fov_y_radians: f64, aspect_ratio: f64, z_near: f64) -> Self {
            Self(DMat4::perspective_infinite_rh(
                fov_y_radians,
                aspect_ratio,
                z_near,
            ))
        }
        #[pystaticmethod]
        fn perspective_infinite_reverse_lh(
            fov_y_radians: f64,
            aspect_ratio: f64,
            z_near: f64,
        ) -> Self {
            Self(DMat4::perspective_infinite_reverse_lh(
                fov_y_radians,
                aspect_ratio,
                z_near,
            ))
        }
        #[pystaticmethod]
        fn perspective_infinite_reverse_rh(
            fov_y_radians: f64,
            aspect_ratio: f64,
            z_near: f64,
        ) -> Self {
            Self(DMat4::perspective_infinite_reverse_rh(
                fov_y_radians,
                aspect_ratio,
                z_near,
            ))
        }
        #[pystaticmethod]
        fn orthographic_rh_gl(
            left: f64,
            right: f64,
            bottom: f64,
            top: f64,
            near: f64,
            far: f64,
        ) -> Self {
            Self(DMat4::orthographic_rh_gl(
                left, right, bottom, top, near, far,
            ))
        }
        #[pystaticmethod]
        fn orthographic_lh(
            left: f64,
            right: f64,
            bottom: f64,
            top: f64,
            near: f64,
            far: f64,
        ) -> Self {
            Self(DMat4::orthographic_lh(left, right, bottom, top, near, far))
        }
        #[pystaticmethod]
        fn orthographic_rh(
            left: f64,
            right: f64,
            bottom: f64,
            top: f64,
            near: f64,
            far: f64,
        ) -> Self {
            Self(DMat4::orthographic_rh(left, right, bottom, top, near, far))
        }
        #[pystaticmethod]
        fn frustum_rh_gl(
            left: f64,
            right: f64,
            bottom: f64,
            top: f64,
            z_near: f64,
            z_far: f64,
        ) -> Self {
            Self(DMat4::frustum_rh_gl(
                left, right, bottom, top, z_near, z_far,
            ))
        }
        #[pystaticmethod]
        fn frustum_lh(
            left: f64,
            right: f64,
            bottom: f64,
            top: f64,
            z_near: f64,
            z_far: f64,
        ) -> Self {
            Self(DMat4::frustum_lh(left, right, bottom, top, z_near, z_far))
        }
        #[pystaticmethod]
        fn frustum_rh(
            left: f64,
            right: f64,
            bottom: f64,
            top: f64,
            z_near: f64,
            z_far: f64,
        ) -> Self {
            Self(DMat4::frustum_rh(left, right, bottom, top, z_near, z_far))
        }
        #[pystaticmethod]
        fn look_to_lh(
            eye: PyObjectRef,
            dir: PyObjectRef,
            up: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DMat4::look_to_lh(
                extract_vec3(&eye, vm)?,
                extract_vec3(&dir, vm)?,
                extract_vec3(&up, vm)?,
            )))
        }
        #[pystaticmethod]
        fn look_to_rh(
            eye: PyObjectRef,
            dir: PyObjectRef,
            up: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DMat4::look_to_rh(
                extract_vec3(&eye, vm)?,
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
            Ok(Self(DMat4::look_at_lh(
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
            Ok(Self(DMat4::look_at_rh(
                extract_vec3(&eye, vm)?,
                extract_vec3(&center, vm)?,
                extract_vec3(&up, vm)?,
            )))
        }

        #[pymethod]
        fn to_cols_array(&self, vm: &VirtualMachine) -> PyObjectRef {
            let items = self
                .0
                .to_cols_array()
                .iter()
                .map(|v| vm.ctx.new_float(*v).into())
                .collect();
            vm.ctx.new_list(items).into()
        }
        #[pymethod]
        fn to_cols_array_2d(&self, vm: &VirtualMachine) -> PyObjectRef {
            let rows = self
                .0
                .to_cols_array_2d()
                .iter()
                .map(|col| {
                    let inner = col.iter().map(|v| vm.ctx.new_float(*v).into()).collect();
                    vm.ctx.new_list(inner).into()
                })
                .collect();
            vm.ctx.new_list(rows).into()
        }
        #[pymethod]
        fn to_numpy(&self, vm: &VirtualMachine) -> PyObjectRef {
            crate::glam_wrappers::pyndarray_from_rows(
                crate::glam_wrappers::transpose_array2_rp(self.0.to_cols_array_2d()),
                vm,
            )
        }
        #[pymethod]
        fn __array__(&self, vm: &VirtualMachine) -> PyObjectRef {
            crate::glam_wrappers::pyndarray_from_rows(
                crate::glam_wrappers::transpose_array2_rp(self.0.to_cols_array_2d()),
                vm,
            )
        }

        #[pymethod]
        fn col(&self, index: usize, vm: &VirtualMachine) -> PyResult<PyDVec4> {
            if index < 4 {
                Ok(PyDVec4(self.0.col(index)))
            } else {
                Err(vm.new_index_error("column index out of range".to_owned()))
            }
        }
        #[pymethod]
        fn row(&self, index: usize, vm: &VirtualMachine) -> PyResult<PyDVec4> {
            if index < 4 {
                Ok(PyDVec4(self.0.row(index)))
            } else {
                Err(vm.new_index_error("row index out of range".to_owned()))
            }
        }
        #[pymethod]
        fn diagonal(&self) -> PyDVec4 {
            PyDVec4(self.0.diagonal())
        }
        #[pymethod]
        fn determinant(&self) -> f64 {
            self.0.determinant()
        }
        #[pymethod]
        fn transpose(&self) -> Self {
            Self(self.0.transpose())
        }
        #[pymethod]
        fn inverse(&self) -> Self {
            Self(self.0.inverse())
        }
        #[pymethod]
        fn try_inverse(&self) -> Option<Self> {
            self.0.try_inverse().map(Self)
        }
        #[pymethod]
        fn inverse_or_zero(&self) -> Self {
            Self(self.0.inverse_or_zero())
        }
        #[pymethod]
        fn to_scale_rotation_translation(&self) -> (PyDVec3, PyDQuat, PyDVec3) {
            let (s, r, t) = self.0.to_scale_rotation_translation();
            (PyDVec3(s), PyDQuat(r), PyDVec3(t))
        }
        #[pymethod]
        fn to_euler(&self, order: PyObjectRef, vm: &VirtualMachine) -> PyResult<(f64, f64, f64)> {
            Ok(self.0.to_euler(extract_euler(&order, vm)?))
        }
        #[pymethod]
        fn mul_vec4(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyDVec4> {
            Ok(PyDVec4(self.0.mul_vec4(extract_vec4(&rhs, vm)?)))
        }
        #[pymethod]
        fn mul_transpose_vec4(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyDVec4> {
            Ok(PyDVec4(self.0.mul_transpose_vec4(extract_vec4(&rhs, vm)?)))
        }
        #[pymethod]
        fn project_point3(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyDVec3> {
            Ok(PyDVec3(self.0.project_point3(extract_vec3(&rhs, vm)?)))
        }
        #[pymethod]
        fn transform_point3(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyDVec3> {
            Ok(PyDVec3(self.0.transform_point3(extract_vec3(&rhs, vm)?)))
        }
        #[pymethod]
        fn transform_vector3(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyDVec3> {
            Ok(PyDVec3(self.0.transform_vector3(extract_vec3(&rhs, vm)?)))
        }
        #[pymethod]
        fn mul_mat4(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(m) = rhs.downcast_ref::<PyDMat4>() {
                Ok(Self(self.0.mul_mat4(&m.0)))
            } else {
                Err(vm.new_type_error("expected Mat4".to_owned()))
            }
        }
        #[pymethod]
        fn add_mat4(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(m) = rhs.downcast_ref::<PyDMat4>() {
                Ok(Self(self.0.add_mat4(&m.0)))
            } else {
                Err(vm.new_type_error("expected Mat4".to_owned()))
            }
        }
        #[pymethod]
        fn sub_mat4(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(m) = rhs.downcast_ref::<PyDMat4>() {
                Ok(Self(self.0.sub_mat4(&m.0)))
            } else {
                Err(vm.new_type_error("expected Mat4".to_owned()))
            }
        }
        #[pymethod]
        fn mul_scalar(&self, rhs: f64) -> Self {
            Self(self.0.mul_scalar(rhs))
        }
        #[pymethod]
        fn div_scalar(&self, rhs: f64) -> Self {
            Self(self.0.div_scalar(rhs))
        }
        #[pymethod]
        fn mul_diagonal_scale(&self, scale: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.mul_diagonal_scale(extract_vec4(&scale, vm)?)))
        }
        #[pymethod]
        fn abs(&self) -> Self {
            Self(self.0.abs())
        }
        #[pymethod]
        fn recip(&self) -> Self {
            Self(self.0.recip())
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
            match rhs.downcast_ref::<PyDMat4>() {
                Some(m) => Ok(self.0.abs_diff_eq(m.0, max_abs_diff)),
                None => Err(vm.new_type_error("expected Mat4".to_owned())),
            }
        }
        #[pymethod]
        fn __getnewargs_ex__(&self, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            crate::rp_serde::getnewargs_ex(&self.0, vm)
        }
        #[pygetset]
        fn __dataclass_fields__(&self, vm: &VirtualMachine) -> PyObjectRef {
            crate::rp_serde::dataclass_fields(&["x_axis", "y_axis", "z_axis", "w_axis"], vm)
        }
    }

    impl Comparable for PyDMat4 {
        fn cmp(
            zelf: &Py<Self>,
            other: &PyObject,
            op: PyComparisonOp,
            _vm: &VirtualMachine,
        ) -> PyResult<PyComparisonValue> {
            op.eq_only(|| match other.downcast_ref::<PyDMat4>() {
                Some(o) => Ok(PyComparisonValue::Implemented(zelf.0 == o.0)),
                None => Ok(PyComparisonValue::NotImplemented),
            })
        }
    }

    impl Hashable for PyDMat4 {
        fn hash(
            zelf: &Py<Self>,
            _vm: &VirtualMachine,
        ) -> PyResult<rustpython_vm::common::hash::PyHash> {
            use std::hash::{Hash, Hasher};
            let mut h = std::collections::hash_map::DefaultHasher::new();
            for c in zelf.0.to_cols_array() {
                c.to_bits().hash(&mut h);
            }
            Ok(h.finish() as rustpython_vm::common::hash::PyHash)
        }
    }

    fn mat4_mul(a: &PyObject, b: &PyObject, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        if let (Some(x), Some(y)) = (a.downcast_ref::<PyDMat4>(), b.downcast_ref::<PyDMat4>()) {
            return Ok(PyDMat4(x.0 * y.0).into_pyobject(vm));
        }
        if let (Some(x), Some(v)) = (a.downcast_ref::<PyDMat4>(), b.downcast_ref::<PyDVec4>()) {
            return Ok(PyDVec4(x.0 * v.0).into_pyobject(vm));
        }
        if let Some(x) = a.downcast_ref::<PyDMat4>()
            && let Ok(s) = f64::try_from_object(vm, b.to_owned())
        {
            return Ok(PyDMat4(x.0 * s).into_pyobject(vm));
        }
        if let Some(x) = b.downcast_ref::<PyDMat4>()
            && let Ok(s) = f64::try_from_object(vm, a.to_owned())
        {
            return Ok(PyDMat4(x.0 * s).into_pyobject(vm));
        }
        Ok(vm.ctx.not_implemented())
    }

    fn mat4_add(a: &PyObject, b: &PyObject, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        if let (Some(x), Some(y)) = (a.downcast_ref::<PyDMat4>(), b.downcast_ref::<PyDMat4>()) {
            return Ok(PyDMat4(x.0 + y.0).into_pyobject(vm));
        }
        Ok(vm.ctx.not_implemented())
    }

    fn mat4_sub(a: &PyObject, b: &PyObject, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        if let (Some(x), Some(y)) = (a.downcast_ref::<PyDMat4>(), b.downcast_ref::<PyDMat4>()) {
            return Ok(PyDMat4(x.0 - y.0).into_pyobject(vm));
        }
        Ok(vm.ctx.not_implemented())
    }

    fn mat4_div(a: &PyObject, b: &PyObject, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        if let Some(x) = a.downcast_ref::<PyDMat4>()
            && let Ok(s) = f64::try_from_object(vm, b.to_owned())
        {
            return Ok(PyDMat4(x.0 / s).into_pyobject(vm));
        }
        Ok(vm.ctx.not_implemented())
    }

    impl AsNumber for PyDMat4 {
        fn as_number() -> &'static PyNumberMethods {
            static N: PyNumberMethods = PyNumberMethods {
                add: Some(mat4_add),
                subtract: Some(mat4_sub),
                multiply: Some(mat4_mul),
                true_divide: Some(mat4_div),
                negative: Some(|num, vm| {
                    let z = <PyDMat4 as AsNumber>::number_downcast(num);
                    Ok(PyDMat4(-z.0).into_pyobject(vm))
                }),
                ..PyNumberMethods::NOT_IMPLEMENTED
            };
            &N
        }
    }

    pub(crate) fn install_constants(typ: &rustpython_vm::builtins::PyTypeRef, vm: &VirtualMachine) {
        let set = |name: &str, v: DMat4| {
            typ.set_attr(vm.ctx.intern_str(name), PyDMat4(v).into_pyobject(vm));
        };
        set("IDENTITY", DMat4::IDENTITY);
        set("ZERO", DMat4::ZERO);
        set("NAN", DMat4::NAN);
    }
}

#[cfg(feature = "rustpython-backend")]
pub(crate) use rustpython_impl::install_constants;
