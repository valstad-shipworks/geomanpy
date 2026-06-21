//! `Affine3` (double-precision) wrapper.

use glam::DAffine3;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, skip_from_py_object, name = "Affine3")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Affine3")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PyDAffine3(pub DAffine3);

impl From<DAffine3> for PyDAffine3 {
    #[inline]
    fn from(a: DAffine3) -> Self {
        Self(a)
    }
}

impl From<PyDAffine3> for DAffine3 {
    #[inline]
    fn from(a: PyDAffine3) -> Self {
        a.0
    }
}

// =============================================================================
// PyO3 backend
// =============================================================================

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::{
        PyDMat3, PyDMat4, PyDQuat, PyDVec3, array2_from_rows, extract_numpy_matrix,
        impl_serde_methods, transpose_array2,
    };
    use crate::pickle::pickle_decode;
    use crate::{impl_dataclass_fields, impl_getnewargs_ex};
    use numpy::{AllowTypeChange, PyArray2, PyArrayLike2};
    use pyo3::prelude::*;

    impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyDAffine3 {
        type Error = pyo3::PyErr;
        fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(v) = ob.cast_exact::<Self>() {
                return Ok(*v.get());
            }
            let py = ob.py();
            let mat3: PyDMat3 = ob.getattr(pyo3::intern!(py, "matrix3"))?.extract()?;
            let trans: PyDVec3 = ob.getattr(pyo3::intern!(py, "translation"))?.extract()?;
            Ok(Self(DAffine3 {
                matrix3: mat3.0,
                translation: trans.0,
            }))
        }
    }

    #[pymethods]
    impl PyDAffine3 {
        #[new]
        #[pyo3(signature = (translation=None, rotation=None, *, __pickle_state__=None))]
        #[inline]
        fn new(
            translation: Option<PyDVec3>,
            rotation: Option<PyDMat3>,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<DAffine3>(&state)?));
            }
            match (translation, rotation) {
                (Some(t), Some(r)) => Ok(Self(DAffine3::from_mat3_translation(r.0, t.0))),
                _ => Err(pyo3::exceptions::PyValueError::new_err(
                    "Affine3 requires translation and rotation arguments",
                )),
            }
        }

        #[staticmethod]
        #[inline]
        fn from_cols(x_axis: PyDVec3, y_axis: PyDVec3, z_axis: PyDVec3, w_axis: PyDVec3) -> Self {
            Self(DAffine3::from_cols(x_axis.0, y_axis.0, z_axis.0, w_axis.0))
        }
        #[staticmethod]
        #[inline]
        fn from_cols_array(m: [f64; 12]) -> Self {
            Self(DAffine3::from_cols_array(&m))
        }
        #[staticmethod]
        #[inline]
        fn from_cols_array_2d(m: [[f64; 3]; 4]) -> Self {
            Self(DAffine3::from_cols_array_2d(&m))
        }
        #[staticmethod]
        #[inline]
        fn from_numpy(array: PyArrayLike2<'_, f64, AllowTypeChange>) -> PyResult<Self> {
            let rows = extract_numpy_matrix::<3, 4>(array, "Affine3")?;
            Ok(Self(DAffine3::from_cols_array_2d(&transpose_array2(rows))))
        }
        #[staticmethod]
        #[inline]
        fn from_scale(scale: PyDVec3) -> Self {
            Self(DAffine3::from_scale(scale.0))
        }
        #[staticmethod]
        #[inline]
        fn from_quat(rotation: PyDQuat) -> Self {
            Self(DAffine3::from_quat(rotation.0))
        }
        #[staticmethod]
        #[inline]
        fn from_axis_angle(axis: PyDVec3, angle: f64) -> Self {
            Self(DAffine3::from_axis_angle(axis.0, angle))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_x(angle: f64) -> Self {
            Self(DAffine3::from_rotation_x(angle))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_y(angle: f64) -> Self {
            Self(DAffine3::from_rotation_y(angle))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_z(angle: f64) -> Self {
            Self(DAffine3::from_rotation_z(angle))
        }
        #[staticmethod]
        #[inline]
        fn from_translation(translation: PyDVec3) -> Self {
            Self(DAffine3::from_translation(translation.0))
        }
        #[staticmethod]
        #[inline]
        fn from_mat3(mat3: PyDMat3) -> Self {
            Self(DAffine3::from_mat3(mat3.0))
        }
        #[staticmethod]
        fn just(component: &Bound<'_, PyAny>) -> PyResult<Self> {
            if let Ok(t) = component.extract::<PyDVec3>() {
                return Ok(Self(DAffine3::from_translation(t.0)));
            }
            if let Ok(r) = component.extract::<PyDMat3>() {
                return Ok(Self(DAffine3::from_mat3(r.0)));
            }
            Err(pyo3::exceptions::PyTypeError::new_err(
                "expected Vec3 or Mat3",
            ))
        }
        #[staticmethod]
        #[inline]
        fn from_mat3_translation(mat3: PyDMat3, translation: PyDVec3) -> Self {
            Self(DAffine3::from_mat3_translation(mat3.0, translation.0))
        }
        #[staticmethod]
        #[inline]
        fn from_scale_rotation_translation(
            scale: PyDVec3,
            rotation: PyDQuat,
            translation: PyDVec3,
        ) -> Self {
            Self(DAffine3::from_scale_rotation_translation(
                scale.0,
                rotation.0,
                translation.0,
            ))
        }
        #[staticmethod]
        #[inline]
        fn from_rotation_translation(rotation: PyDQuat, translation: PyDVec3) -> Self {
            Self(DAffine3::from_rotation_translation(
                rotation.0,
                translation.0,
            ))
        }
        #[staticmethod]
        #[inline]
        fn from_mat4(m: PyDMat4) -> Self {
            Self(DAffine3::from_mat4(m.0))
        }
        #[staticmethod]
        #[inline]
        fn look_to_lh(eye: PyDVec3, dir: PyDVec3, up: PyDVec3) -> Self {
            Self(DAffine3::look_to_lh(eye.0, dir.0, up.0))
        }
        #[staticmethod]
        #[inline]
        fn look_to_rh(eye: PyDVec3, dir: PyDVec3, up: PyDVec3) -> Self {
            Self(DAffine3::look_to_rh(eye.0, dir.0, up.0))
        }
        #[staticmethod]
        #[inline]
        fn look_at_lh(eye: PyDVec3, center: PyDVec3, up: PyDVec3) -> Self {
            Self(DAffine3::look_at_lh(eye.0, center.0, up.0))
        }
        #[staticmethod]
        #[inline]
        fn look_at_rh(eye: PyDVec3, center: PyDVec3, up: PyDVec3) -> Self {
            Self(DAffine3::look_at_rh(eye.0, center.0, up.0))
        }
    }

    #[pymethods]
    impl PyDAffine3 {
        #[getter]
        #[inline]
        fn matrix3(&self) -> PyDMat3 {
            PyDMat3(self.0.matrix3)
        }
        #[getter]
        #[inline]
        fn translation(&self) -> PyDVec3 {
            PyDVec3(self.0.translation)
        }
    }

    #[pymethods]
    impl PyDAffine3 {
        #[inline]
        fn transform_point3(&self, rhs: PyDVec3) -> PyDVec3 {
            PyDVec3(self.0.transform_point3(rhs.0))
        }
        #[inline]
        fn transform_vector3(&self, rhs: PyDVec3) -> PyDVec3 {
            PyDVec3(self.0.transform_vector3(rhs.0))
        }
        #[inline]
        fn inverse(&self) -> Self {
            Self(self.0.inverse())
        }
        #[inline]
        fn to_scale_rotation_translation(&self) -> (PyDVec3, PyDQuat, PyDVec3) {
            let (s, r, t) = self.0.to_scale_rotation_translation();
            (PyDVec3(s), PyDQuat(r), PyDVec3(t))
        }
        #[inline]
        fn to_cols_array(&self) -> [f64; 12] {
            self.0.to_cols_array()
        }
        #[inline]
        fn to_cols_array_2d(&self) -> [[f64; 3]; 4] {
            self.0.to_cols_array_2d()
        }
        #[inline]
        fn to_numpy<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
            array2_from_rows(py, transpose_array2(self.0.to_cols_array_2d()))
        }
    }

    #[pymethods]
    impl PyDAffine3 {
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
    impl PyDAffine3 {
        #[classattr]
        #[pyo3(name = "IDENTITY")]
        fn identity_const() -> Self {
            Self(DAffine3::IDENTITY)
        }
        #[classattr]
        #[pyo3(name = "ZERO")]
        fn zero_const() -> Self {
            Self(DAffine3::ZERO)
        }
        #[classattr]
        #[pyo3(name = "NAN")]
        fn nan_const() -> Self {
            Self(DAffine3::NAN)
        }
    }

    #[pymethods]
    impl PyDAffine3 {
        fn __repr__(&self) -> String {
            format!(
                "Affine3(matrix3={:?}, translation={:?})",
                self.0.matrix3, self.0.translation
            )
        }
        fn __eq__(&self, other: Self) -> bool {
            self.0 == other.0
        }
        fn __ne__(&self, other: Self) -> bool {
            self.0 != other.0
        }
        fn __mul__(&self, other: Self) -> Self {
            Self(self.0 * other.0)
        }

        fn __array__<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
            array2_from_rows(py, transpose_array2(self.0.to_cols_array_2d()))
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

    impl_serde_methods!(PyDAffine3, DAffine3);
    impl_getnewargs_ex!(PyDAffine3);
    impl_dataclass_fields!(PyDAffine3, ["matrix3", "translation"]);
}

// =============================================================================
// RustPython backend
// =============================================================================

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::glam_wrappers::{
        PyDMat3, PyDMat4, PyDQuat, PyDVec3, quat::extract_quat, vec3::extract_vec3,
    };
    use rustpython_vm::{
        Py, PyObject, PyObjectRef, PyPayload, PyResult, VirtualMachine,
        builtins::PyType,
        function::{FuncArgs, PyComparisonValue},
        protocol::PyNumberMethods,
        pyclass,
        types::{AsNumber, Comparable, Constructor, Hashable, PyComparisonOp, Representable},
    };

    pub(crate) fn extract(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<DAffine3> {
        if let Some(a) = obj.downcast_ref::<PyDAffine3>() {
            return Ok(a.0);
        }
        let mat3 = extract_mat3(&obj.get_attr("matrix3", vm)?, vm)?;
        let translation = extract_vec3(&obj.get_attr("translation", vm)?, vm)?;
        Ok(DAffine3 {
            matrix3: mat3,
            translation,
        })
    }

    impl Constructor for PyDAffine3 {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<DAffine3>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            match args.args.len() {
                2 => {
                    let translation = extract_vec3(&args.args[0], vm)?;
                    let rotation = extract_mat3(&args.args[1], vm)?;
                    Ok(Self(DAffine3::from_mat3_translation(rotation, translation)))
                }
                n => Err(vm.new_type_error(format!(
                    "Affine3() takes 2 positional arguments (translation, rotation), got {n}"
                ))),
            }
        }
    }

    impl Representable for PyDAffine3 {
        #[inline]
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok(format!(
                "Affine3(matrix3={:?}, translation={:?})",
                zelf.0.matrix3, zelf.0.translation
            ))
        }
    }

    #[pyclass(with(Constructor, Representable, AsNumber, Comparable, Hashable))]
    impl PyDAffine3 {
        #[pygetset]
        fn matrix3(&self) -> PyDMat3 {
            PyDMat3(self.0.matrix3)
        }
        #[pygetset]
        fn translation(&self) -> PyDVec3 {
            PyDVec3(self.0.translation)
        }

        #[pystaticmethod]
        fn from_cols(
            x_axis: PyObjectRef,
            y_axis: PyObjectRef,
            z_axis: PyObjectRef,
            w_axis: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DAffine3::from_cols(
                extract_vec3(&x_axis, vm)?,
                extract_vec3(&y_axis, vm)?,
                extract_vec3(&z_axis, vm)?,
                extract_vec3(&w_axis, vm)?,
            )))
        }
        #[pystaticmethod]
        fn from_cols_array(m: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let xs: Vec<f64> = m.try_to_value(vm)?;
            if xs.len() != 12 {
                return Err(
                    vm.new_value_error("Affine3.from_cols_array expects 12 elements".to_owned())
                );
            }
            let mut arr = [0.0; 12];
            arr.copy_from_slice(&xs);
            Ok(Self(DAffine3::from_cols_array(&arr)))
        }
        #[pystaticmethod]
        fn from_cols_array_2d(m: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let rows = crate::glam_wrappers::extract_numpy_matrix_rp::<4, 3>(&m, "Affine3", vm)?;
            Ok(Self(DAffine3::from_cols_array_2d(&rows)))
        }
        #[pystaticmethod]
        fn from_numpy(obj: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let rows = crate::glam_wrappers::extract_numpy_matrix_rp::<3, 4>(&obj, "Affine3", vm)?;
            Ok(Self(DAffine3::from_cols_array_2d(
                &crate::glam_wrappers::transpose_array2_rp(rows),
            )))
        }
        #[pystaticmethod]
        fn from_scale(scale: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DAffine3::from_scale(extract_vec3(&scale, vm)?)))
        }
        #[pystaticmethod]
        fn from_quat(rotation: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DAffine3::from_quat(extract_quat(&rotation, vm)?)))
        }
        #[pystaticmethod]
        fn from_axis_angle(axis: PyObjectRef, angle: f64, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DAffine3::from_axis_angle(
                extract_vec3(&axis, vm)?,
                angle,
            )))
        }
        #[pystaticmethod]
        fn from_rotation_x(angle: f64) -> Self {
            Self(DAffine3::from_rotation_x(angle))
        }
        #[pystaticmethod]
        fn from_rotation_y(angle: f64) -> Self {
            Self(DAffine3::from_rotation_y(angle))
        }
        #[pystaticmethod]
        fn from_rotation_z(angle: f64) -> Self {
            Self(DAffine3::from_rotation_z(angle))
        }
        #[pystaticmethod]
        fn from_translation(translation: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DAffine3::from_translation(extract_vec3(
                &translation,
                vm,
            )?)))
        }
        #[pystaticmethod]
        fn from_mat3(mat3: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DAffine3::from_mat3(extract_mat3(&mat3, vm)?)))
        }
        #[pystaticmethod]
        fn just(component: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(v) = component.downcast_ref::<PyDVec3>() {
                return Ok(Self(DAffine3::from_translation(v.0)));
            }
            if let Some(m) = component.downcast_ref::<PyDMat3>() {
                return Ok(Self(DAffine3::from_mat3(m.0)));
            }
            Err(vm.new_type_error("expected Vec3 or Mat3".to_owned()))
        }
        #[pystaticmethod]
        fn from_mat3_translation(
            mat3: PyObjectRef,
            translation: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DAffine3::from_mat3_translation(
                extract_mat3(&mat3, vm)?,
                extract_vec3(&translation, vm)?,
            )))
        }
        #[pystaticmethod]
        fn from_scale_rotation_translation(
            scale: PyObjectRef,
            rotation: PyObjectRef,
            translation: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DAffine3::from_scale_rotation_translation(
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
            Ok(Self(DAffine3::from_rotation_translation(
                extract_quat(&rotation, vm)?,
                extract_vec3(&translation, vm)?,
            )))
        }
        #[pystaticmethod]
        fn from_mat4(m: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(DAffine3::from_mat4(extract_mat4(&m, vm)?)))
        }
        #[pystaticmethod]
        fn look_to_lh(
            eye: PyObjectRef,
            dir: PyObjectRef,
            up: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            Ok(Self(DAffine3::look_to_lh(
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
            Ok(Self(DAffine3::look_to_rh(
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
            Ok(Self(DAffine3::look_at_lh(
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
            Ok(Self(DAffine3::look_at_rh(
                extract_vec3(&eye, vm)?,
                extract_vec3(&center, vm)?,
                extract_vec3(&up, vm)?,
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
        fn to_scale_rotation_translation(&self) -> (PyDVec3, PyDQuat, PyDVec3) {
            let (s, r, t) = self.0.to_scale_rotation_translation();
            (PyDVec3(s), PyDQuat(r), PyDVec3(t))
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
        fn inverse(&self) -> Self {
            Self(self.0.inverse())
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
        fn to_json(&self, vm: &VirtualMachine) -> PyResult<String> {
            crate::rp_serde::to_json(&self.0, vm)
        }
        #[pystaticmethod]
        fn from_json(s: String, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(crate::rp_serde::from_json::<DAffine3>(&s, vm)?))
        }
        #[pystaticmethod]
        fn try_from_json(s: String) -> Option<Self> {
            crate::rp_serde::try_from_json::<DAffine3>(&s).map(Self)
        }
        #[pymethod]
        fn to_dict(&self, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            crate::rp_serde::to_dict(&self.0, vm)
        }
        #[pystaticmethod]
        fn from_dict(obj: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(crate::rp_serde::from_dict::<DAffine3>(&obj, vm)?))
        }
        #[pystaticmethod]
        fn try_from_dict(obj: PyObjectRef, vm: &VirtualMachine) -> Option<Self> {
            crate::rp_serde::try_from_dict::<DAffine3>(&obj, vm).map(Self)
        }

        #[pymethod]
        fn __getnewargs_ex__(&self, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            crate::rp_serde::getnewargs_ex(&self.0, vm)
        }
        #[pygetset]
        fn __dataclass_fields__(&self, vm: &VirtualMachine) -> PyObjectRef {
            crate::rp_serde::dataclass_fields(&["matrix3", "translation"], vm)
        }
    }

    fn extract_mat3(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<glam::DMat3> {
        obj.downcast_ref::<PyDMat3>()
            .map(|m| m.0)
            .ok_or_else(|| vm.new_type_error("expected Mat3".to_owned()))
    }

    fn extract_mat4(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<glam::DMat4> {
        obj.downcast_ref::<PyDMat4>()
            .map(|m| m.0)
            .ok_or_else(|| vm.new_type_error("expected Mat4".to_owned()))
    }

    fn affine3_mul(a: &PyObject, b: &PyObject, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        if let (Some(x), Some(y)) = (
            a.downcast_ref::<PyDAffine3>(),
            b.downcast_ref::<PyDAffine3>(),
        ) {
            return Ok(PyDAffine3(x.0 * y.0).into_pyobject(vm));
        }
        Ok(vm.ctx.not_implemented())
    }

    impl AsNumber for PyDAffine3 {
        fn as_number() -> &'static PyNumberMethods {
            static N: PyNumberMethods = PyNumberMethods {
                multiply: Some(affine3_mul),
                ..PyNumberMethods::NOT_IMPLEMENTED
            };
            &N
        }
    }

    impl Comparable for PyDAffine3 {
        fn cmp(
            zelf: &Py<Self>,
            other: &PyObject,
            op: PyComparisonOp,
            _vm: &VirtualMachine,
        ) -> PyResult<PyComparisonValue> {
            op.eq_only(|| match other.downcast_ref::<PyDAffine3>() {
                Some(o) => Ok(PyComparisonValue::Implemented(zelf.0 == o.0)),
                None => Ok(PyComparisonValue::NotImplemented),
            })
        }
    }

    impl Hashable for PyDAffine3 {
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
        let set = |name: &str, v: DAffine3| {
            typ.set_attr(vm.ctx.intern_str(name), PyDAffine3(v).into_pyobject(vm));
        };
        set("IDENTITY", DAffine3::IDENTITY);
        set("ZERO", DAffine3::ZERO);
        set("NAN", DAffine3::NAN);
    }
}

#[cfg(feature = "rustpython-backend")]
pub(crate) use rustpython_impl::install_constants;
