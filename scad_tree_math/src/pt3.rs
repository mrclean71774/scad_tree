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

use crate::{dcos, dsin, Mt4, Pt4};

/// Wraps a `Vec<Pt3>`.
#[derive(Clone, PartialEq)]
pub struct Pt3s {
    inner: Vec<Pt3>,
}

impl std::ops::Deref for Pt3s {
    type Target = Vec<Pt3>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for Pt3s {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl std::fmt::Display for Pt3s {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.len() - 1 {
            write!(f, "{},", self[i])?
        }
        write!(f, "{}]", self[self.len() - 1])
    }
}

impl Pt3s {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn from_pt3s(pt2s: Vec<Pt3>) -> Self {
        Self { inner: pt2s }
    }

    pub fn translate(&mut self, point: Pt3) {
        for pt in self.iter_mut() {
            *pt = *pt + point
        }
    }

    pub fn apply_matrix(&mut self, matrix: &Mt4) {
        for pt in self.iter_mut() {
            *pt = (*matrix * pt.as_pt4(1.0)).as_pt3()
        }
    }

    pub fn rotate_x(&mut self, degrees: f64) -> &mut Self {
        for point in self.iter_mut() {
            point.rotate_x(degrees);
        }
        self
    }

    pub fn rotate_y(&mut self, degrees: f64) -> &mut Self {
        for point in self.iter_mut() {
            point.rotate_y(degrees);
        }
        self
    }

    pub fn rotate_z(&mut self, degrees: f64) -> &mut Self {
        for point in self.iter_mut() {
            point.rotate_z(degrees);
        }
        self
    }
}

/// A 3D point.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pt3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl std::fmt::Display for Pt3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl std::ops::Index<usize> for Pt3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index {} is out of bounds.", index),
        }
    }
}

impl std::ops::IndexMut<usize> for Pt3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index {} is out of bounds.", index),
        }
    }
}

impl std::ops::Add for Pt3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::AddAssign for Pt3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Pt3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::SubAssign for Pt3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<f64> for Pt3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::MulAssign<f64> for Pt3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<f64> for Pt3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl std::ops::DivAssign<f64> for Pt3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl std::ops::Neg for Pt3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Pt3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Pt3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn len2(self) -> f64 {
        self.dot(self)
    }

    pub fn len(self) -> f64 {
        self.len2().sqrt()
    }

    pub fn normalize(&mut self) {
        *self /= self.len();
    }

    pub fn normalized(self) -> Self {
        let l = self.len();
        Self::new(self.x / l, self.y / l, self.z / l)
    }

    pub fn rotated_x(self, degrees: f64) -> Self {
        let s = dsin(degrees);
        let c = dcos(degrees);
        Self::new(self.x, self.y * c - self.z * s, self.y * s + self.z * c)
    }

    pub fn rotate_x(&mut self, degrees: f64) {
        *self = self.rotated_x(degrees);
    }

    pub fn rotated_y(self, degrees: f64) -> Self {
        let s = dsin(degrees);
        let c = dcos(degrees);
        Self::new(self.x * c - self.z * s, self.y, self.x * s + self.z * c)
    }

    pub fn rotate_y(&mut self, degrees: f64) {
        *self = self.rotated_y(degrees);
    }

    pub fn rotated_z(self, degrees: f64) -> Self {
        let s = dsin(degrees);
        let c = dcos(degrees);
        Self::new(self.x * c - self.y * s, self.x * s + self.y * c, self.z)
    }

    pub fn rotate_z(&mut self, degrees: f64) {
        *self = self.rotated_z(degrees);
    }

    pub fn lerp(self, b: Self, t: f64) -> Self {
        self + (b - self) * t
    }

    pub fn as_pt4(self, w: f64) -> Pt4 {
        Pt4::new(self.x, self.y, self.z, w)
    }
}
