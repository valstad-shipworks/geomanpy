//! RustPython backend module registration.
//!
//! All type implementations live alongside their pyo3 counterparts in
//! `crate::glam_wrappers::*` and `crate::wreck_wrappers::*`. This module
//! exposes them under the `geomanpy` module name.

/// Build the `geomanpy` module definition for embedding into a
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
    #[pyattr(name = "Shape")]
    fn shape_type(_vm: &VirtualMachine) -> PyTypeRef {
        crate::wreck_wrappers::PyShape::make_static_type()
    }

    // Domain-name aliases, mirroring the pure-Python facade in
    // `py_src/geomanpy/__init__.py` so an embedded module exposes the same
    // public names as the CPython package without layering that facade on top.
    // Each resolves to the same static type object as its canonical name.
    #[pyattr(name = "Quaternion")]
    fn quaternion_type(vm: &VirtualMachine) -> PyTypeRef {
        quat_type(vm)
    }
    #[pyattr(name = "Rotation3d")]
    fn rotation3d_type(vm: &VirtualMachine) -> PyTypeRef {
        mat3_type(vm)
    }
    #[pyattr(name = "Translation3d")]
    fn translation3d_type(vm: &VirtualMachine) -> PyTypeRef {
        vec3_type(vm)
    }
    #[pyattr(name = "Transform3d")]
    fn transform3d_type(vm: &VirtualMachine) -> PyTypeRef {
        affine3_type(vm)
    }
    #[pyattr(name = "Box3d")]
    fn box3d_type(vm: &VirtualMachine) -> PyTypeRef {
        cuboid_type(vm)
    }
    #[pyattr(name = "Sphere3d")]
    fn sphere3d_type(vm: &VirtualMachine) -> PyTypeRef {
        sphere_type(vm)
    }
    #[pyattr(name = "Cylinder3d")]
    fn cylinder3d_type(vm: &VirtualMachine) -> PyTypeRef {
        cylinder_type(vm)
    }
    #[pyattr(name = "ObstacleUnion")]
    fn obstacle_union_type(vm: &VirtualMachine) -> PyTypeRef {
        collider_type(vm)
    }
    #[pyattr(name = "PointCloud")]
    fn point_cloud_type(vm: &VirtualMachine) -> PyTypeRef {
        pointcloud_type(vm)
    }
}
