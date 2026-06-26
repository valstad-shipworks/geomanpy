//! `Collider` wrapper.

use wreck::{Collider, Pointcloud};

#[cfg_attr(
    feature = "pyo3-backend",
    pyo3::pyclass(skip_from_py_object, name = "Collider")
)]
#[cfg_attr(
    feature = "rustpython-backend",
    rustpython_vm::pyclass(module = "geomanpy", name = "Collider")
)]
#[cfg_attr(feature = "rustpython-backend", derive(rustpython_vm::PyPayload))]
#[derive(Debug, Clone)]
pub struct PyCollider(pub Collider<Pointcloud>);

#[cfg(feature = "pyo3-backend")]
mod pyo3_impl {
    use super::*;
    use crate::glam_wrappers::PyDVec3;
    use crate::pickle::pickle_decode;
    use crate::wreck_wrappers::{
        AnyShape, PyCapsule, PyConvexPolygon, PyConvexPolytope, PyCuboid, PyCylinder, PyLine,
        PyLineSegment, PyPlane, PyPointcloud, PyRay, PyShape, PySphereCollection,
    };
    use pyo3::PyResult;
    use pyo3::prelude::*;

    #[pymethods]
    impl PyCollider {
        #[new]
        #[pyo3(signature = (*, __pickle_state__=None))]
        fn new(__pickle_state__: Option<Vec<u8>>) -> PyResult<Self> {
            if let Some(state) = __pickle_state__ {
                return Ok(Self(pickle_decode::<Collider<Pointcloud>>(&state)?));
            }
            Ok(Self(Collider::new()))
        }
        #[staticmethod]
        fn from_any(obstacles: PyCollider) -> Self {
            obstacles
        }
        fn add(&mut self, shape: PyShape) {
            AnyShape::from(shape).push_into(&mut self.0);
        }
        fn include(&mut self, other: PyCollider) {
            self.0.include(other.0);
        }
        fn merge(&self, other: PyCollider) -> Self {
            let mut out = self.0.clone();
            out.include(other.0);
            Self(out)
        }
        fn with_any(&self, obstacle: PyCollider) -> Self {
            let mut out = self.0.clone();
            out.include(obstacle.0);
            Self(out)
        }
        fn collides(&self, shape: PyShape) -> PyResult<bool> {
            AnyShape::from(shape).query_collider(&self.0).ok_or_else(|| {
                pyo3::exceptions::PyValueError::new_err(
                    "Pointcloud cannot query a Collider<Pointcloud>; use individual shape queries instead",
                )
            })
        }
        fn collides_other(&self, other: PyCollider) -> bool {
            self.0.collides_other(&other.0)
        }
        fn refine_bounding(&mut self) {
            self.0.refine_bounding();
        }
        fn mask(&self) -> u16 {
            self.0.mask()
        }
        fn capsules(&self) -> Vec<PyCapsule> {
            self.0.capsules().iter().map(|c| PyCapsule(*c)).collect()
        }
        fn cuboids(&self) -> Vec<PyCuboid> {
            self.0.cuboids().iter().map(|c| PyCuboid(*c)).collect()
        }
        fn cylinders(&self) -> Vec<PyCylinder> {
            self.0.cylinders().iter().map(|c| PyCylinder(*c)).collect()
        }
        fn planes(&self) -> Vec<PyPlane> {
            self.0.planes().iter().map(|p| PyPlane(*p)).collect()
        }
        fn try_stretch_d(&self, translation: PyDVec3) -> Option<Self> {
            self.0.try_stretch_d(translation.0).map(|c| Self(c.into()))
        }
        fn polygons(&self) -> Vec<PyConvexPolygon> {
            self.0
                .polygons()
                .iter()
                .map(|p| PyConvexPolygon(p.clone()))
                .collect()
        }
        fn polytopes(&self) -> Vec<PyConvexPolytope> {
            self.0
                .polytopes()
                .iter()
                .map(|p| PyConvexPolytope(p.clone()))
                .collect()
        }
        fn lines(&self) -> Vec<PyLine> {
            self.0.lines().iter().map(|l| PyLine(*l)).collect()
        }
        fn rays(&self) -> Vec<PyRay> {
            self.0.rays().iter().map(|r| PyRay(*r)).collect()
        }
        fn segments(&self) -> Vec<PyLineSegment> {
            self.0
                .segments()
                .iter()
                .map(|s| PyLineSegment(*s))
                .collect()
        }
        fn pointclouds(&self) -> Vec<PyPointcloud> {
            self.0
                .pointclouds()
                .iter()
                .map(|p| PyPointcloud(p.clone()))
                .collect()
        }
        fn spheres(&self) -> PySphereCollection {
            PySphereCollection(self.0.spheres().clone())
        }
        fn __repr__(&self) -> String {
            format!("Collider(mask=0x{:04x})", self.0.mask())
        }
    }
}

