use crate::wreck_wrappers::PyConvexPolytope;
use glam::Vec3;
use pyo3::prelude::*;
use wreck::ConvexPolytope;

/// Convert a convex trimesh to a ConvexPolytope.
///
/// Extracts vertices and face normals from the trimesh,
/// computes halfspace planes, and builds the polytope.
#[pyfunction]
pub fn trimesh_to_convex_polytope(mesh: &Bound<'_, PyAny>) -> PyResult<PyConvexPolytope> {
    let face_normals = mesh.getattr("face_normals")?;
    let vertices_attr = mesh.getattr("vertices")?;
    let faces_attr = mesh.getattr("faces")?;

    let normals: Vec<[f64; 3]> = face_normals.extract()?;
    let vertices: Vec<[f64; 3]> = vertices_attr.extract()?;
    let faces: Vec<[usize; 3]> = faces_attr.extract()?;

    let mut planes: Vec<(Vec3, f32)> = Vec::with_capacity(normals.len());
    for (i, n) in normals.iter().enumerate() {
        let normal = Vec3::new(n[0] as f32, n[1] as f32, n[2] as f32);
        let first_vertex_idx = faces[i][0];
        let v = &vertices[first_vertex_idx];
        let v_f32 = Vec3::new(v[0] as f32, v[1] as f32, v[2] as f32);
        let d = normal.dot(v_f32);
        planes.push((normal, d));
    }

    let verts: Vec<Vec3> = vertices
        .iter()
        .map(|v| Vec3::new(v[0] as f32, v[1] as f32, v[2] as f32))
        .collect();

    Ok(PyConvexPolytope(ConvexPolytope::new(planes, verts)))
}
