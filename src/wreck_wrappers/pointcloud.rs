//! `Pointcloud` wrapper.

use wreck::Pointcloud;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, from_py_object, name = "Pointcloud")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Pointcloud")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone)]
pub struct PyPointcloud(pub Pointcloud);

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::pickle::pickle_decode;
    use pyo3::PyResult;
    use pyo3::prelude::*;

    #[pymethods]
    impl PyPointcloud {
        #[new]
        #[pyo3(signature = (points=None, r_range=(0.0, 0.0), point_radius=0.0, *, __pickle_state__=None))]
        fn new(
            points: Option<Vec<[f32; 3]>>,
            r_range: (f32, f32),
            point_radius: f32,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<Pointcloud>(&state)?));
            }
            match points {
                Some(pts) => Ok(Self(Pointcloud::new(&pts, r_range, point_radius))),
                None => Err(pyo3::exceptions::PyValueError::new_err(
                    "Pointcloud requires points argument",
                )),
            }
        }

        #[staticmethod]
        #[pyo3(signature = (points, point_radius = 0.033))]
        fn from_numpy(
            points: numpy::PyArrayLike2<'_, f64, numpy::AllowTypeChange>,
            point_radius: f32,
        ) -> PyResult<Self> {
            let view = points.as_array();
            if view.shape()[1] != 3 {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "points must be (N, 3)",
                ));
            }
            let n = view.shape()[0];
            let mut pts = Vec::with_capacity(n);
            let mut min_r = f32::MAX;
            let mut max_r = 0.0f32;
            for i in 0..n {
                let x = view[(i, 0)] as f32;
                let y = view[(i, 1)] as f32;
                let z = view[(i, 2)] as f32;
                let r = (x * x + y * y + z * z).sqrt();
                if r < min_r {
                    min_r = r;
                }
                if r > max_r {
                    max_r = r;
                }
                pts.push([x, y, z]);
            }
            if pts.is_empty() {
                min_r = 0.0;
                max_r = 0.0;
            }
            Ok(Self(Pointcloud::new(
                &pts,
                (min_r, max_r + point_radius),
                point_radius,
            )))
        }

        #[staticmethod]
        fn from_list(points: Vec<[f64; 3]>, point_radius: f64) -> PyResult<Self> {
            let mut pts: Vec<[f32; 3]> = Vec::with_capacity(points.len());
            let mut min_r = f32::MAX;
            let mut max_r = 0.0f32;
            for p in &points {
                let x = p[0] as f32;
                let y = p[1] as f32;
                let z = p[2] as f32;
                let r = (x * x + y * y + z * z).sqrt();
                if r < min_r {
                    min_r = r;
                }
                if r > max_r {
                    max_r = r;
                }
                pts.push([x, y, z]);
            }
            if pts.is_empty() {
                min_r = 0.0;
                max_r = 0.0;
            }
            Ok(Self(Pointcloud::new(
                &pts,
                (min_r, max_r + point_radius as f32),
                point_radius as f32,
            )))
        }

        fn __repr__(&self) -> String {
            "Pointcloud(...)".to_string()
        }
    }
}

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::glam_wrappers::quat::extract_quat;
    use crate::glam_wrappers::vec3::extract_vec3;
    use crate::wreck_wrappers::rustpython_glue::{
        extract_affine3, extract_mat3, shape_collides_no_pcl,
    };
    use crate::wreck_wrappers::{PyCuboid, PySphere};
    use rustpython_vm::{
        Py, PyObjectRef, PyResult, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        pyclass,
        types::{Constructor, Representable},
    };
    use wreck::{Scalable, Transformable};

    impl Constructor for PyPointcloud {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<Pointcloud>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            Ok(Self(Pointcloud::new(&[], (0.0, 0.0), 0.0)))
        }
    }
    impl Representable for PyPointcloud {
        fn repr_str(_zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok("Pointcloud(...)".to_owned())
        }
    }
    #[pyclass(with(Constructor, Representable))]
    impl PyPointcloud {
        /// Create a Pointcloud from a Python list of 3-tuples
        /// (the rustpython equivalent of `from_numpy`).
        #[pystaticmethod]
        fn from_list(
            points: PyObjectRef,
            point_radius: f64,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            let pts: Vec<Vec<f64>> = points.try_to_value(vm)?;
            let mut pts3: Vec<[f32; 3]> = Vec::with_capacity(pts.len());
            let mut min_r = f32::MAX;
            let mut max_r = 0.0f32;
            for p in &pts {
                if p.len() != 3 {
                    return Err(vm.new_value_error("each point must be a 3-tuple".to_owned()));
                }
                let x = p[0] as f32;
                let y = p[1] as f32;
                let z = p[2] as f32;
                let r = (x * x + y * y + z * z).sqrt();
                if r < min_r {
                    min_r = r;
                }
                if r > max_r {
                    max_r = r;
                }
                pts3.push([x, y, z]);
            }
            if pts3.is_empty() {
                min_r = 0.0;
                max_r = 0.0;
            }
            Ok(Self(Pointcloud::new(
                &pts3,
                (min_r, max_r + point_radius as f32),
                point_radius as f32,
            )))
        }

        #[pymethod]
        fn scaled(&self, factor: f64) -> Self {
            Self(self.0.scaled_d(factor))
        }
        #[pymethod]
        fn translated(&self, offset: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.translated_d(extract_vec3(&offset, vm)?)))
        }
        #[pymethod]
        fn rotated_mat(&self, mat: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.rotated_mat_d(extract_mat3(&mat, vm)?)))
        }
        #[pymethod]
        fn rotated_quat(&self, quat: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.rotated_quat_d(extract_quat(&quat, vm)?)))
        }
        #[pymethod]
        fn transformed(&self, tf: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(self.0.transformed_d(extract_affine3(&tf, vm)?)))
        }

        #[pymethod]
        fn broadphase(&self) -> PySphere {
            PySphere(self.0.broadphase())
        }
        #[pymethod]
        fn obb(&self) -> PyCuboid {
            PyCuboid(self.0.obb())
        }
        #[pymethod]
        fn aabb(&self) -> PyCuboid {
            PyCuboid(self.0.aabb())
        }

        #[pymethod]
        fn collides(&self, other: PyObjectRef, vm: &VirtualMachine) -> PyResult<bool> {
            shape_collides_no_pcl(&self.0, &other, vm)
        }
        #[pymethod]
        fn abs_diff_eq(
            &self,
            other: PyObjectRef,
            max_abs_diff: f64,
            vm: &VirtualMachine,
        ) -> PyResult<bool> {
            let o = other
                .downcast_ref::<PyPointcloud>()
                .ok_or_else(|| vm.new_type_error("expected Pointcloud".to_owned()))?;
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
            crate::rp_serde::dataclass_fields(&[], vm)
        }
    }
}
