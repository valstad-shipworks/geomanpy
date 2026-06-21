//! Squiggle wrapper types — the curve primitives `QuadraticBezier`,
//! `CubicBezier`, `Polyline`, and `Spline`, plus the `Interval` and `Nearest`
//! value types they query against. Shared across the pyo3 and rustpython
//! backends via `cfg_attr`, mirroring `crate::wreck_wrappers`.
//!
//! `Line`, `Ray`, and `LineSegment` are squiggle types too, but they reach
//! Python through `crate::wreck_wrappers` (wreck re-exports them and adds
//! collision), so they are not re-wrapped here.

use glam::Vec3;
use squiggle::{ArcLength, Bounds, ControlPoints, Curve, NearestPoint};

use crate::glam_wrappers::PyDVec3;
use crate::wreck_wrappers::PyCuboid;

pub(crate) mod bezier;
pub(crate) mod interval;
pub(crate) mod nearest;
pub(crate) mod polyline;
pub(crate) mod spline;

pub use bezier::{PyCubicBezier, PyQuadraticBezier};
pub use interval::PyInterval;
pub use nearest::PyNearest;
pub use polyline::PyPolyline;
pub use spline::PySpline;

/// `Vec3` (f32) → the double-precision Python wrapper. Identical to each
/// backend's `v3d`, but usable from backend-agnostic helper code.
#[inline]
pub(crate) fn vp(v: Vec3) -> PyDVec3 {
    PyDVec3(glam::DVec3::new(v.x as f64, v.y as f64, v.z as f64))
}

pub(crate) fn domain<C: Curve>(c: &C) -> PyInterval {
    PyInterval(c.domain())
}
pub(crate) fn point<C: Curve>(c: &C, t: f32) -> PyDVec3 {
    vp(c.point(t))
}
pub(crate) fn velocity<C: Curve>(c: &C, t: f32) -> PyDVec3 {
    vp(c.velocity(t))
}
pub(crate) fn acceleration<C: Curve>(c: &C, t: f32) -> PyDVec3 {
    vp(c.acceleration(t))
}
pub(crate) fn tangent<C: Curve>(c: &C, t: f32) -> PyDVec3 {
    vp(c.tangent(t))
}
pub(crate) fn normal<C: Curve>(c: &C, t: f32) -> PyDVec3 {
    vp(c.normal(t))
}
pub(crate) fn binormal<C: Curve>(c: &C, t: f32) -> PyDVec3 {
    vp(c.binormal(t))
}
pub(crate) fn curvature<C: Curve>(c: &C, t: f32) -> f64 {
    c.curvature(t) as f64
}
pub(crate) fn point_clamped<C: Curve>(c: &C, t: f32) -> PyDVec3 {
    vp(c.point_clamped(t))
}
pub(crate) fn endpoints<C: Curve>(c: &C) -> (PyDVec3, PyDVec3) {
    let (a, b) = c.endpoints();
    (vp(a), vp(b))
}
pub(crate) fn length<C: Curve>(c: &C) -> f64 {
    c.length() as f64
}
pub(crate) fn aabb<C: Bounds>(c: &C) -> PyCuboid {
    let bb = c.aabb();
    PyCuboid(wreck::Cuboid::from_aabb(bb.min, bb.max))
}
pub(crate) fn nearest<C: NearestPoint>(c: &C, query: Vec3) -> PyNearest {
    PyNearest(c.nearest(query))
}
pub(crate) fn distance<C: NearestPoint>(c: &C, query: Vec3) -> f64 {
    c.distance(query) as f64
}
pub(crate) fn distance_sq<C: NearestPoint>(c: &C, query: Vec3) -> f64 {
    c.distance_sq(query) as f64
}
#[cfg_attr(feature = "rustpython-backend", allow(dead_code))]
pub(crate) fn control_points<C: ControlPoints>(c: &C) -> Vec<PyDVec3> {
    c.control_points().iter().map(|p| vp(*p)).collect()
}
pub(crate) fn arc_length_to<C: ArcLength>(c: &C, t: f32) -> f64 {
    c.arc_length_to(t) as f64
}
pub(crate) fn t_at_distance<C: ArcLength>(c: &C, s: f32) -> f64 {
    c.t_at_distance(s) as f64
}
pub(crate) fn point_at_distance<C: ArcLength>(c: &C, s: f32) -> PyDVec3 {
    vp(c.point_at_distance(s))
}

