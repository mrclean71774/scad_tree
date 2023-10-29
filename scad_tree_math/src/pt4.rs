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

use crate::Pt3;

/// Wraps a `Vec<Pt4>`.
#[derive(Clone, PartialEq)]
pub struct Pt4s {
    inner: Vec<Pt4>,
}

impl std::ops::Deref for Pt4s {
    type Target = Vec<Pt4>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for Pt4s {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl std::fmt::Display for Pt4s {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.len() - 1 {
            write!(f, "{},", self[i])?
        }
        write!(f, "{}]", self[self.len() - 1])
    }
}

impl Pt4s {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn from_pt4s(pt4s: Vec<Pt4>) -> Self {
        Self { inner: pt4s }
    }
}

/// A 4D point.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pt4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl std::fmt::Display for Pt4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

impl std::ops::Index<usize> for Pt4 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index {} is out of bounds.", index),
        }
    }
}

impl std::ops::IndexMut<usize> for Pt4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index {} is out of bounds.", index),
        }
    }
}

impl std::ops::Add for Pt4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl std::ops::AddAssign for Pt4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Pt4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl std::ops::SubAssign for Pt4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<f64> for Pt4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl std::ops::MulAssign<f64> for Pt4 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<f64> for Pt4 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl std::ops::DivAssign<f64> for Pt4 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl std::ops::Neg for Pt4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Pt4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Pt4::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
            0.0,
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
        Self::new(self.x / l, self.y / l, self.z / l, 0.0)
    }

    pub fn lerp(self, b: Self, t: f64) -> Self {
        self + (b - self) * t
    }

    pub fn as_pt3(&self) -> Pt3 {
        Pt3::new(self.x, self.y, self.z)
    }
}
