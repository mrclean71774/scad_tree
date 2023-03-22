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

mod scad;

pub mod prelude {
    pub use {
        crate::{
            circle, color, cube, cylinder, difference, hull, import, intersection, linear_extrude,
            minkowski, mirror, polygon, polyhedron, projection, resize, rotate, rotate_extrude,
            scad_file, scale, sphere, square, surface, text, translate, union, Faces, Indices,
            Paths, Pt2, Pt2s, Pt3, Pt3s, Pt4, Scad, ScadColor, ScadOp, TextDirection, TextHalign,
            TextParams, TextValign,
        },
        std::io::Write,
    };
}

pub use {
    scad::{Scad, ScadColor, ScadOp, TextDirection, TextHalign, TextParams, TextValign},
    scadder_math::{
        approx_eq, dacos, dasin, datan, dcos, dsin, dtan, MersenneTwister, Mt4, Pt2, Pt2s, Pt3,
        Pt3s, Pt4, Pt4s,
    },
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

    pub fn from_paths(paths: Vec<Indices>) -> Self {
        Self { inner: paths }
    }

    pub fn from_faces(faces: Vec<Indices>) -> Self {
        Self { inner: faces }
    }
}

pub type Faces = Paths;
