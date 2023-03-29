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

pub mod dim2;
pub mod dim3;
pub mod metric_thread;
mod scad;
mod triangulate;
mod viewer;

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
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn from_indices(indices: Vec<u64>) -> Self {
        Self { inner: indices }
    }
}

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
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn from_paths(paths: Vec<Indices>) -> Self {
        Self { inner: paths }
    }

    pub fn from_faces(faces: Vec<Indices>) -> Self {
        Self { inner: faces }
    }
}

pub type Faces = Paths;

#[macro_export]
macro_rules! fat_thread {
    ($code:block) => {
        std::thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .spawn(|| $code)
            .unwrap()
    };
}
