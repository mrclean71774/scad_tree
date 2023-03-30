// MIT License
//
// Copyright (c) 2023 Michael H. Phillips
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//! The scad_tree library is a library for generating OpenSCAD code from Rust.
//!
//! Notes on usage:
//! * 2D profiles for non-OpenSCAD functions/macros are specified by points in
//!     clockwise order.
//! * Polyhedron faces are specified in clockwise order.

/// Module for the creation of 2D profiles and curves.
pub mod dim2;
/// Module for the creation of 3D curves and polyhedrons.
pub mod dim3;
/// Module for metric threaded rod, nuts and bolts.
pub mod metric_thread;
mod scad;
mod triangulate;
mod viewer;

/// Module for quickly importing library types and macros.
pub mod prelude {
    pub use {
        crate::{
            circle, color, cube, cylinder, difference, dim2, dim3, fat_thread, hull, import,
            intersection, linear_extrude, metric_thread, minkowski, mirror, polygon, polyhedron,
            projection, resize, rotate, rotate_extrude, scad_file, scale, sphere, square, surface,
            text, translate, union, BezierStar, CubicBezier2D, CubicBezier3D, CubicBezierChain2D,
            CubicBezierChain3D, Faces, Indices, Paths, Polyhedron, Pt2, Pt2s, Pt3, Pt3s, Pt4,
            QuadraticBezier2D, QuadraticBezier3D, Scad, ScadColor, ScadOp, TextDirection,
            TextHalign, TextParams, TextValign, Viewer,
        },
        std::io::Write,
    };
}

pub use {
    dim2::{BezierStar, CubicBezier2D, CubicBezierChain2D, QuadraticBezier2D},
    dim3::{CubicBezier3D, CubicBezierChain3D, Polyhedron, QuadraticBezier3D},
    scad::{Scad, ScadColor, ScadOp, TextDirection, TextHalign, TextParams, TextValign},
    scad_tree_math::{
        approx_eq, dacos, dasin, datan, dcos, dsin, dtan, MersenneTwister, Mt4, Pt2, Pt2s, Pt3,
        Pt3s, Pt4, Pt4s,
    },
    triangulate::{triangulate2d, triangulate2d_rev, triangulate3d, triangulate3d_rev},
    viewer::Viewer,
};

/// Wraps a `Vec<u64>`.
#[derive(Clone, PartialEq)]
pub struct Indices {
    inner: Vec<u64>,
}

impl std::ops::Deref for Indices {
    type Target = Vec<u64>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for Indices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl std::fmt::Display for Indices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.len() - 1 {
            write!(f, "{}, ", self[i])?;
        }
        write!(f, "{}]", self[self.len() - 1])
    }
}

impl Indices {
    /// Create an empty Indices.
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Create Indices from a `Vec<u64>`.
    pub fn from_indices(indices: Vec<u64>) -> Self {
        Self { inner: indices }
    }
}

/// Paths wrap a `Vec<Indices>`.
///
/// Used for polygon macro. Faces is an alias used for polyhedron macro.
#[derive(Clone, PartialEq)]
pub struct Paths {
    inner: Vec<Indices>,
}

impl std::ops::Deref for Paths {
    type Target = Vec<Indices>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for Paths {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl std::fmt::Display for Paths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.len() - 1 {
            write!(f, "{}, ", self[i])?;
        }
        write!(f, "{}]", self[self.len() - 1])
    }
}

impl Paths {
    /// Create an empty Paths.
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Create a Paths with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    /// Create a Paths from a `Vec<Indices>`.
    ///
    /// Duplicate of from_faces for readability.
    pub fn from_paths(paths: Vec<Indices>) -> Self {
        Self { inner: paths }
    }

    /// Create a Paths from a `Vec<Indices>`.
    ///
    /// Duplicate of from_paths for readability.
    pub fn from_faces(faces: Vec<Indices>) -> Self {
        Self { inner: faces }
    }
}

/// Alias for Paths.
pub type Faces = Paths;

/// Runs a code block in a separate thread.
///
/// The thread has a stack size of 32 megabytes to avoid stack overflow
/// do to recursion. Automatically used by the scad_file macro.
#[macro_export]
macro_rules! fat_thread {
    ($code:block) => {
        std::thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .spawn(|| $code)
            .unwrap()
    };
}
