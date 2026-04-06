use glam::Vec3;
use pyo3::Bound;
use pyo3::PyResult;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use wreck::{
    Bounded, Capsule, Collider, Collides, ConvexPolygon, ConvexPolytope, Cuboid, Cylinder, Line,
    LineSegment, Plane, Pointcloud, Ray, Scalable, Sphere, Stretchable, Transformable,
    soa::SpheresSoA,
    stretched::{
        CapsuleStretch, ConvexPolygonStretch, CuboidStretch, CylinderStretch, LineSegmentStretch,
        LineStretch, RayStretch, SphereStretch,
    },
};

use crate::glam_wrappers::{PyDAffine3, PyDMat3, PyDQuat, PyDVec3};

#[inline]
fn dv3(v: PyDVec3) -> Vec3 {
    v.0.as_vec3()
}

#[inline]
fn v3d(v: Vec3) -> PyDVec3 {
    PyDVec3(glam::DVec3::new(v.x as f64, v.y as f64, v.z as f64))
}

#[pyclass(from_py_object, name = "Collider")]
#[derive(Debug, Clone)]
pub struct PyCollider(pub(crate) Collider<Pointcloud>);

#[pyclass(from_py_object, name = "Pointcloud")]
#[derive(Debug, Clone)]
pub struct PyPointcloud(pub(crate) Pointcloud);

#[pyclass(from_py_object, name = "Sphere")]
#[derive(Debug, Clone, Copy)]
pub struct PySphere(pub(crate) Sphere);

#[pyclass(from_py_object, name = "Capsule")]
#[derive(Debug, Clone, Copy)]
pub struct PyCapsule(pub(crate) Capsule);

#[pyclass(from_py_object, name = "Cuboid")]
#[derive(Debug, Clone, Copy)]
pub struct PyCuboid(pub(crate) Cuboid);

#[pyclass(from_py_object, name = "Cylinder")]
#[derive(Debug, Clone, Copy)]
pub struct PyCylinder(pub(crate) Cylinder);

#[pyclass(from_py_object, name = "ConvexPolytope")]
#[derive(Debug, Clone)]
pub struct PyConvexPolytope(pub(crate) ConvexPolytope);

#[pyclass(from_py_object, name = "ConvexPolygon")]
#[derive(Debug, Clone)]
pub struct PyConvexPolygon(pub(crate) ConvexPolygon);

#[pyclass(from_py_object, name = "Line")]
#[derive(Debug, Clone, Copy)]
pub struct PyLine(pub(crate) Line);

#[pyclass(from_py_object, name = "Ray")]
#[derive(Debug, Clone, Copy)]
pub struct PyRay(pub(crate) Ray);

#[pyclass(from_py_object, name = "LineSegment")]
#[derive(Debug, Clone, Copy)]
pub struct PyLineSegment(pub(crate) LineSegment);

#[pyclass(from_py_object, name = "Plane")]
#[derive(Debug, Clone, Copy)]
pub struct PyPlane(pub(crate) Plane);

#[pyclass(from_py_object, name = "SphereCollection")]
#[derive(Debug, Clone)]
pub struct PySphereCollection(pub(crate) SpheresSoA);

macro_rules! impl_from_wreck {
    ($py:ty, $inner:ty) => {
        impl From<$inner> for $py {
            #[inline]
            fn from(v: $inner) -> Self {
                Self(v)
            }
        }
        impl From<$py> for $inner {
            #[inline]
            fn from(v: $py) -> Self {
                v.0
            }
        }
    };
}

impl_from_wreck!(PySphere, Sphere);
impl_from_wreck!(PyCapsule, Capsule);
impl_from_wreck!(PyCuboid, Cuboid);
impl_from_wreck!(PyCylinder, Cylinder);
impl_from_wreck!(PyConvexPolytope, ConvexPolytope);
impl_from_wreck!(PyConvexPolygon, ConvexPolygon);
impl_from_wreck!(PyLine, Line);
impl_from_wreck!(PyRay, Ray);
impl_from_wreck!(PyLineSegment, LineSegment);
impl_from_wreck!(PyPlane, Plane);
impl_from_wreck!(PyPointcloud, Pointcloud);
impl_from_wreck!(PySphereCollection, SpheresSoA);
impl_from_wreck!(PyCollider, Collider<Pointcloud>);