#[cfg(feature = "rustpython-backend")]
mod rustpython_impl {
    use super::*;
    use crate::glam_wrappers::quat::extract_quat;
    use crate::glam_wrappers::vec3::extract_vec3;
    use crate::wreck_wrappers::rustpython_glue::{
        add_to_collider, extract_affine3, extract_mat3, shape_collides_collider,
    };
    use crate::wreck_wrappers::{
        AnyShape, PyCapsule, PyConvexPolygon, PyConvexPolytope, PyCuboid, PyCylinder, PyLine,
        PyLineSegment, PyPlane, PyPointcloud, PyRay, PySphere, PySphereCollection,
    };
    use rustpython_vm::{
        Py, PyObjectRef, PyResult, VirtualMachine,
        builtins::PyType,
        function::FuncArgs,
        pyclass,
        types::{Constructor, Representable},
    };
    use wreck::{Bounded, Scalable, Transformable};

    /// Fold a single shape, another `Collider`, a sequence of these, or `None`
    /// into `c`. Mirrors the pyo3 `Collider` `FromPyObject` extraction.
    fn extend_any(
        c: &mut Collider<Pointcloud>,
        obj: &PyObjectRef,
        vm: &VirtualMachine,
    ) -> PyResult<()> {
        if vm.is_none(obj) {
            return Ok(());
        }
        if let Some(other) = obj.downcast_ref::<PyCollider>() {
            c.include(other.0.clone());
            return Ok(());
        }
        if let Ok(shape) = AnyShape::try_from_object(obj, vm) {
            shape.push_into(c);
            return Ok(());
        }
        let seq: Vec<PyObjectRef> = obj.try_to_value(vm).map_err(|_| {
            vm.new_type_error(
                "expected a Shape, primitive, Collider, or a sequence of these".to_owned(),
            )
        })?;
        for item in &seq {
            if let Some(other) = item.downcast_ref::<PyCollider>() {
                c.include(other.0.clone());
            } else {
                add_to_collider(c, item, vm)?;
            }
        }
        Ok(())
    }

