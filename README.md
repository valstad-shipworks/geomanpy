# geomanpy

Python bindings for [`glam`](https://crates.io/crates/glam) and [`wreck`](https://crates.io/crates/wreck) — fast geometric/linear-algebra primitives and collision detection, backed by Rust.

`geomanpy` exposes glam's vectors, quaternions, matrices and affine transforms, plus wreck's collision shapes and broad/narrow-phase queries, as native Python classes. Every type integrates with NumPy and behaves like a dataclass for serialization frameworks.

## Features

- **Linear algebra** — `Vec2`, `Vec3`, `Vec4`, `Quat`, `Mat3`, `Mat4`, `Affine3` with the full glam method surface (normalization, projection, interpolation, Euler conversions, projection/view matrices, …).
- **Collision shapes** — `Sphere`, `Capsule`, `Cuboid`, `Cylinder`, `ConvexPolytope`, `ConvexPolygon`, `Line`, `Ray`, `LineSegment`, `Plane`, `Pointcloud`, plus `SphereCollection` and `Collider` aggregates.
- **Transform & query protocols** — shared `Scalable`, `Transformable`, `Bounded`, `Stretchable`, and `Collides` interfaces across shapes (`scaled`, `translated`, `rotated_quat`, `transformed`, `broadphase`, `obb`, `aabb`, `collides`).
- **NumPy interop** — `from_numpy`/`to_numpy` and the `__array__` protocol on every primitive.
- **Serialization** — instances expose `__dataclass_fields__` so `orjson`, `msgspec`, and `pydantic` treat them as dataclasses; pickling is supported via `__getnewargs_ex__`.
- **Fully typed** — ships a `py.typed` marker and complete `.pyi` stubs.

## Installation

```bash
pip install geomanpy
```

## Usage

```python
import numpy as np
from geomanpy import Vec3, Quat, Sphere, Cuboid, Collider

# Linear algebra
a = Vec3(1.0, 2.0, 3.0)
b = Vec3.from_numpy(np.array([4.0, 5.0, 6.0]))
print(a.dot(b), a.cross(b).to_numpy())

# Rotations
q = Quat.from_rotation_z(np.pi / 2)
print((q * Vec3.X).to_numpy())

# Collision detection
ball = Sphere(Vec3.ZERO, radius=1.0)
box = Cuboid.from_aabb(Vec3(0.5, 0.5, 0.5), Vec3(2.0, 2.0, 2.0))
print(ball.collides(box))

# Aggregate many shapes behind a single broad/narrow-phase query
scene = Collider()
scene.add(box)
print(scene.collides(ball))
```

Convenience aliases are provided for common names: `Quaternion` (`Quat`), `Rotation3d` (`Mat3`), `Translation3d` (`Vec3`), `Transform3d` (`Affine3`), `Box3d` (`Cuboid`), `Sphere3d` (`Sphere`), `Cylinder3d` (`Cylinder`), `ObstacleUnion` (`Collider`), and `PointCloud` (`Pointcloud`).

## Backends

`geomanpy` can target two Python runtimes, selected at build time by Cargo feature:

- **`pyo3-backend`** (default) — builds a CPython extension module via [PyO3](https://pyo3.rs).
- **`rustpython-backend`** — registers a `_geomanpy` module inside an embedded [RustPython](https://rustpython.github.io) VM, reusing the same wrapper types.

Exactly one backend must be active. The optional `safe-locks` feature switches mutable wrappers to RustPython's `PyRwLock`.

## Building from source

The CPython wheel is built with [maturin](https://www.maturin.rs):

```bash
maturin develop   # build and install into the active virtualenv
maturin build --release
```

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