#[pyclass(from_py_object, name = "Shape")]
#[derive(Debug, Clone)]
pub enum PyShape {
    Sphere(PySphere),
    Capsule(PyCapsule),
    Cuboid(PyCuboid),
    Cylinder(PyCylinder),
    ConvexPolytope(PyConvexPolytope),
    ConvexPolygon(PyConvexPolygon),
    Line(PyLine),
    Ray(PyRay),
    LineSegment(PyLineSegment),
    Plane(PyPlane),
    Pointcloud(PyPointcloud),
}

macro_rules! impl_transform_scale_py {
    ($ty:ty) => {
        #[pymethods]
        impl $ty {
            fn scale(&mut self, factor: f64) {
                self.0.scale_d(factor);
            }
            fn scaled(&self, factor: f64) -> Self {
                Self(self.0.clone().scaled_d(factor))
            }
            fn translate(&mut self, offset: PyDVec3) {
                self.0.translate_d(offset.0);
            }
            fn translated(&self, offset: PyDVec3) -> Self {
                Self(self.0.clone().translated_d(offset.0))
            }
            fn rotate_mat(&mut self, mat: PyDMat3) {
                self.0.rotate_mat_d(mat.0);
            }
            fn rotated_mat(&self, mat: PyDMat3) -> Self {
                Self(self.0.clone().rotated_mat_d(mat.0))
            }
            fn rotate_quat(&mut self, quat: PyDQuat) {
                self.0.rotate_quat_d(quat.0);
            }
            fn rotated_quat(&self, quat: PyDQuat) -> Self {
                Self(self.0.clone().rotated_quat_d(quat.0))
            }
            fn transform(&mut self, mat: PyDAffine3) {
                self.0.transform_d(mat.0);
            }
            fn transformed(&self, mat: PyDAffine3) -> Self {
                Self(self.0.clone().transformed_d(mat.0))
            }
        }
    };
}

macro_rules! impl_bounded_py {
    ($ty:ty) => {
        #[pymethods]
        impl $ty {
            fn broadphase(&self) -> PySphere {
                PySphere(self.0.broadphase())
            }
            fn obb(&self) -> PyCuboid {
                PyCuboid(self.0.obb())
            }
            fn aabb(&self) -> PyCuboid {
                PyCuboid(self.0.aabb())
            }
        }
    };
}

macro_rules! impl_collides_all {
    ($ty:ty) => {
        #[pymethods]
        impl $ty {
            fn collides(&self, other: &PyShape) -> bool {
                match other {
                    PyShape::Sphere(s) => self.0.collides(&s.0),
                    PyShape::Capsule(c) => self.0.collides(&c.0),
                    PyShape::Cuboid(c) => self.0.collides(&c.0),
                    PyShape::Cylinder(c) => self.0.collides(&c.0),
                    PyShape::ConvexPolytope(p) => self.0.collides(&p.0),
                    PyShape::ConvexPolygon(p) => self.0.collides(&p.0),
                    PyShape::Line(l) => self.0.collides(&l.0),
                    PyShape::Ray(r) => self.0.collides(&r.0),
                    PyShape::LineSegment(s) => self.0.collides(&s.0),
                    PyShape::Plane(p) => self.0.collides(&p.0),
                    PyShape::Pointcloud(p) => self.0.collides(&p.0),
                }
            }
        }
    };
}

macro_rules! impl_collides_no_pcl_self {
    ($ty:ty) => {
        #[pymethods]
        impl $ty {
            fn collides(&self, other: &PyShape) -> PyResult<bool> {
                match other {
                    PyShape::Sphere(s) => Ok(self.0.collides(&s.0)),
                    PyShape::Capsule(c) => Ok(self.0.collides(&c.0)),
                    PyShape::Cuboid(c) => Ok(self.0.collides(&c.0)),
                    PyShape::Cylinder(c) => Ok(self.0.collides(&c.0)),
                    PyShape::ConvexPolytope(p) => Ok(self.0.collides(&p.0)),
                    PyShape::ConvexPolygon(p) => Ok(self.0.collides(&p.0)),
                    PyShape::Line(l) => Ok(self.0.collides(&l.0)),
                    PyShape::Ray(r) => Ok(self.0.collides(&r.0)),
                    PyShape::LineSegment(s) => Ok(self.0.collides(&s.0)),
                    PyShape::Plane(p) => Ok(self.0.collides(&p.0)),
                    PyShape::Pointcloud(_) => Err(pyo3::exceptions::PyValueError::new_err(
                        "Pointcloud-Pointcloud collision is not supported",
                    )),
                }
            }
        }
    };
}