#[cfg(feature = "pyo3-backend")]
mod pyo3_glue {
    macro_rules! impl_curve_py {
        ($ty:ty) => {
            #[pyo3::pymethods]
            impl $ty {
                fn domain(&self) -> $crate::squiggle_wrappers::PyInterval {
                    $crate::squiggle_wrappers::domain(&self.0)
                }
                fn point(&self, t: f64) -> $crate::glam_wrappers::PyDVec3 {
                    $crate::squiggle_wrappers::point(&self.0, t as f32)
                }
                fn velocity(&self, t: f64) -> $crate::glam_wrappers::PyDVec3 {
                    $crate::squiggle_wrappers::velocity(&self.0, t as f32)
                }
                fn acceleration(&self, t: f64) -> $crate::glam_wrappers::PyDVec3 {
                    $crate::squiggle_wrappers::acceleration(&self.0, t as f32)
                }
                fn tangent(&self, t: f64) -> $crate::glam_wrappers::PyDVec3 {
                    $crate::squiggle_wrappers::tangent(&self.0, t as f32)
                }
                fn normal(&self, t: f64) -> $crate::glam_wrappers::PyDVec3 {
                    $crate::squiggle_wrappers::normal(&self.0, t as f32)
                }
                fn binormal(&self, t: f64) -> $crate::glam_wrappers::PyDVec3 {
                    $crate::squiggle_wrappers::binormal(&self.0, t as f32)
                }
                fn curvature(&self, t: f64) -> f64 {
                    $crate::squiggle_wrappers::curvature(&self.0, t as f32)
                }
                fn point_clamped(&self, t: f64) -> $crate::glam_wrappers::PyDVec3 {
                    $crate::squiggle_wrappers::point_clamped(&self.0, t as f32)
                }
                fn endpoints(
                    &self,
                ) -> (
                    $crate::glam_wrappers::PyDVec3,
                    $crate::glam_wrappers::PyDVec3,
                ) {
                    $crate::squiggle_wrappers::endpoints(&self.0)
                }
                fn length(&self) -> f64 {
                    $crate::squiggle_wrappers::length(&self.0)
                }
                fn aabb(&self) -> $crate::wreck_wrappers::PyCuboid {
                    $crate::squiggle_wrappers::aabb(&self.0)
                }
                fn nearest(
                    &self,
                    query: $crate::glam_wrappers::PyDVec3,
                ) -> $crate::squiggle_wrappers::PyNearest {
                    $crate::squiggle_wrappers::nearest(
                        &self.0,
                        $crate::wreck_wrappers::pyo3_glue::dv3(query),
                    )
                }
                fn distance(&self, query: $crate::glam_wrappers::PyDVec3) -> f64 {
                    $crate::squiggle_wrappers::distance(
                        &self.0,
                        $crate::wreck_wrappers::pyo3_glue::dv3(query),
                    )
                }
                fn distance_sq(&self, query: $crate::glam_wrappers::PyDVec3) -> f64 {
                    $crate::squiggle_wrappers::distance_sq(
                        &self.0,
                        $crate::wreck_wrappers::pyo3_glue::dv3(query),
                    )
                }
                fn control_points(&self) -> Vec<$crate::glam_wrappers::PyDVec3> {
                    $crate::squiggle_wrappers::control_points(&self.0)
                }
            }
        };
    }

