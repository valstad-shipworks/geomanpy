use crate::geom_api::trimesh_convert::trimesh_to_convex_polytope;
use crate::glam_wrappers::PyDAffine3;
use crate::wreck_wrappers::{
    PyCollider, PyConvexPolytope, PyCuboid, PyCylinder, PyPointcloud, PyShape, PySphere,
};
use pyo3::prelude::*;
use wreck::Transformable;

/// Push a PyShape into a Collider. Centralises the match across variants.
fn push_shape(out: &mut wreck::Collider<wreck::Pointcloud>, shape: PyShape) {
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

/// Quack-typed check: does this look like a trimesh.Trimesh?
fn looks_like_trimesh(obj: &Bound<'_, PyAny>) -> bool {
    obj.hasattr("face_normals").unwrap_or(false)
        && obj.hasattr("vertices").unwrap_or(false)
        && obj.hasattr("faces").unwrap_or(false)
}

/// Try to extract a single obstacle (shape or trimesh) and push it into
/// the collider. Returns Ok(true) on success, Ok(false) if the object was
/// not a recognised obstacle, Err on conversion failure.
fn try_push_obstacle(
    out: &mut wreck::Collider<wreck::Pointcloud>,
    obj: &Bound<'_, PyAny>,
) -> PyResult<bool> {
    if let Ok(s) = obj.extract::<PySphere>() {
        out.add(s.0);
        return Ok(true);
    }
    if let Ok(c) = obj.extract::<PyCuboid>() {
        out.add(c.0);
        return Ok(true);
    }
    if let Ok(c) = obj.extract::<PyCylinder>() {
        out.add(c.0);
        return Ok(true);
    }
    if let Ok(p) = obj.extract::<PyPointcloud>() {
        out.add(p.0);
        return Ok(true);
    }
    if let Ok(shape) = obj.extract::<PyShape>() {
        push_shape(out, shape);
        return Ok(true);
    }
    if looks_like_trimesh(obj) {
        let poly = trimesh_to_convex_polytope(obj)?;
        out.add(poly.0);
        return Ok(true);
    }
    Ok(false)
}

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

    /// Add one or more obstacles and return a new Collider.
    ///
    /// Accepts a single Shape/Sphere/Cuboid/Cylinder/Pointcloud, a
    /// ``trimesh.Trimesh`` (auto-converted to a ConvexPolytope), or a
    /// list/tuple of any of the above. ``None`` is a no-op.
    fn new_with_any(&self, obstacle: &Bound<'_, PyAny>) -> PyResult<Self> {
        let mut out = self.0.clone();
        if obstacle.is_none() {
            return Ok(Self(out));
        }
        if try_push_obstacle(&mut out, obstacle)? {
            return Ok(Self(out));
        }
        // Not a single obstacle — try iterating (list, tuple, any sequence).
        if let Ok(iter) = obstacle.try_iter() {
            for item in iter {
                let item = item?;
                if !try_push_obstacle(&mut out, &item)? {
                    return Err(pyo3::exceptions::PyTypeError::new_err(
                        "sequence contained an item that is not a Shape, primitive, or trimesh.Trimesh",
                    ));
                }
            }
            return Ok(Self(out));
        }
        Err(pyo3::exceptions::PyTypeError::new_err(
            "expected a Shape, Sphere, Cuboid, Cylinder, Pointcloud, trimesh.Trimesh, or a sequence of these",
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

    /// Create a Collider from a single obstacle, list of obstacles, or None.
    ///
    /// Accepts the same inputs as ``new_with_any``, plus ``None`` → empty.
    #[classmethod]
    fn from_any(_cls: &Bound<'_, pyo3::types::PyType>, obstacles: &Bound<'_, PyAny>) -> PyResult<Self> {
        let empty = Self(wreck::Collider::new());
        empty.new_with_any(obstacles)
    }
}