impl_transform_scale_py!(PySphere);
impl_transform_scale_py!(PyCapsule);
impl_transform_scale_py!(PyCuboid);
impl_transform_scale_py!(PyCylinder);
impl_transform_scale_py!(PyConvexPolytope);
impl_transform_scale_py!(PyConvexPolygon);
impl_transform_scale_py!(PyLine);
impl_transform_scale_py!(PyRay);
impl_transform_scale_py!(PyLineSegment);
impl_transform_scale_py!(PyPlane);
impl_transform_scale_py!(PyPointcloud);
impl_transform_scale_py!(PyCollider);

impl_bounded_py!(PySphere);
impl_bounded_py!(PyCapsule);
impl_bounded_py!(PyCuboid);
impl_bounded_py!(PyCylinder);
impl_bounded_py!(PyConvexPolytope);
impl_bounded_py!(PyConvexPolygon);
impl_bounded_py!(PyLineSegment);
impl_bounded_py!(PyPointcloud);
impl_bounded_py!(PyCollider);

impl_collides_all!(PySphere);
impl_collides_all!(PyCapsule);
impl_collides_all!(PyCuboid);
impl_collides_all!(PyCylinder);
impl_collides_all!(PyConvexPolytope);
impl_collides_all!(PyConvexPolygon);
impl_collides_all!(PyLine);
impl_collides_all!(PyRay);
impl_collides_all!(PyLineSegment);
impl_collides_all!(PyPlane);
impl_collides_no_pcl_self!(PyPointcloud);

macro_rules! impl_approx_py {
    ($ty:ty) => {
        #[pymethods]
        impl $ty {
            #[inline]
            fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f64) -> bool {
                approx::AbsDiffEq::abs_diff_eq(&self.0, &rhs.0, max_abs_diff as f32)
            }
            #[inline]
            fn relative_eq(&self, rhs: Self, max_abs_diff: f64, max_relative: f64) -> bool {
                approx::RelativeEq::relative_eq(
                    &self.0,
                    &rhs.0,
                    max_abs_diff as f32,
                    max_relative as f32,
                )
            }
        }
    };
}

impl_approx_py!(PySphere);
impl_approx_py!(PyCapsule);
impl_approx_py!(PyCuboid);
impl_approx_py!(PyPlane);
impl_approx_py!(PyLine);
impl_approx_py!(PyRay);
impl_approx_py!(PyLineSegment);
impl_approx_py!(PyConvexPolygon);
impl_approx_py!(PyConvexPolytope);
impl_approx_py!(PySphereCollection);

#[pymethods]
impl PySphere {
    #[new]
    fn new(center: PyDVec3, radius: f64) -> Self {
        Self(Sphere::new_d(center.0, radius))
    }

    #[getter]
    fn center(&self) -> PyDVec3 {
        v3d(self.0.center)
    }

    #[getter]
    fn radius(&self) -> f64 {
        self.0.radius as f64
    }

    fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
        match self.0.stretch(dv3(translation)) {
            SphereStretch::NoStretch(s) => vec![PyShape::Sphere(PySphere(s))],
            SphereStretch::Stretch(c) => vec![PyShape::Capsule(PyCapsule(c))],
        }
    }

    fn __repr__(&self) -> String {
        self.0.to_string()
    }
}

#[pymethods]
impl PyCapsule {
    #[new]
    fn new(p1: PyDVec3, p2: PyDVec3, radius: f64) -> Self {
        Self(Capsule::new(dv3(p1), dv3(p2), radius as f32))
    }

    #[getter]
    fn p1(&self) -> PyDVec3 {
        v3d(self.0.p1)
    }