    macro_rules! impl_transform_py {
        ($ty:ty) => {
            #[pyo3::pymethods]
            impl $ty {
                fn scaled(&self, factor: f64) -> Self {
                    Self(squiggle::Transform::scaled(&self.0, factor as f32))
                }
                fn translated(&self, offset: $crate::glam_wrappers::PyDVec3) -> Self {
                    Self(squiggle::Transform::translated(
                        &self.0,
                        $crate::wreck_wrappers::pyo3_glue::dv3(offset),
                    ))
                }
                fn rotated_mat(&self, mat: $crate::glam_wrappers::PyDMat3) -> Self {
                    Self(squiggle::Transform::rotated_mat(&self.0, mat.0.as_mat3()))
                }
                fn rotated_quat(&self, quat: $crate::glam_wrappers::PyDQuat) -> Self {
                    Self(squiggle::Transform::rotated(&self.0, quat.0.as_quat()))
                }
                fn transformed(&self, mat: $crate::glam_wrappers::PyDAffine3) -> Self {
                    Self(squiggle::Transform::transformed(
                        &self.0,
                        glam::Affine3A::from(mat.0.as_affine3()),
                    ))
                }
            }
        };
    }

    macro_rules! impl_trim_py {
        ($ty:ty) => {
            #[pyo3::pymethods]
            impl $ty {
                fn subcurve(&self, t0: f64, t1: f64) -> Self {
                    Self(squiggle::Trim::subcurve(&self.0, t0 as f32, t1 as f32))
                }
                fn reversed(&self) -> Self {
                    Self(squiggle::Trim::reversed(&self.0))
                }
                fn truncate_start(&self, t0: f64) -> Self {
                    Self(squiggle::Trim::truncate_start(&self.0, t0 as f32))
                }
                fn truncate_end(&self, t1: f64) -> Self {
                    Self(squiggle::Trim::truncate_end(&self.0, t1 as f32))
                }
                fn split_at(&self, t: f64) -> (Self, Self) {
                    let (a, b) = squiggle::Trim::split_at(&self.0, t as f32);
                    (Self(a), Self(b))
                }
            }
        };
    }

    macro_rules! impl_arclength_py {
        ($ty:ty) => {
            #[pyo3::pymethods]
            impl $ty {
                fn arc_length_to(&self, t: f64) -> f64 {
                    $crate::squiggle_wrappers::arc_length_to(&self.0, t as f32)
                }
                fn t_at_distance(&self, s: f64) -> f64 {
                    $crate::squiggle_wrappers::t_at_distance(&self.0, s as f32)
                }
                fn point_at_distance(&self, s: f64) -> $crate::glam_wrappers::PyDVec3 {
                    $crate::squiggle_wrappers::point_at_distance(&self.0, s as f32)
                }
            }
        };
    }

    macro_rules! impl_approx_py {
        ($ty:ty) => {
            #[pyo3::pymethods]
            impl $ty {
                #[inline]
                fn abs_diff_eq(&self, rhs: &Self, max_abs_diff: f64) -> bool {
                    approx::AbsDiffEq::abs_diff_eq(&self.0, &rhs.0, max_abs_diff as f32)
                }
            }
        };
    }

    pub(crate) use impl_approx_py;
    pub(crate) use impl_arclength_py;
    pub(crate) use impl_curve_py;
    pub(crate) use impl_transform_py;
    pub(crate) use impl_trim_py;
}

#[cfg(feature = "pyo3-backend")]
pub(crate) use pyo3_glue::{
    impl_approx_py, impl_arclength_py, impl_curve_py, impl_transform_py, impl_trim_py,
};

#[cfg(feature = "pyo3-backend")]
pub fn register(m: &pyo3::Bound<'_, pyo3::types::PyModule>) -> pyo3::PyResult<()> {
    use pyo3::prelude::*;
    m.add_class::<PyInterval>()?;
    m.add_class::<PyNearest>()?;
    m.add_class::<PyQuadraticBezier>()?;
    m.add_class::<PyCubicBezier>()?;
    m.add_class::<PyPolyline>()?;
    m.add_class::<PySpline>()?;
    Ok(())
}
