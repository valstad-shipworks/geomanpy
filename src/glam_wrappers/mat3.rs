//! `Mat3` (double-precision) wrapper.

use glam::DMat3;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, skip_from_py_object, name = "Mat3")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Mat3")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PyDMat3(pub DMat3);

impl From<DMat3> for PyDMat3 {
    #[inline]
    fn from(m: DMat3) -> Self {
        Self(m)
    }
}

impl From<PyDMat3> for DMat3 {
    #[inline]
    fn from(m: PyDMat3) -> Self {
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
        PyDMat4, PyDQuat, PyDVec2, PyDVec3, PyEulerRot, array2_from_rows, extract_numpy_matrix,
        impl_serde_methods, transpose_array2,
    };
    use crate::pickle::pickle_decode;
    use crate::{impl_dataclass_fields, impl_getnewargs_ex};
    use glam::DQuat;
    use numpy::{AllowTypeChange, PyArray2, PyArrayLike2};
    use pyo3::exceptions::PyIndexError;
    use pyo3::prelude::*;

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyDMat3 {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(*v.get());
            }
            let py = ob.py();
            let cols: [f64; 9] = ob
                .call_method0(pyo3::intern!(py, "to_cols_array"))?
                .extract()?;
            Ok(Self(DMat3::from_cols_array(&cols)))
        }
    }

    #[pymethods]
    impl PyDMat3 {
        #[new]
        #[pyo3(signature = (x_axis=None, y_axis=None, z_axis=None, *, __pickle_state__=None))]
        #[inline]
        fn new(
            x_axis: Option<PyDVec3>,
            y_axis: Option<PyDVec3>,
            z_axis: Option<PyDVec3>,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<DMat3>(&state)?));
            }
            match (x_axis, y_axis, z_axis) {
                (Some(x), Some(y), Some(z)) => Ok(Self(DMat3::from_cols(x.0, y.0, z.0))),
                _ => Err(pyo3::exceptions::PyValueError::new_err(
                    "Mat3 requires x_axis, y_axis, z_axis arguments",
                )),
            }
        }

        #[staticmethod]
        #[inline]
        fn from_cols(x_axis: PyDVec3, y_axis: PyDVec3, z_axis: PyDVec3) -> Self {
            Self(DMat3::from_cols(x_axis.0, y_axis.0, z_axis.0))
        }
        #[staticmethod]
        #[inline]
        fn from_cols_array(m: [f64; 9]) -> Self {
            Self(DMat3::from_cols_array(&m))
        }
        #[staticmethod]
        #[inline]
        fn from_cols_array_2d(m: [[f64; 3]; 3]) -> Self {
            Self(DMat3::from_cols_array_2d(&m))
        }
        #[staticmethod]
        #[inline]
        fn from_numpy(array: PyArrayLike2<'_, f64, AllowTypeChange>) -> PyResult<Self> {
            let rows = extract_numpy_matrix::<3, 3>(array, "Mat3")?;
            Ok(Self(DMat3::from_cols_array_2d(&transpose_array2(rows))))
        }
        #[staticmethod]
        #[inline]
        fn from_diagonal(diagonal: PyDVec3) -> Self {
            Self(DMat3::from_diagonal(diagonal.0))
        }
        #[staticmethod]
        #[inline]
        fn from_quat(rotation: PyDQuat) -> Self {
            Self(DMat3::from_quat(rotation.0))
        }
        #[staticmethod]
        #[inline]
        fn from_axis_angle(axis: PyDVec3, angle: f64) -> Self {
            Self(DMat3::from_axis_angle(axis.0, angle))
        }
        #[staticmethod]
        #[inline]
        fn from_scaled_axis(v: PyDVec3) -> Self {
            Self(DMat3::from_quat(DQuat::from_scaled_axis(v.0)))
        }
        #[staticmethod]
        #[inline]
        fn from_euler(order: PyEulerRot, a: f64, b: f64, c: f64) -> Self {
            Self(DMat3::from_euler(order.into(), a, b, c))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_x(angle: f64) -> Self {
            Self(DMat3::from_rotation_x(angle))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_y(angle: f64) -> Self {
            Self(DMat3::from_rotation_y(angle))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_z(angle: f64) -> Self {
            Self(DMat3::from_rotation_z(angle))
        }
        #[staticmethod]
        #[inline]
        fn from_angle(angle: f64) -> Self {
            Self(DMat3::from_angle(angle))
        }
        #[staticmethod]
        #[inline]
        fn from_translation(translation: PyDVec2) -> Self {
            Self(DMat3::from_translation(translation.0))
        }
        #[staticmethod]
        #[inline]
        fn from_scale(scale: PyDVec2) -> Self {
            Self(DMat3::from_scale(scale.0))
        }
        #[staticmethod]
        #[inline]
        fn from_scale_angle_translation(scale: PyDVec2, angle: f64, translation: PyDVec2) -> Self {
            Self(DMat3::from_scale_angle_translation(
                scale.0,
                angle,
                translation.0,
            ))
        }
        #[staticmethod]
        #[inline]
        fn from_mat4(m: PyDMat4) -> Self {
            Self(DMat3::from_mat4(m.0))
        }
        #[staticmethod]
        #[inline]
        fn from_mat4_minor(m: PyDMat4, i: usize, j: usize) -> Self {
            Self(DMat3::from_mat4_minor(m.0, i, j))
        }
        #[staticmethod]
        #[inline]
        fn look_to_lh(dir: PyDVec3, up: PyDVec3) -> Self {
            Self(DMat3::look_to_lh(dir.0, up.0))
        }
        #[staticmethod]
        #[inline]
        fn look_to_rh(dir: PyDVec3, up: PyDVec3) -> Self {
            Self(DMat3::look_to_rh(dir.0, up.0))
        }
    }

    #[pymethods]
    impl PyDMat3 {
        #[getter]
        #[inline]
        fn x_axis(&self) -> PyDVec3 {
            PyDVec3(self.0.x_axis)
        }
        #[getter]
        #[inline]
        fn y_axis(&self) -> PyDVec3 {
            PyDVec3(self.0.y_axis)
        }
        #[getter]
        #[inline]
        fn z_axis(&self) -> PyDVec3 {
            PyDVec3(self.0.z_axis)
        }

        #[inline]
        fn col(&self, index: usize) -> PyResult<PyDVec3> {
            if index < 3 {
                Ok(PyDVec3(self.0.col(index)))
            } else {
                Err(PyIndexError::new_err("column index out of range"))
            }
        }
        #[inline]
        fn row(&self, index: usize) -> PyResult<PyDVec3> {
            if index < 3 {
                Ok(PyDVec3(self.0.row(index)))
            } else {
                Err(PyIndexError::new_err("row index out of range"))
            }
        }
    }

    #[pymethods]
    impl PyDMat3 {
        #[inline]
        fn to_cols_array(&self) -> [f64; 9] {
            self.0.to_cols_array()
        }
        #[inline]
        fn to_cols_array_2d(&self) -> [[f64; 3]; 3] {
            self.0.to_cols_array_2d()
        }
        #[inline]
        fn to_numpy<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
            array2_from_rows(py, transpose_array2(self.0.to_cols_array_2d()))
        }
    }

    #[pymethods]
    impl PyDMat3 {
        #[inline]
        fn diagonal(&self) -> PyDVec3 {
            PyDVec3(self.0.diagonal())
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
        fn to_euler(&self, order: PyEulerRot) -> (f64, f64, f64) {
            self.0.to_euler(order.into())
        }
    }

    #[pymethods]
    impl PyDMat3 {
        #[inline]
        fn mul_vec3(&self, rhs: PyDVec3) -> PyDVec3 {
            PyDVec3(self.0.mul_vec3(rhs.0))
        }
        #[inline]
        fn mul_transpose_vec3(&self, rhs: PyDVec3) -> PyDVec3 {
            PyDVec3(self.0.mul_transpose_vec3(rhs.0))
        }
        #[inline]
        fn transform_point2(&self, rhs: PyDVec2) -> PyDVec2 {
            PyDVec2(self.0.transform_point2(rhs.0))
        }
        #[inline]
        fn transform_vector2(&self, rhs: PyDVec2) -> PyDVec2 {
            PyDVec2(self.0.transform_vector2(rhs.0))
        }
    }

    #[pymethods]
    impl PyDMat3 {
        #[inline]
        fn mul_mat3(&self, rhs: Self) -> Self {
            Self(self.0.mul_mat3(&rhs.0))
        }
        #[inline]
        fn add_mat3(&self, rhs: Self) -> Self {
            Self(self.0.add_mat3(&rhs.0))
        }
        #[inline]
        fn sub_mat3(&self, rhs: Self) -> Self {
            Self(self.0.sub_mat3(&rhs.0))
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
        fn mul_diagonal_scale(&self, scale: PyDVec3) -> Self {
            Self(self.0.mul_diagonal_scale(scale.0))
        }
    }

    #[pymethods]
    impl PyDMat3 {
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
    impl PyDMat3 {
        #[classattr]
        #[pyo3(name = "IDENTITY")]
        fn identity_const() -> Self {
            Self(DMat3::IDENTITY)
        }
        #[classattr]
        #[pyo3(name = "ZERO")]
        fn zero_const() -> Self {
            Self(DMat3::ZERO)
        }
        #[classattr]
        #[pyo3(name = "NAN")]
        fn nan_const() -> Self {
            Self(DMat3::NAN)
        }
    }

    #[pymethods]
    impl PyDMat3 {
        fn __repr__(&self) -> String {
            format!(
                "Mat3([{}, {}, {}], [{}, {}, {}], [{}, {}, {}])",
                self.0.x_axis.x,
                self.0.x_axis.y,
                self.0.x_axis.z,
                self.0.y_axis.x,
                self.0.y_axis.y,
                self.0.y_axis.z,
                self.0.z_axis.x,
                self.0.z_axis.y,
                self.0.z_axis.z,
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
            } else if let Ok(v) = other.extract::<PyDVec3>() {
                Ok(PyDVec3(self.0 * v.0).into_pyobject(py)?.into_any().unbind())
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

    impl_serde_methods!(PyDMat3, DMat3);
    impl_getnewargs_ex!(PyDMat3);
    impl_dataclass_fields!(PyDMat3, ["x_axis", "y_axis", "z_axis"]);
}

// =============================================================================
// RustPython backend
// =============================================================================

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::glam_wrappers::{
        PyDVec2, PyDVec3, PyEulerRot, quat::extract_quat, vec2::extract_vec2, vec3::extract_vec3,
    };
    use rustpython_vm::{
        Py, PyObject, PyObjectRef, PyPayload, PyResult, TryFromObject, VirtualMachine,
        builtins::PyType,
        function::{FuncArgs, PyComparisonValue},
        pyclass,
        types::{AsNumber, Comparable, Constructor, Hashable, PyComparisonOp, Representable},
    };

    fn extract_euler(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<glam::EulerRot> {
        obj.downcast_ref::<PyEulerRot>()
            .map(|e| (**e).into())
            .ok_or_else(|| vm.new_type_error("expected EulerRot".to_owned()))
    }

    impl Constructor for PyDMat3 {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<DMat3>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            if args.args.is_empty() {
                return Ok(Self(DMat3::IDENTITY));
            }
            if args.args.len() == 3 {
                let x = extract_vec3(&args.args[0], vm)?;
                let y = extract_vec3(&args.args[1], vm)?;
                let z = extract_vec3(&args.args[2], vm)?;
                return Ok(Self(DMat3::from_cols(x, y, z)));
            }
            Err(vm.new_type_error(format!(
                "Mat3() takes 0 or 3 positional arguments, got {}",
                args.args.len()
            )))
        }
    }

    impl Representable for PyDMat3 {
        #[inline]
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            let m = zelf.0;
            Ok(format!(
                "Mat3([{}, {}, {}], [{}, {}, {}], [{}, {}, {}])",
                m.x_axis.x,
                m.x_axis.y,
                m.x_axis.z,
                m.y_axis.x,
                m.y_axis.y,
                m.y_axis.z,
                m.z_axis.x,
                m.z_axis.y,
                m.z_axis.z,
            ))
        }
    }

    #[pyclass(with(Constructor, Representable, AsNumber, Comparable, Hashable))]
    impl PyDMat3 {
        #[pygetset]
        fn x_axis(&self) -> PyDVec3 {
            PyDVec3(self.0.x_axis)
        }
        #[pygetset]
        fn y_axis(&self) -> PyDVec3 {
            PyDVec3(self.0.y_axis)
        }
        #[pygetset]
        fn z_axis(&self) -> PyDVec3 {
            PyDVec3(self.0.z_axis)
        }

        #[pystaticmethod]
        fn from_cols(
            x_axis: PyObjectRef,
            y_axis: PyObjectRef,
            z_axis: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DMat3::from_cols(
                extract_vec3(&x_axis, vm)?,
                extract_vec3(&y_axis, vm)?,
                extract_vec3(&z_axis, vm)?,
            )))
        }
        #[pystaticmethod]
        fn from_cols_array(m: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let xs: Vec<f64> = m.try_to_value(vm)?;
            if xs.len() != 9 {
                return Err(
                    vm.new_value_error("Mat3.from_cols_array expects 9 elements".to_owned())
                );
            }
            let mut a = [0.0; 9];
            a.copy_from_slice(&xs);
            Ok(Self(DMat3::from_cols_array(&a)))
        }
        #[pystaticmethod]
        fn from_cols_array_2d(m: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let rows: Vec<Vec<f64>> = m.try_to_value(vm)?;
            if rows.len() != 3 || rows.iter().any(|r| r.len() != 3) {
                return Err(
                    vm.new_value_error("Mat3.from_cols_array_2d expects a 3x3 array".to_owned())
                );
            }
            let cols = [
                [rows[0][0], rows[0][1], rows[0][2]],
                [rows[1][0], rows[1][1], rows[1][2]],
                [rows[2][0], rows[2][1], rows[2][2]],
            ];
            Ok(Self(DMat3::from_cols_array_2d(&cols)))
        }
        #[pystaticmethod]
        fn from_numpy(obj: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let rows = crate::glam_wrappers::extract_numpy_matrix_rp::<3, 3>(&obj, "Mat3", vm)?;
            Ok(Self(DMat3::from_cols_array_2d(
                &crate::glam_wrappers::transpose_array2_rp(rows),
            )))
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
            let cols = self.0.to_cols_array_2d();
            let rows = cols
                .iter()
                .map(|col| {
                    let inner = col.iter().map(|v| vm.ctx.new_float(*v).into()).collect();
                    vm.ctx.new_list(inner).into()
                })
                .collect();
            vm.ctx.new_list(rows).into()
        }
        #[pystaticmethod]
        fn from_diagonal(diagonal: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DMat3::from_diagonal(extract_vec3(&diagonal, vm)?)))
        }
        #[pystaticmethod]
        fn from_axis_angle(axis: PyObjectRef, angle: f64, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DMat3::from_axis_angle(
                extract_vec3(&axis, vm)?,
                angle,
            )))
        }
        #[pystaticmethod]
        fn from_scaled_axis(v: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DMat3::from_quat(glam::DQuat::from_scaled_axis(
                extract_vec3(&v, vm)?,
            ))))
        }
        #[pystaticmethod]
        fn from_euler(
            order: PyObjectRef,
            a: f64,
            b: f64,
            c: f64,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DMat3::from_euler(extract_euler(&order, vm)?, a, b, c)))
        }
        #[pystaticmethod]
        fn from_rotation_x(angle: f64) -> Self {
            Self(DMat3::from_rotation_x(angle))
        }
        #[pystaticmethod]
        fn from_rotation_y(angle: f64) -> Self {
            Self(DMat3::from_rotation_y(angle))
        }
        #[pystaticmethod]
        fn from_rotation_z(angle: f64) -> Self {
            Self(DMat3::from_rotation_z(angle))
        }
        #[pystaticmethod]
        fn from_angle(angle: f64) -> Self {
            Self(DMat3::from_angle(angle))
        }
        #[pystaticmethod]
        fn from_quat(rotation: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DMat3::from_quat(extract_quat(&rotation, vm)?)))
        }
        #[pystaticmethod]
        fn from_translation(translation: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DMat3::from_translation(extract_vec2(
                &translation,
                vm,
            )?)))
        }
        #[pystaticmethod]
        fn from_scale(scale: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DMat3::from_scale(extract_vec2(&scale, vm)?)))
        }
        #[pystaticmethod]
        fn from_scale_angle_translation(
            scale: PyObjectRef,
            angle: f64,
            translation: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DMat3::from_scale_angle_translation(
                extract_vec2(&scale, vm)?,
                angle,
                extract_vec2(&translation, vm)?,
            )))
        }
        #[pystaticmethod]
        fn from_mat4(m: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let m4 = m
                .downcast_ref::<crate::glam_wrappers::PyDMat4>()
                .ok_or_else(|| vm.new_type_error("expected Mat4".to_owned()))?;
            Ok(Self(DMat3::from_mat4(m4.0)))
        }
        #[pystaticmethod]
        fn from_mat4_minor(
            m: PyObjectRef,
            i: usize,
            j: usize,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            let m4 = m
                .downcast_ref::<crate::glam_wrappers::PyDMat4>()
                .ok_or_else(|| vm.new_type_error("expected Mat4".to_owned()))?;
            Ok(Self(DMat3::from_mat4_minor(m4.0, i, j)))
        }
        #[pystaticmethod]
        fn look_to_lh(dir: PyObjectRef, up: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DMat3::look_to_lh(
                extract_vec3(&dir, vm)?,
                extract_vec3(&up, vm)?,
            )))
        }
        #[pystaticmethod]
        fn look_to_rh(dir: PyObjectRef, up: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DMat3::look_to_rh(
                extract_vec3(&dir, vm)?,
                extract_vec3(&up, vm)?,
            )))
        }

        #[pymethod]
        fn col(&self, index: usize, vm: &VirtualMachine) -> PyResult<PyDVec3> {
            if index < 3 {
                Ok(PyDVec3(self.0.col(index)))
            } else {
                Err(vm.new_index_error("column index out of range".to_owned()))
            }
        }
        #[pymethod]
        fn row(&self, index: usize, vm: &VirtualMachine) -> PyResult<PyDVec3> {
            if index < 3 {
                Ok(PyDVec3(self.0.row(index)))
            } else {
                Err(vm.new_index_error("row index out of range".to_owned()))
            }
        }
        #[pymethod]
        fn diagonal(&self) -> PyDVec3 {
            PyDVec3(self.0.diagonal())
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
        fn to_euler(&self, order: PyObjectRef, vm: &VirtualMachine) -> PyResult<(f64, f64, f64)> {
            Ok(self.0.to_euler(extract_euler(&order, vm)?))
        }
        #[pymethod]
        fn mul_vec3(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyDVec3> {
            Ok(PyDVec3(self.0.mul_vec3(extract_vec3(&rhs, vm)?)))
        }
        #[pymethod]
        fn mul_transpose_vec3(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyDVec3> {
            Ok(PyDVec3(self.0.mul_transpose_vec3(extract_vec3(&rhs, vm)?)))
        }
        #[pymethod]
        fn transform_point2(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyDVec2> {
            Ok(PyDVec2(self.0.transform_point2(extract_vec2(&rhs, vm)?)))
        }
        #[pymethod]
        fn transform_vector2(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyDVec2> {
            Ok(PyDVec2(self.0.transform_vector2(extract_vec2(&rhs, vm)?)))
        }
        #[pymethod]
        fn mul_mat3(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(m) = rhs.downcast_ref::<PyDMat3>() {
                Ok(Self(self.0.mul_mat3(&m.0)))
            } else {
                Err(vm.new_type_error("expected Mat3".to_owned()))
            }
        }
        #[pymethod]
        fn add_mat3(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(m) = rhs.downcast_ref::<PyDMat3>() {
                Ok(Self(self.0.add_mat3(&m.0)))
            } else {
                Err(vm.new_type_error("expected Mat3".to_owned()))
            }
        }
        #[pymethod]
        fn sub_mat3(&self, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(m) = rhs.downcast_ref::<PyDMat3>() {
                Ok(Self(self.0.sub_mat3(&m.0)))
            } else {
                Err(vm.new_type_error("expected Mat3".to_owned()))
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
            Ok(Self(self.0.mul_diagonal_scale(extract_vec3(&scale, vm)?)))
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
            let m = rhs
                .downcast_ref::<PyDMat3>()
                .ok_or_else(|| vm.new_type_error("expected Mat3".to_owned()))?;
            Ok(self.0.abs_diff_eq(m.0, max_abs_diff))
        }

        #[pymethod]
        fn to_json(&self, vm: &VirtualMachine) -> PyResult<String> {
            crate::rp_serde::to_json(&self.0, vm)
        }
        #[pystaticmethod]
        fn from_json(json: String, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(crate::rp_serde::from_json::<DMat3>(&json, vm)?))
        }
        #[pystaticmethod]
        fn try_from_json(json: String) -> Option<Self> {
            crate::rp_serde::try_from_json::<DMat3>(&json).map(Self)
        }
        #[pymethod]
        fn to_dict(&self, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            crate::rp_serde::to_dict(&self.0, vm)
        }
        #[pystaticmethod]
        fn from_dict(obj: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(crate::rp_serde::from_dict::<DMat3>(&obj, vm)?))
        }
        #[pystaticmethod]
        fn try_from_dict(obj: PyObjectRef, vm: &VirtualMachine) -> Option<Self> {
            crate::rp_serde::try_from_dict::<DMat3>(&obj, vm).map(Self)
        }

        #[pymethod]
        fn __getnewargs_ex__(&self, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            crate::rp_serde::getnewargs_ex(&self.0, vm)
        }
        #[pygetset]
        fn __dataclass_fields__(&self, vm: &VirtualMachine) -> PyObjectRef {
            crate::rp_serde::dataclass_fields(&["x_axis", "y_axis", "z_axis"], vm)
        }
    }

    fn mat3_mul(a: &PyObject, b: &PyObject, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        if let (Some(x), Some(y)) = (a.downcast_ref::<PyDMat3>(), b.downcast_ref::<PyDMat3>()) {
            return Ok(PyDMat3(x.0 * y.0).into_pyobject(vm));
        }
        if let (Some(x), Some(v)) = (a.downcast_ref::<PyDMat3>(), b.downcast_ref::<PyDVec3>()) {
            return Ok(PyDVec3(x.0 * v.0).into_pyobject(vm));
        }
        if let Some(x) = a.downcast_ref::<PyDMat3>()
            && let Ok(s) = f64::try_from_object(vm, b.to_owned())
        {
            return Ok(PyDMat3(x.0 * s).into_pyobject(vm));
        }
        if let Some(y) = b.downcast_ref::<PyDMat3>()
            && let Ok(s) = f64::try_from_object(vm, a.to_owned())
        {
            return Ok(PyDMat3(s * y.0).into_pyobject(vm));
        }
        Ok(vm.ctx.not_implemented())
    }

    fn mat3_add(a: &PyObject, b: &PyObject, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        if let (Some(x), Some(y)) = (a.downcast_ref::<PyDMat3>(), b.downcast_ref::<PyDMat3>()) {
            return Ok(PyDMat3(x.0 + y.0).into_pyobject(vm));
        }
        Ok(vm.ctx.not_implemented())
    }

    fn mat3_sub(a: &PyObject, b: &PyObject, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        if let (Some(x), Some(y)) = (a.downcast_ref::<PyDMat3>(), b.downcast_ref::<PyDMat3>()) {
            return Ok(PyDMat3(x.0 - y.0).into_pyobject(vm));
        }
        Ok(vm.ctx.not_implemented())
    }

    fn mat3_div(a: &PyObject, b: &PyObject, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        if let Some(x) = a.downcast_ref::<PyDMat3>()
            && let Ok(s) = f64::try_from_object(vm, b.to_owned())
        {
            return Ok(PyDMat3(x.0 / s).into_pyobject(vm));
        }
        Ok(vm.ctx.not_implemented())
    }

    impl AsNumber for PyDMat3 {
        fn as_number() -> &'static rustpython_vm::protocol::PyNumberMethods {
            static N: rustpython_vm::protocol::PyNumberMethods =
                rustpython_vm::protocol::PyNumberMethods {
                    add: Some(mat3_add),
                    subtract: Some(mat3_sub),
                    multiply: Some(mat3_mul),
                    true_divide: Some(mat3_div),
                    negative: Some(|num, vm| {
                        let z = PyDMat3::number_downcast(num);
                        Ok(PyDMat3(-z.0).into_pyobject(vm))
                    }),
                    ..rustpython_vm::protocol::PyNumberMethods::NOT_IMPLEMENTED
                };
            &N
        }
    }

    impl Comparable for PyDMat3 {
        fn cmp(
            zelf: &Py<Self>,
            other: &PyObject,
            op: PyComparisonOp,
            _vm: &VirtualMachine,
        ) -> PyResult<PyComparisonValue> {
            op.eq_only(|| match other.downcast_ref::<PyDMat3>() {
                Some(o) => Ok(PyComparisonValue::Implemented(zelf.0 == o.0)),
                None => Ok(PyComparisonValue::NotImplemented),
            })
        }
    }

    impl Hashable for PyDMat3 {
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

    pub(crate) fn install_constants(typ: &rustpython_vm::builtins::PyTypeRef, vm: &VirtualMachine) {
        let set = |name: &str, v: DMat3| {
            typ.set_attr(vm.ctx.intern_str(name), PyDMat3(v).into_pyobject(vm));
        };
        set("IDENTITY", DMat3::IDENTITY);
        set("ZERO", DMat3::ZERO);
        set("NAN", DMat3::NAN);
    }
}

#[cfg(feature = "rustpython-backend")]
pub(crate) use rustpython_impl::install_constants;