    #[getter]
    fn p2(&self) -> PyResult<PyDVec3> {
        Ok(v3d(self.0.p2()))
    }

    #[getter]
    fn radius(&self) -> f64 {
        self.0.radius as f64
    }

    fn closest_point_to(&self, point: PyDVec3) -> PyResult<PyDVec3> {
        Ok(v3d(self.0.closest_point_to(dv3(point))))
    }

    fn bounding_sphere(&self) -> PyResult<(PyDVec3, f64)> {
        let (c, r) = self.0.bounding_sphere();
        Ok((v3d(c), r as f64))
    }

    fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
        match self.0.stretch(dv3(translation)) {
            CapsuleStretch::Aligned(c) => vec![PyShape::Capsule(PyCapsule(c))],
            CapsuleStretch::Unaligned(edges, poly) => {
                let mut out: Vec<PyShape> = edges
                    .into_iter()
                    .map(|c| PyShape::Capsule(PyCapsule(c)))
                    .collect();
                out.push(PyShape::ConvexPolytope(PyConvexPolytope(poly)));
                out
            }
        }
    }

    fn __repr__(&self) -> String {
        self.0.to_string()
    }
}

#[pymethods]
impl PyCuboid {
    #[new]
    fn new(center: PyDVec3, axes: [[f64; 3]; 3], half_extents: [f64; 3]) -> Self {
        Self(Cuboid::new(
            dv3(center),
            [
                Vec3::new(axes[0][0] as f32, axes[0][1] as f32, axes[0][2] as f32),
                Vec3::new(axes[1][0] as f32, axes[1][1] as f32, axes[1][2] as f32),
                Vec3::new(axes[2][0] as f32, axes[2][1] as f32, axes[2][2] as f32),
            ],
            [
                half_extents[0] as f32,
                half_extents[1] as f32,
                half_extents[2] as f32,
            ],
        ))
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
    fn half_extents(&self) -> [f64; 3] {
        [
            self.0.half_extents[0] as f64,
            self.0.half_extents[1] as f64,
            self.0.half_extents[2] as f64,
        ]
    }

    #[getter]
    fn axis_aligned(&self) -> bool {
        self.0.axis_aligned
    }

    fn bounding_sphere_radius(&self) -> f64 {
        self.0.bounding_sphere_radius() as f64
    }

    fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
        match self.0.stretch(dv3(translation)) {
            CuboidStretch::Aligned(c) => vec![PyShape::Cuboid(PyCuboid(c))],
            CuboidStretch::Unaligned(p) => {
                vec![PyShape::ConvexPolytope(PyConvexPolytope(p))]
            }
        }
    }

    fn __repr__(&self) -> String {
        self.0.to_string()
    }
}

#[pymethods]
impl PyCylinder {
    #[new]
    fn new(p1: PyDVec3, p2: PyDVec3, radius: f64) -> Self {
        Self(Cylinder::new(dv3(p1), dv3(p2), radius as f32))
    }

    #[getter]
    fn p1(&self) -> PyDVec3 {
        v3d(self.0.p1)
    }

    fn p2(&self) -> PyResult<PyDVec3> {
        Ok(v3d(self.0.p2()))
    }

    #[getter]
    fn radius(&self) -> f64 {
        self.0.radius as f64
    }

    fn bounding_sphere(&self) -> PyResult<(PyDVec3, f64)> {
        let (c, r) = self.0.bounding_sphere();
        Ok((v3d(c), r as f64))
    }

    fn point_dist_sq(&self, point: PyDVec3) -> f64 {
        self.0.point_dist_sq(dv3(point)) as f64
    }

    fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
        match self.0.stretch(dv3(translation)) {
            CylinderStretch::Aligned(c) => vec![PyShape::Cylinder(PyCylinder(c))],
            CylinderStretch::Unaligned(edges, poly) => {
                let mut out: Vec<PyShape> = edges
                    .into_iter()
                    .map(|c| PyShape::Capsule(PyCapsule(c)))
                    .collect();
                out.push(PyShape::ConvexPolytope(PyConvexPolytope(poly)));
                out
            }
        }
    }

    fn __repr__(&self) -> String {
        self.0.to_string()
    }
}

