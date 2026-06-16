//! RustPython backend module registration.
//!
//! All type implementations live alongside their pyo3 counterparts in
//! `crate::glam_wrappers::*` and `crate::wreck_wrappers::*`. This module
//! exposes them under the `geomanpy` module name.

/// Build the `_geomanpy` module definition for embedding into a
/// [`rustpython_vm::Interpreter`].
pub fn make_module(ctx: &rustpython_vm::Context) -> &'static rustpython_vm::builtins::PyModuleDef {
    geomanpy_module::module_def(ctx)
}

#[rustpython_vm::pymodule(name = "geomanpy")]
pub(crate) mod geomanpy_module {
    use rustpython_vm::{VirtualMachine, builtins::PyTypeRef, class::PyClassImpl};

    #[pyattr(name = "EulerRot")]
    fn euler_rot_type(vm: &VirtualMachine) -> PyTypeRef {
        let t = crate::glam_wrappers::PyEulerRot::make_static_type();
        crate::glam_wrappers::install_euler_constants(&t, vm);
        t
    }
    #[pyattr(name = "Vec2")]
    fn vec2_type(vm: &VirtualMachine) -> PyTypeRef {
        let t = crate::glam_wrappers::PyDVec2::make_static_type();
        crate::glam_wrappers::vec2::install_constants(&t, vm);
        t
    }
    #[pyattr(name = "Vec3")]
    fn vec3_type(vm: &VirtualMachine) -> PyTypeRef {
        let t = crate::glam_wrappers::PyDVec3::make_static_type();
        crate::glam_wrappers::vec3::install_constants(&t, vm);
        t
    }
    #[pyattr(name = "Vec4")]
    fn vec4_type(vm: &VirtualMachine) -> PyTypeRef {
        let t = crate::glam_wrappers::PyDVec4::make_static_type();
        crate::glam_wrappers::vec4::install_constants(&t, vm);
        t
    }
    #[pyattr(name = "Quat")]
    fn quat_type(vm: &VirtualMachine) -> PyTypeRef {
        let t = crate::glam_wrappers::PyDQuat::make_static_type();
        crate::glam_wrappers::quat::install_constants(&t, vm);
        t
    }
    #[pyattr(name = "Mat3")]
    fn mat3_type(vm: &VirtualMachine) -> PyTypeRef {
        let t = crate::glam_wrappers::PyDMat3::make_static_type();
        crate::glam_wrappers::mat3::install_constants(&t, vm);
        t
    }
    #[pyattr(name = "Mat4")]
    fn mat4_type(vm: &VirtualMachine) -> PyTypeRef {
        let t = crate::glam_wrappers::PyDMat4::make_static_type();
        crate::glam_wrappers::mat4::install_constants(&t, vm);
        t
    }
    #[pyattr(name = "Affine3")]
    fn affine3_type(vm: &VirtualMachine) -> PyTypeRef {
        let t = crate::glam_wrappers::PyDAffine3::make_static_type();
        crate::glam_wrappers::affine3::install_constants(&t, vm);
        t
    }

    #[pyattr(name = "Sphere")]
    fn sphere_type(_vm: &VirtualMachine) -> PyTypeRef {
        crate::wreck_wrappers::PySphere::make_static_type()
    }
    #[pyattr(name = "Capsule")]
    fn capsule_type(_vm: &VirtualMachine) -> PyTypeRef {
        crate::wreck_wrappers::PyCapsule::make_static_type()
    }
    #[pyattr(name = "Cuboid")]
    fn cuboid_type(_vm: &VirtualMachine) -> PyTypeRef {
        crate::wreck_wrappers::PyCuboid::make_static_type()
    }
    #[pyattr(name = "Cylinder")]
    fn cylinder_type(_vm: &VirtualMachine) -> PyTypeRef {
        crate::wreck_wrappers::PyCylinder::make_static_type()
    }
    #[pyattr(name = "ConvexPolytope")]
    fn convex_polytope_type(_vm: &VirtualMachine) -> PyTypeRef {
        crate::wreck_wrappers::PyConvexPolytope::make_static_type()
    }
    #[pyattr(name = "ConvexPolygon")]
    fn convex_polygon_type(_vm: &VirtualMachine) -> PyTypeRef {
        crate::wreck_wrappers::PyConvexPolygon::make_static_type()
    }
    #[pyattr(name = "Line")]
    fn line_type(_vm: &VirtualMachine) -> PyTypeRef {
        crate::wreck_wrappers::PyLine::make_static_type()
    }
    #[pyattr(name = "Ray")]
    fn ray_type(_vm: &VirtualMachine) -> PyTypeRef {
        crate::wreck_wrappers::PyRay::make_static_type()
    }
    #[pyattr(name = "LineSegment")]
    fn line_segment_type(_vm: &VirtualMachine) -> PyTypeRef {
        crate::wreck_wrappers::PyLineSegment::make_static_type()
    }
    #[pyattr(name = "Plane")]
    fn plane_type(_vm: &VirtualMachine) -> PyTypeRef {
        crate::wreck_wrappers::PyPlane::make_static_type()
    }
    #[pyattr(name = "Pointcloud")]
    fn pointcloud_type(_vm: &VirtualMachine) -> PyTypeRef {
        crate::wreck_wrappers::PyPointcloud::make_static_type()
    }
    #[pyattr(name = "SphereCollection")]
    fn sphere_collection_type(_vm: &VirtualMachine) -> PyTypeRef {
        crate::wreck_wrappers::PySphereCollection::make_static_type()
    }
    #[pyattr(name = "Collider")]
    fn collider_type(_vm: &VirtualMachine) -> PyTypeRef {
        crate::wreck_wrappers::PyCollider::make_static_type()
    }
}