    impl Constructor for PyCollider {
        type Args = FuncArgs;
        fn py_new(_cls: &Py<PyType>, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
            if let Some(state) = crate::rp_serde::take_pickle_state(&args, vm)? {
                return Ok(Self(
                    crate::pickle::pickle_decode_raw::<Collider<Pointcloud>>(&state)
                        .map_err(|e| vm.new_value_error(e))?,
                ));
            }
            Ok(Self(Collider::new()))
        }
    }
    impl Representable for PyCollider {
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok(format!("Collider(mask=0x{:04x})", zelf.0.mask()))
        }
    }
    #[pyclass(with(Constructor, Representable))]
    impl PyCollider {
        #[pymethod]
        fn mask(&self) -> u16 {
            self.0.mask()
        }
        #[pymethod]
        fn try_stretch_d(
            &self,
            translation: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<Option<Self>> {
            let t = extract_vec3(&translation, vm)?;
            Ok(self.0.try_stretch_d(t).map(|c| Self(c.into())))
        }
        #[pymethod]
        fn __getnewargs_ex__(&self, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            crate::rp_serde::getnewargs_ex(&self.0, vm)
        }
        #[pygetset]
        fn __dataclass_fields__(&self, vm: &VirtualMachine) -> PyObjectRef {
            crate::rp_serde::dataclass_fields(&[], vm)
        }

        /// Add a shape and return the new Collider.
        ///
        /// Note: pyo3 mutates in place; under rustpython we return a new
        /// Collider because `#[pymethod]` only gives us `&self`. Use the
        /// returned value for chaining.
        #[pymethod]
        fn add(&self, shape: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let mut out = self.0.clone();
            add_to_collider(&mut out, &shape, vm)?;
            Ok(Self(out))
        }

        /// Merge another Collider into a new Collider.
        #[pymethod]
        fn include(&self, other: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let other = other
                .downcast_ref::<PyCollider>()
                .ok_or_else(|| vm.new_type_error("expected Collider".to_owned()))?;
            let mut out = self.0.clone();
            out.include(other.0.clone());
            Ok(Self(out))
        }

        /// Build a Collider from a single shape, another Collider, a sequence
        /// of these, or None.
        #[pystaticmethod]
        fn from_any(obstacles: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let mut out = Collider::new();
            extend_any(&mut out, &obstacles, vm)?;
            Ok(Self(out))
        }

        /// Merge another Collider (or any obstacle) into a new Collider.
        #[pymethod]
        fn merge(&self, other: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let mut out = self.0.clone();
            extend_any(&mut out, &other, vm)?;
            Ok(Self(out))
        }

        /// New Collider combining this one with any obstacle (shape, sequence,
        /// Collider, or None).
        #[pymethod]
        fn with_any(&self, obstacle: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
            let mut out = self.0.clone();
            extend_any(&mut out, &obstacle, vm)?;
            Ok(Self(out))
        }

        /// Refine bounding volumes — returns a new Collider.
        #[pymethod]
        fn refine_bounding(&self) -> Self {
            let mut out = self.0.clone();
            out.refine_bounding();
            Self(out)
        }

        /// Test whether any contained shape collides with the given shape.
        #[pymethod]
        fn collides(&self, shape: PyObjectRef, vm: &VirtualMachine) -> PyResult<bool> {
            shape_collides_collider(&self.0, &shape, vm)
        }

        /// Collider vs Collider.
        #[pymethod]
        fn collides_other(&self, other: PyObjectRef, vm: &VirtualMachine) -> PyResult<bool> {
            let other = other
                .downcast_ref::<PyCollider>()
                .ok_or_else(|| vm.new_type_error("expected Collider".to_owned()))?;
            Ok(self.0.collides_other(&other.0))
        }

        #[pymethod]
        fn spheres(&self) -> PySphereCollection {
            PySphereCollection(self.0.spheres().clone())
        }
        #[pymethod]
        fn capsules(&self, vm: &VirtualMachine) -> PyObjectRef {
            use rustpython_vm::PyPayload;
            let items: Vec<PyObjectRef> = self
                .0
                .capsules()
                .iter()
                .copied()
                .map(|c| PyCapsule(c).into_pyobject(vm))
                .collect();
            vm.ctx.new_list(items).into()
        }
        #[pymethod]
        fn cuboids(&self, vm: &VirtualMachine) -> PyObjectRef {
            use rustpython_vm::PyPayload;
            let items: Vec<PyObjectRef> = self
                .0
                .cuboids()
                .iter()
                .copied()
                .map(|c| PyCuboid(c).into_pyobject(vm))
                .collect();
            vm.ctx.new_list(items).into()
        }
        #[pymethod]
        fn cylinders(&self, vm: &VirtualMachine) -> PyObjectRef {
            use rustpython_vm::PyPayload;
            let items: Vec<PyObjectRef> = self
                .0
                .cylinders()
                .iter()
                .copied()
                .map(|c| PyCylinder(c).into_pyobject(vm))
                .collect();
            vm.ctx.new_list(items).into()
        }
        #[pymethod]
        fn polytopes(&self, vm: &VirtualMachine) -> PyObjectRef {
            use rustpython_vm::PyPayload;
            let items: Vec<PyObjectRef> = self
                .0
                .polytopes()
                .iter()
                .map(|p| PyConvexPolytope(p.clone()).into_pyobject(vm))
                .collect();
            vm.ctx.new_list(items).into()
        }
        #[pymethod]
        fn polygons(&self, vm: &VirtualMachine) -> PyObjectRef {
            use rustpython_vm::PyPayload;
            let items: Vec<PyObjectRef> = self
                .0
                .polygons()
                .iter()
                .map(|p| PyConvexPolygon(p.clone()).into_pyobject(vm))
                .collect();
            vm.ctx.new_list(items).into()
        }
        #[pymethod]
        fn lines(&self, vm: &VirtualMachine) -> PyObjectRef {
            use rustpython_vm::PyPayload;
            let items: Vec<PyObjectRef> = self
                .0
                .lines()
                .iter()
                .copied()
                .map(|l| PyLine(l).into_pyobject(vm))
                .collect();
            vm.ctx.new_list(items).into()
        }
        #[pymethod]
        fn rays(&self, vm: &VirtualMachine) -> PyObjectRef {
            use rustpython_vm::PyPayload;
            let items: Vec<PyObjectRef> = self
                .0
                .rays()
                .iter()
                .copied()
                .map(|r| PyRay(r).into_pyobject(vm))
                .collect();
            vm.ctx.new_list(items).into()
        }
        #[pymethod]
        fn segments(&self, vm: &VirtualMachine) -> PyObjectRef {
            use rustpython_vm::PyPayload;
            let items: Vec<PyObjectRef> = self
                .0
                .segments()
                .iter()
                .copied()
                .map(|s| PyLineSegment(s).into_pyobject(vm))
                .collect();
            vm.ctx.new_list(items).into()
        }
        #[pymethod]
        fn planes(&self, vm: &VirtualMachine) -> PyObjectRef {
            use rustpython_vm::PyPayload;
            let items: Vec<PyObjectRef> = self
                .0
                .planes()
                .iter()
                .copied()
                .map(|p| PyPlane(p).into_pyobject(vm))
                .collect();
            vm.ctx.new_list(items).into()
        }
        #[pymethod]
        fn pointclouds(&self, vm: &VirtualMachine) -> PyObjectRef {
            use rustpython_vm::PyPayload;
            let items: Vec<PyObjectRef> = self
                .0
                .pointclouds()
                .iter()
                .map(|p| PyPointcloud(p.clone()).into_pyobject(vm))
                .collect();
            vm.ctx.new_list(items).into()
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
    }
}