#[pymethods]
impl PyConvexPolytope {
    #[new]
    fn new(planes: Vec<([f64; 3], f64)>, vertices: Vec<[f64; 3]>) -> Self {
        let planes: Vec<(Vec3, f32)> = planes
            .into_iter()
            .map(|(n, d)| (Vec3::new(n[0] as f32, n[1] as f32, n[2] as f32), d as f32))
            .collect();
        let vertices: Vec<Vec3> = vertices
            .into_iter()
            .map(|v| Vec3::new(v[0] as f32, v[1] as f32, v[2] as f32))
            .collect();
        Self(ConvexPolytope::new(planes, vertices))
    }

    #[staticmethod]
    fn with_obb(planes: Vec<([f64; 3], f64)>, vertices: Vec<[f64; 3]>, obb: PyCuboid) -> Self {
        let planes: Vec<(Vec3, f32)> = planes
            .into_iter()
            .map(|(n, d)| (Vec3::new(n[0] as f32, n[1] as f32, n[2] as f32), d as f32))
            .collect();
        let vertices: Vec<Vec3> = vertices
            .into_iter()
            .map(|v| Vec3::new(v[0] as f32, v[1] as f32, v[2] as f32))
            .collect();
        Self(ConvexPolytope::with_obb(planes, vertices, obb.0))
    }

    #[getter]
    fn planes(&self) -> Vec<([f64; 3], f64)> {
        self.0
            .planes
            .iter()
            .map(|(n, d)| ([n.x as f64, n.y as f64, n.z as f64], *d as f64))
            .collect()
    }

    #[getter]
    fn vertices(&self) -> Vec<[f64; 3]> {
        self.0
            .vertices
            .iter()
            .map(|v| [v.x as f64, v.y as f64, v.z as f64])
            .collect()
    }

    #[getter]
    fn get_obb(&self) -> PyCuboid {
        PyCuboid(self.0.obb)
    }

    fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
        vec![PyShape::ConvexPolytope(PyConvexPolytope(
            self.0.stretch(dv3(translation)),
        ))]
    }

    fn __repr__(&self) -> String {
        self.0.to_string()
    }
}

#[pymethods]
impl PyConvexPolygon {
    #[new]
    fn new(center: PyDVec3, normal: PyDVec3, vertices_2d: Vec<[f64; 2]>) -> Self {
        let verts: Vec<[f32; 2]> = vertices_2d
            .into_iter()
            .map(|v| [v[0] as f32, v[1] as f32])
            .collect();
        Self(ConvexPolygon::new(dv3(center), dv3(normal), verts))
    }

    #[staticmethod]
    fn with_axes(
        center: PyDVec3,
        normal: PyDVec3,
        u_axis: PyDVec3,
        v_axis: PyDVec3,
        vertices_2d: Vec<[f64; 2]>,
    ) -> Self {
        let verts: Vec<[f32; 2]> = vertices_2d
            .into_iter()
            .map(|v| [v[0] as f32, v[1] as f32])
            .collect();
        Self(ConvexPolygon::with_axes(
            dv3(center),
            dv3(normal),
            dv3(u_axis),
            dv3(v_axis),
            verts,
        ))
    }

    #[getter]
    fn center(&self) -> PyDVec3 {
        v3d(self.0.center)
    }

    #[getter]
    fn normal(&self) -> PyDVec3 {
        v3d(self.0.normal)
    }

    #[getter]
    fn u_axis(&self) -> PyDVec3 {
        v3d(self.0.u_axis)
    }

    #[getter]
    fn v_axis(&self) -> PyDVec3 {
        v3d(self.0.v_axis)
    }

    #[getter]
    fn vertices_2d(&self) -> Vec<[f64; 2]> {
        self.0
            .vertices_2d
            .iter()
            .map(|v| [v[0] as f64, v[1] as f64])
            .collect()
    }

    fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
        match self.0.stretch(dv3(translation)) {
            ConvexPolygonStretch::InPlane(p) => {
                vec![PyShape::ConvexPolygon(PyConvexPolygon(p))]
            }
            ConvexPolygonStretch::OutOfPlane(p) => {
                vec![PyShape::ConvexPolytope(PyConvexPolytope(p))]
            }
        }
    }

    fn __repr__(&self) -> String {
        self.0.to_string()
    }
}

