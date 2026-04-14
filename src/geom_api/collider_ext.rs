use crate::glam_wrappers::PyDAffine3;
use crate::wreck_wrappers::{
    PyCollider, PyConvexPolytope, PyCuboid, PyCylinder, PyPointcloud, PyShape, PySphere,
};
use pyo3::prelude::*;
use wreck::Transformable;

#[pymethods]
impl PyCollider {
    /// Create an empty ObstacleUnion / Collider.
    #[classmethod]
    fn empty(_cls: &Bound<'_, pyo3::types::PyType>) -> Self {
        Self(wreck::Collider::new())
    }

    /// Merge another Collider into a new Collider.
    fn merge(&self, other: &Self) -> Self {
        let mut out = self.0.clone();
        out.include(other.0.clone());
        Self(out)
    }

    /// Add a single obstacle and return a new Collider.
    fn new_with_any(&self, obstacle: &Bound<'_, PyAny>) -> PyResult<Self> {
        let mut out = self.0.clone();
        if let Ok(s) = obstacle.extract::<PySphere>() {
            out.add(s.0);
            return Ok(Self(out));
        }
        if let Ok(c) = obstacle.extract::<PyCuboid>() {
            out.add(c.0);
            return Ok(Self(out));
        }
        if let Ok(c) = obstacle.extract::<PyCylinder>() {
            out.add(c.0);
            return Ok(Self(out));
        }
        if let Ok(p) = obstacle.extract::<PyPointcloud>() {
            out.add(p.0);
            return Ok(Self(out));
        }
        if let Ok(shape) = obstacle.extract::<PyShape>() {
            match shape {
                PyShape::Sphere(s) => out.add(s.0),
                PyShape::Capsule(c) => out.add(c.0),
                PyShape::Cuboid(c) => out.add(c.0),
                PyShape::Cylinder(c) => out.add(c.0),
                PyShape::ConvexPolytope(p) => out.add(p.0),
                PyShape::ConvexPolygon(p) => out.add(p.0),
                PyShape::Line(l) => out.add(l.0),
                PyShape::Ray(r) => out.add(r.0),
                PyShape::LineSegment(s) => out.add(s.0),
                PyShape::Plane(p) => out.add(p.0),
                PyShape::Pointcloud(p) => out.add(p.0),
            }
            return Ok(Self(out));
        }
        Err(pyo3::exceptions::PyTypeError::new_err(
            "expected a Shape, Sphere, Cuboid, Cylinder, or Pointcloud",
        ))
    }

    /// Transform all obstacles by an Affine3.
    #[pyo3(name = "transform_all")]
    fn transform_all(&self, tf: &PyDAffine3) -> Self {
        let mut out = self.0.clone();
        out.transform_d(tf.0);
        Self(out)
    }

    /// Return all obstacles as a flat list of Shapes.
    fn stream(&self) -> Vec<PyShape> {
        let mut out = Vec::new();
        let spheres = self.0.spheres();
        for i in 0..spheres.len() {
            out.push(PyShape::Sphere(PySphere(spheres.get(i))));
        }
        for c in self.0.capsules() {
            out.push(PyShape::Capsule(crate::wreck_wrappers::PyCapsule(*c)));
        }
        for c in self.0.cuboids() {
            out.push(PyShape::Cuboid(PyCuboid(*c)));
        }
        for c in self.0.cylinders() {
            out.push(PyShape::Cylinder(PyCylinder(*c)));
        }
        for p in self.0.polytopes() {
            out.push(PyShape::ConvexPolytope(
                crate::wreck_wrappers::PyConvexPolytope(p.clone()),
            ));
        }
        for p in self.0.polygons() {
            out.push(PyShape::ConvexPolygon(
                crate::wreck_wrappers::PyConvexPolygon(p.clone()),
            ));
        }
        for p in self.0.pointclouds() {
            out.push(PyShape::Pointcloud(PyPointcloud(p.clone())));
        }
        out
    }

    fn point_clouds(&self) -> Vec<PyPointcloud> {
        self.0
            .pointclouds()
            .iter()
            .map(|p| PyPointcloud(p.clone()))
            .collect()
    }

    fn convex_hulls(&self) -> Vec<PyConvexPolytope> {
        self.0
            .polytopes()
            .iter()
            .map(|p| PyConvexPolytope(p.clone()))
            .collect()
    }

    fn __len__(&self) -> usize {
        self.0.spheres().len()
            + self.0.capsules().len()
            + self.0.cuboids().len()
            + self.0.cylinders().len()
            + self.0.polytopes().len()
            + self.0.polygons().len()
            + self.0.pointclouds().len()
    }

    /// Create a Collider from a single obstacle or list of obstacles.
    #[classmethod]
    fn from_any(_cls: &Bound<'_, pyo3::types::PyType>, obstacles: &Bound<'_, PyAny>) -> PyResult<Self> {
        let mut out = wreck::Collider::new();
        if obstacles.is_none() {
            return Ok(Self(out));
        }
        if let Ok(shapes) = obstacles.extract::<Vec<PyShape>>() {
            for shape in shapes {
                match shape {
                    PyShape::Sphere(s) => out.add(s.0),
                    PyShape::Capsule(c) => out.add(c.0),
                    PyShape::Cuboid(c) => out.add(c.0),
                    PyShape::Cylinder(c) => out.add(c.0),
                    PyShape::ConvexPolytope(p) => out.add(p.0),
                    PyShape::ConvexPolygon(p) => out.add(p.0),
                    PyShape::Line(l) => out.add(l.0),
                    PyShape::Ray(r) => out.add(r.0),
                    PyShape::LineSegment(s) => out.add(s.0),
                    PyShape::Plane(p) => out.add(p.0),
                    PyShape::Pointcloud(p) => out.add(p.0),
                }
            }
            return Ok(Self(out));
        }
        if let Ok(shape) = obstacles.extract::<PyShape>() {
            match shape {
                PyShape::Sphere(s) => out.add(s.0),
                PyShape::Capsule(c) => out.add(c.0),
                PyShape::Cuboid(c) => out.add(c.0),
                PyShape::Cylinder(c) => out.add(c.0),
                PyShape::ConvexPolytope(p) => out.add(p.0),
                PyShape::ConvexPolygon(p) => out.add(p.0),
                PyShape::Line(l) => out.add(l.0),
                PyShape::Ray(r) => out.add(r.0),
                PyShape::LineSegment(s) => out.add(s.0),
                PyShape::Plane(p) => out.add(p.0),
                PyShape::Pointcloud(p) => out.add(p.0),
            }
            return Ok(Self(out));
        }
        Err(pyo3::exceptions::PyTypeError::new_err(
            "expected a Shape, list of Shapes, or None",
        ))
    }
}
