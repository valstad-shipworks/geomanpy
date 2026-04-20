use crate::glam_wrappers::{PyDAffine3, PyDVec3};
use crate::impl_dataclass_fields;
use crate::pickle::{make_getnewargs_ex, pickle_decode};
use crate::wreck_wrappers::PyCuboid;
use glam::DVec3;
use pyo3::Bound;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

#[pyclass(frozen, from_py_object, name = "AlignedBox3d")]
#[derive(Debug, Clone, Copy)]
pub struct PyAlignedBox3d {
    pub(crate) min: DVec3,
    pub(crate) max: DVec3,
}

#[pymethods]
impl PyAlignedBox3d {
    #[new]
    #[pyo3(signature = (min=None, max=None, *, __pickle_state__=None))]
    fn new(
        min: Option<PyDVec3>,
        max: Option<PyDVec3>,
        __pickle_state__: Option<Vec<u8>>,
    ) -> PyResult<Self> {
        if let Some(state) = __pickle_state__ {
            let (min, max): (DVec3, DVec3) = pickle_decode(&state)?;
            return Ok(Self { min, max });
        }
        match (min, max) {
            (Some(min), Some(max)) => Ok(Self { min: min.0, max: max.0 }),
            _ => Err(pyo3::exceptions::PyValueError::new_err(
                "AlignedBox3d requires min and max arguments",
            )),
        }
    }

    /// Create from two corners, sorting each axis so min <= max.
    #[staticmethod]
    fn from_corners(lo: PyDVec3, hi: PyDVec3) -> Self {
        Self {
            min: DVec3::new(
                lo.0.x.min(hi.0.x),
                lo.0.y.min(hi.0.y),
                lo.0.z.min(hi.0.z),
            ),
            max: DVec3::new(
                lo.0.x.max(hi.0.x),
                lo.0.y.max(hi.0.y),
                lo.0.z.max(hi.0.z),
            ),
        }
    }

    /// Minimum corner.
    #[getter]
    fn min(&self) -> PyDVec3 {
        PyDVec3(self.min)
    }

    /// Maximum corner.
    #[getter]
    fn max(&self) -> PyDVec3 {
        PyDVec3(self.max)
    }

    /// Center point of the box.
    #[getter]
    fn center(&self) -> PyDVec3 {
        PyDVec3((self.min + self.max) / 2.0)
    }

    /// Size along each axis.
    #[getter]
    fn size(&self) -> (f64, f64, f64) {
        let s = self.max - self.min;
        (s.x, s.y, s.z)
    }

    /// Apply an affine transform (only translation is applied to keep axis-alignment).
    fn transform(&self, tf: PyDAffine3) -> Self {
        let t = tf.0.translation;
        Self {
            min: self.min + t,
            max: self.max + t,
        }
    }

    /// Check if a point is inside the box.
    fn contains(&self, point: PyDVec3) -> bool {
        let p = point.0;
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }

    /// Scale the box by a uniform factor or per-axis (f64, f64, f64) tuple.
    fn scale(&self, factor: &Bound<'_, PyAny>) -> PyResult<Self> {
        let center = (self.min + self.max) / 2.0;
        let half = (self.max - self.min) / 2.0;
        let scaled_half = if let Ok(f) = factor.extract::<f64>() {
            half * f.abs()
        } else if let Ok((fx, fy, fz)) = factor.extract::<(f64, f64, f64)>() {
            DVec3::new(half.x * fx.abs(), half.y * fy.abs(), half.z * fz.abs())
        } else {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "factor must be a float or (f64, f64, f64) tuple",
            ));
        };
        Ok(Self {
            min: center - scaled_half,
            max: center + scaled_half,
        })
    }

    /// Convert to an axis-aligned Cuboid.
    fn as_unaligned(&self) -> PyCuboid {
        let center = (self.min + self.max) / 2.0;
        let he = (self.max - self.min) / 2.0;
        PyCuboid(wreck::Cuboid::new(
            glam::Vec3::new(center.x as f32, center.y as f32, center.z as f32),
            [glam::Vec3::X, glam::Vec3::Y, glam::Vec3::Z],
            [he.x.abs() as f32, he.y.abs() as f32, he.z.abs() as f32],
        ))
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.min.abs_diff_eq(other.min, 1e-12) && self.max.abs_diff_eq(other.max, 1e-12)
    }

    fn __repr__(&self) -> String {
        format!(
            "AlignedBox3d(min=({:.6}, {:.6}, {:.6}), max=({:.6}, {:.6}, {:.6}))",
            self.min.x, self.min.y, self.min.z, self.max.x, self.max.y, self.max.z
        )
    }

    fn __getnewargs_ex__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyTuple>, Bound<'py, PyDict>)> {
        make_getnewargs_ex(py, &(self.min, self.max))
    }
}

impl_dataclass_fields!(PyAlignedBox3d, ["min", "max"]);