#[pymethods]
impl PyLine {
    #[new]
    fn new(origin: PyDVec3, dir: PyDVec3) -> Self {
        Self(Line::new(dv3(origin), dv3(dir)))
    }

    #[staticmethod]
    fn from_points(a: PyDVec3, b: PyDVec3) -> Self {
        Self(Line::from_points(dv3(a), dv3(b)))
    }

    #[getter]
    fn origin(&self) -> PyDVec3 {
        v3d(self.0.origin)
    }

    #[getter]
    fn dir(&self) -> PyDVec3 {
        v3d(self.0.dir)
    }

    fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
        match self.0.stretch(dv3(translation)) {
            LineStretch::Parallel(l) => vec![PyShape::Line(PyLine(l))],
            LineStretch::Polygon(p) => vec![PyShape::ConvexPolygon(PyConvexPolygon(p))],
        }
    }

    fn __repr__(&self) -> String {
        self.0.to_string()
    }
}

#[pymethods]
impl PyRay {
    #[new]
    fn new(origin: PyDVec3, dir: PyDVec3) -> Self {
        Self(Ray::new(dv3(origin), dv3(dir)))
    }

    #[getter]
    fn origin(&self) -> PyDVec3 {
        v3d(self.0.origin)
    }

    #[getter]
    fn dir(&self) -> PyDVec3 {
        v3d(self.0.dir)
    }

    fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
        match self.0.stretch(dv3(translation)) {
            RayStretch::Parallel(r) => vec![PyShape::Ray(PyRay(r))],
            RayStretch::Polygon(p) => vec![PyShape::ConvexPolygon(PyConvexPolygon(p))],
        }
    }

    fn __repr__(&self) -> String {
        self.0.to_string()
    }
}

#[pymethods]
impl PyLineSegment {
    #[new]
    fn new(p1: PyDVec3, p2: PyDVec3) -> Self {
        Self(LineSegment::new(dv3(p1), dv3(p2)))
    }

    #[getter]
    fn p1(&self) -> PyDVec3 {
        v3d(self.0.p1)
    }

    fn p2(&self) -> PyResult<PyDVec3> {
        Ok(v3d(self.0.p2()))
    }

    fn bounding_sphere(&self) -> PyResult<(PyDVec3, f64)> {
        let (c, r) = self.0.bounding_sphere();
        Ok((v3d(c), r as f64))
    }

    fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
        match self.0.stretch(dv3(translation)) {
            LineSegmentStretch::Parallel(s) => vec![PyShape::LineSegment(PyLineSegment(s))],
            LineSegmentStretch::Polygon(p) => {
                vec![PyShape::ConvexPolygon(PyConvexPolygon(p))]
            }
        }
    }

    fn __repr__(&self) -> String {
        self.0.to_string()
    }
}

#[pymethods]
impl PyPlane {
    #[new]
    fn new(normal: PyDVec3, d: f64) -> Self {
        Self(Plane::new(dv3(normal), d as f32))
    }

    #[staticmethod]
    fn from_point_normal(point: PyDVec3, normal: PyDVec3) -> Self {
        Self(Plane::from_point_normal(dv3(point), dv3(normal)))
    }

    #[getter]
    fn normal(&self) -> PyDVec3 {
        v3d(self.0.normal)
    }

    #[getter]
    fn d(&self) -> f64 {
        self.0.d as f64
    }

    fn stretch(&self, translation: PyDVec3) -> Vec<PyShape> {
        vec![PyShape::Plane(PyPlane(self.0.stretch(dv3(translation))))]
    }

    fn __repr__(&self) -> String {
        self.0.to_string()
    }
}

#[pymethods]
impl PyPointcloud {
    #[new]
    fn new(points: Vec<[f32; 3]>, r_range: (f32, f32), point_radius: f32) -> Self {
        Self(Pointcloud::new(&points, r_range, point_radius))
    }

    fn __repr__(&self) -> String {
        "Pointcloud(...)".to_string()
    }
}

