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

use crate::{dcos, dsin, Pt3};

#[derive(Clone, PartialEq)]
pub struct Pt2s {
    inner: Vec<Pt2>,
}

impl std::ops::Deref for Pt2s {
    type Target = Vec<Pt2>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for Pt2s {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl std::fmt::Display for Pt2s {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.len() - 1 {
            write!(f, "{},", self[i])?
        }
        write!(f, "{}]", self[self.len() - 1])
    }
}

impl Pt2s {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn from_pt2s(pt2s: Vec<Pt2>) -> Self {
        Self { inner: pt2s }
    }

    pub fn translate(&mut self, point: Pt2) {
        for pt in self.iter_mut() {
            *pt = *pt + point
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pt2 {
    pub x: f64,
    pub y: f64,
}

impl std::fmt::Display for Pt2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl std::ops::Index<usize> for Pt2 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index {} is out of bounds.", index),
        }
    }
}

impl std::ops::IndexMut<usize> for Pt2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index {} is out of bounds.", index),
        }
    }
}

impl std::ops::Add for Pt2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Pt2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Pt2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::SubAssign for Pt2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<f64> for Pt2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::MulAssign<f64> for Pt2 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<f64> for Pt2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl std::ops::DivAssign<f64> for Pt2 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl std::ops::Neg for Pt2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Pt2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn dot(self, rhs: Pt2) -> f64 {
        self.x * rhs.x + self.y * rhs.y
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
        Self::new(self.x / l, self.y / l)
    }

    pub fn rotate(&mut self, degrees: f64) {
        *self = self.rotated(degrees);
    }

    pub fn rotated(self, degrees: f64) -> Self {
        let c = dcos(degrees);
        let s = dsin(degrees);
        Self::new(self.x * c - self.y * s, self.x * s + self.y * c)
    }

    pub fn lerp(self, b: Self, t: f64) -> Self {
        self + (b - self) * t
    }

    pub fn to_xz(self) -> Pt3 {
        Pt3::new(self.x, 0.0, self.y)
    }

    pub fn as_pt3(self, z: f64) -> Pt3 {
        Pt3::new(self.x, self.y, z)
    }
}
