mod affine3_ext;
pub mod aligned_box3d;
mod collider_ext;
mod cuboid_ext;
mod mat3_ext;
mod pointcloud_ext;
mod quat_ext;
pub mod raw_geom_util;
mod sphere_ext;
pub mod trimesh_convert;
mod vec3_ext;

use pyo3::prelude::*;

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<aligned_box3d::PyAlignedBox3d>()?;
    m.add_class::<raw_geom_util::PyRawGeomUtil>()?;
    m.add_function(wrap_pyfunction!(trimesh_convert::trimesh_to_convex_polytope, m)?)?;
    Ok(())
}