#[pymethods]
impl PySphereCollection {
    #[new]
    fn new() -> Self {
        Self(SpheresSoA::new())
    }

    #[staticmethod]
    fn with_capacity(cap: usize) -> Self {
        Self(SpheresSoA::with_capacity(cap))
    }

    #[staticmethod]
    fn from_spheres(spheres: Vec<PySphere>) -> Self {
        let inner: Vec<Sphere> = spheres.into_iter().map(|s| s.0).collect();
        Self(SpheresSoA::from_slice(&inner))
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn push(&mut self, sphere: PySphere) {
        self.0.push(sphere.0);
    }

    fn get(&self, index: usize) -> PySphere {
        PySphere(self.0.get(index))
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn any_collides_sphere(&self, sphere: &PySphere) -> bool {
        self.0.any_collides_sphere(&sphere.0)
    }

    fn __len__(&self) -> usize {
        self.0.len()
    }

    fn __repr__(&self) -> String {
        format!("SphereCollection(len={})", self.0.len())
    }
}

#[pymethods]
impl PyCollider {
    #[new]
    fn new() -> Self {
        Self(Collider::new())
    }

    fn add(&mut self, shape: PyShape) {
        match shape {
            PyShape::Sphere(s) => self.0.add(s.0),
            PyShape::Capsule(c) => self.0.add(c.0),
            PyShape::Cuboid(c) => self.0.add(c.0),
            PyShape::Cylinder(c) => self.0.add(c.0),
            PyShape::ConvexPolytope(p) => self.0.add(p.0),
            PyShape::ConvexPolygon(p) => self.0.add(p.0),
            PyShape::Line(l) => self.0.add(l.0),
            PyShape::Ray(r) => self.0.add(r.0),
            PyShape::LineSegment(s) => self.0.add(s.0),
            PyShape::Plane(p) => self.0.add(p.0),
            PyShape::Pointcloud(p) => self.0.add(p.0),
        }
    }

    fn extend(&mut self, shapes: Vec<PyShape>) {
        for shape in shapes {
            self.add(shape);
        }
    }

    fn include(&mut self, other: PyCollider) {
        self.0.include(other.0);
    }

    fn collides(&self, shape: &PyShape) -> PyResult<bool> {
        match shape {
            PyShape::Sphere(s) => Ok(self.0.collides(&s.0)),
            PyShape::Capsule(c) => Ok(self.0.collides(&c.0)),
            PyShape::Cuboid(c) => Ok(self.0.collides(&c.0)),
            PyShape::Cylinder(c) => Ok(self.0.collides(&c.0)),
            PyShape::ConvexPolytope(p) => Ok(self.0.collides(&p.0)),
            PyShape::ConvexPolygon(p) => Ok(self.0.collides(&p.0)),
            PyShape::Line(l) => Ok(self.0.collides(&l.0)),
            PyShape::Ray(r) => Ok(self.0.collides(&r.0)),
            PyShape::LineSegment(s) => Ok(self.0.collides(&s.0)),
            PyShape::Plane(p) => Ok(self.0.collides(&p.0)),
            PyShape::Pointcloud(_) => Err(pyo3::exceptions::PyValueError::new_err(
                "Pointcloud cannot query a Collider<Pointcloud>; use individual shape queries instead",
            )),
        }
    }

    fn collides_other(&self, other: &PyCollider) -> bool {
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

    fn get_planes(&self) -> Vec<PyPlane> {
        self.0.planes().iter().map(|p| PyPlane(*p)).collect()
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

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySphere>()?;
    m.add_class::<PyCapsule>()?;
    m.add_class::<PyCuboid>()?;
    m.add_class::<PyCylinder>()?;
    m.add_class::<PyConvexPolytope>()?;
    m.add_class::<PyConvexPolygon>()?;
    m.add_class::<PyLine>()?;
    m.add_class::<PyRay>()?;
    m.add_class::<PyLineSegment>()?;
    m.add_class::<PyPlane>()?;
    m.add_class::<PyPointcloud>()?;
    m.add_class::<PySphereCollection>()?;
    m.add_class::<PyShape>()?;
    m.add_class::<PyCollider>()?;
    Ok(())
}
