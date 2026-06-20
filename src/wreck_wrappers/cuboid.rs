//! `Cuboid` wrapper.

use wreck::Cuboid;

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(frozen, skip_from_py_object, name = "Cuboid")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Cuboid")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone, Copy)]
pub struct PyCuboid(pub Cuboid);

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::{PyDMat3, PyDVec3};
    use crate::pickle::pickle_decode;
    use crate::wreck_wrappers::pyo3_glue::{dv3, v3d};
    use crate::wreck_wrappers::{PyConvexPolytope, PyShape};
    use glam::Vec3;
    use pyo3::PyResult;
    use pyo3::prelude::*;
    use wreck::Stretchable;
    use wreck::stretched::CuboidStretch;

    #[pymethods]
    impl PyCuboid {
        #[new]
        #[pyo3(signature = (center=None, axes=None, half_extents=None, *, __pickle_state__=None))]
        fn new(
            center: Option<PyDVec3>,
            axes: Option<[[f64; 3]; 3]>,
            half_extents: Option<[f64; 3]>,
            __pickle_state__: Option<Vec<u8>>,
        ) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<Cuboid>(&state)?));
            }
            match (center, axes, half_extents) {
                (Some(center), Some(axes), Some(he)) => Ok(Self(Cuboid::new(
                    dv3(center),
                    [
                        Vec3::new(axes[0][0] as f32, axes[0][1] as f32, axes[0][2] as f32),
                        Vec3::new(axes[1][0] as f32, axes[1][1] as f32, axes[1][2] as f32),
                        Vec3::new(axes[2][0] as f32, axes[2][1] as f32, axes[2][2] as f32),
                    ],
                    [he[0] as f32, he[1] as f32, he[2] as f32],
                ))),
                _ => Err(pyo3::exceptions::PyValueError::new_err(
                    "Cuboid requires center, axes, half_extents arguments",
                )),
            }
        }
        #[staticmethod]
        fn from_aabb(min: PyDVec3, max: PyDVec3) -> Self {
            Self(Cuboid::from_aabb(dv3(min), dv3(max)))
        }
        #[getter]
        fn center(&self) -> PyDVec3 {
            v3d(self.0.center)
        }
        #[getter]
        fn axes(&self) -> [[f64; 3]; 3] {
            [
                [
                    self.0.axes[0].x as f64,
                    self.0.axes[0].y as f64,
                    self.0.axes[0].z as f64,
                ],
                [
                    self.0.axes[1].x as f64,
                    self.0.axes[1].y as f64,
                    self.0.axes[1].z as f64,
                ],
                [
                    self.0.axes[2].x as f64,
                    self.0.axes[2].y as f64,
                    self.0.axes[2].z as f64,
                ],
            ]
        }
        #[getter]
        fn orientation(&self) -> PyDMat3 {
            let a = &self.0.axes;
            PyDMat3(glam::DMat3::from_cols(
                glam::DVec3::new(a[0].x as f64, a[0].y as f64, a[0].z as f64),
                glam::DVec3::new(a[1].x as f64, a[1].y as f64, a[1].z as f64),
                glam::DVec3::new(a[2].x as f64, a[2].y as f64, a[2].z as f64),
            ))
        }
        #[getter]
        fn half_extents(&self) -> [f64; 3] {
            [
                self.0.half_extents[0] as f64,
                self.0.half_extents[1] as f64,
                self.0.half_extents[2] as f64,
            ]
        }
        #[getter]
        fn full_extents(&self) -> [f64; 3] {
            [
                self.0.half_extents[0] as f64 * 2.0,
                self.0.half_extents[1] as f64 * 2.0,
                self.0.half_extents[2] as f64 * 2.0,
            ]
        }
        #[getter]
        fn axis_aligned(&self) -> bool {
            self.0.axis_aligned
        }
        fn contains_point(&self, point: PyDVec3) -> bool {
            self.0.contains_point(dv3(point))
        }
        fn point_dist_sq(&self, point: PyDVec3) -> f64 {
            self.0.point_dist_sq(dv3(point)) as f64
        }
        fn bounding_sphere_radius(&self) -> f64 {
            self.0.bounding_sphere_radius() as f64
        }
        fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
            match self.0.stretch(dv3(translation)) {
                CuboidStretch::Aligned(c) => vec![PyShape::Cuboid(PyCuboid(c))],
                CuboidStretch::Unaligned(p) => vec![PyShape::ConvexPolytope(PyConvexPolytope(p))],
            }
        }
        fn __repr__(&self) -> String {
            self.0.to_string()
        }
        fn corners(&self) -> Vec<PyDVec3> {
            let c = self.0.center;
            let he = self.0.half_extents;
            let ax = &self.0.axes;
            let mut out = Vec::with_capacity(8);
            for sx in [-1.0f32, 1.0] {
                for sy in [-1.0f32, 1.0] {
                    for sz in [-1.0f32, 1.0] {
                        let local =
                            ax[0] * (he[0] * sx) + ax[1] * (he[1] * sy) + ax[2] * (he[2] * sz);
                        out.push(v3d(c + local));
                    }
                }
            }
            out
        }
        #[staticmethod]
        fn from_center_size_orientation(
            center: PyDVec3,
            size: (f64, f64, f64),
            orientation: PyDMat3,
        ) -> Self {
            let half = [
                (size.0.abs() / 2.0) as f32,
                (size.1.abs() / 2.0) as f32,
                (size.2.abs() / 2.0) as f32,
            ];
            let m = orientation.0;
            let axes = [
                Vec3::new(m.x_axis.x as f32, m.x_axis.y as f32, m.x_axis.z as f32),
                Vec3::new(m.y_axis.x as f32, m.y_axis.y as f32, m.y_axis.z as f32),
                Vec3::new(m.z_axis.x as f32, m.z_axis.y as f32, m.z_axis.z as f32),
            ];
            Self(Cuboid {
                center: dv3(center),
                axes,
                half_extents: half,
                axis_aligned: false,
            })
        }
    }
}

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::glam_wrappers::quat::extract_quat;
    use crate::glam_wrappers::vec3::extract_vec3;
    use crate::glam_wrappers::{PyDMat3, PyDVec3};
    use crate::wreck_wrappers::rustpython_glue::{
        dv3, extract_affine3, extract_mat3, shape_collides, v3d,
    };
    use crate::wreck_wrappers::{PyConvexPolytope, PySphere};
    use glam::Vec3;
    use rustpython_vm::{
        Py, PyObjectRef, PyPayload, PyResult, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        pyclass,
        types::{Constructor, Representable},
    };
    use wreck::stretched::CuboidStretch;
    use wreck::{Scalable, Stretchable, Transformable};

    impl Constructor for PyCuboid {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<Cuboid>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            Ok(Self(Cuboid::new(
                Vec3::ZERO,
                [Vec3::X, Vec3::Y, Vec3::Z],
                [0.0, 0.0, 0.0],
            )))
        }
    }
    impl Representable for PyCuboid {
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok(zelf.0.to_string())
        }
    }
    #[pyclass(with(Constructor, Representable))]
    impl PyCuboid {
        #[pygetset]
        fn center(&self) -> PyDVec3 {
            v3d(self.0.center)
        }
        #[pygetset]
        fn half_extents(&self) -> (f64, f64, f64) {
            let he = self.0.half_extents;
            (he[0] as f64, he[1] as f64, he[2] as f64)
        }
        #[pygetset]
        fn full_extents(&self) -> (f64, f64, f64) {
            self.0.full_extents()
        }
        #[pygetset]
        fn axis_aligned(&self) -> bool {
            self.0.axis_aligned
        }
        #[pygetset]
        fn orientation(&self) -> PyDMat3 {
            let a = &self.0.axes;
            PyDMat3(glam::DMat3::from_cols(
                glam::DVec3::new(a[0].x as f64, a[0].y as f64, a[0].z as f64),
                glam::DVec3::new(a[1].x as f64, a[1].y as f64, a[1].z as f64),
                glam::DVec3::new(a[2].x as f64, a[2].y as f64, a[2].z as f64),
            ))
        }
        #[pygetset]
        fn axes(&self) -> ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64)) {
            let a = &self.0.axes;
            (
                (a[0].x as f64, a[0].y as f64, a[0].z as f64),
                (a[1].x as f64, a[1].y as f64, a[1].z as f64),
                (a[2].x as f64, a[2].y as f64, a[2].z as f64),
            )
        }
        #[pymethod]
        fn contains_point(&self, point: PyObjectRef, vm: &VirtualMachine) -> PyResult<bool> {
            Ok(self.0.contains_point(dv3(extract_vec3(&point, vm)?)))
        }
        #[pymethod]
        fn point_dist_sq(&self, point: PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
            Ok(self.0.point_dist_sq(dv3(extract_vec3(&point, vm)?)) as f64)
        }
        #[pymethod]
        fn bounding_sphere_radius(&self) -> f64 {
            self.0.bounding_sphere_radius() as f64
        }
        #[pymethod]
        fn corners(&self, vm: &VirtualMachine) -> PyObjectRef {
            let items: Vec<PyObjectRef> = self
                .0
                .corners()
                .into_iter()
                .map(|c| PyDVec3(c).into_pyobject(vm))
                .collect();
            vm.ctx.new_list(items).into()
        }
        #[pystaticmethod]
        fn from_aabb(min: PyObjectRef, max: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            Ok(Self(Cuboid::from_aabb(
                dv3(extract_vec3(&min, vm)?),
                dv3(extract_vec3(&max, vm)?),
            )))
        }
        #[pystaticmethod]
        fn from_center_size_orientation(
            center: PyObjectRef,
            size: (f64, f64, f64),
            orientation: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Self> {
            let c = extract_vec3(&center, vm)?;
            let o = extract_mat3(&orientation, vm)?;
            Ok(Self(Cuboid::from_center_size_orientation(c, size, o)))
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
            shape_collides(&self.0, &other, vm)
        }
        #[pymethod]
        fn stretch(&self, translation: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            let t = dv3(extract_vec3(&translation, vm)?);
            let items: Vec<PyObjectRef> = match self.0.stretch(t) {
                CuboidStretch::Aligned(c) => vec![PyCuboid(c).into_pyobject(vm)],
                CuboidStretch::Unaligned(p) => vec![PyConvexPolytope(p).into_pyobject(vm)],
            };
            Ok(vm.ctx.new_list(items).into())
        }
        #[pymethod]
        fn abs_diff_eq(
            &self,
            other: PyObjectRef,
            max_abs_diff: f64,
            vm: &VirtualMachine,
        ) -> PyResult<bool> {
            let o = other
                .downcast_ref::<PyCuboid>()
                .ok_or_else(|| vm.new_type_error("expected Cuboid".to_owned()))?;
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
            crate::rp_serde::dataclass_fields(&["center", "axes", "half_extents"], vm)
        }
    }
}
