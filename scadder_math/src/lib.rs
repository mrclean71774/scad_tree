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

mod mt4;
mod pt2;
mod pt3;
mod pt4;
mod rng;

pub use crate::{
    mt4::Mt4,
    pt2::{Pt2, Pt2s},
    pt3::{Pt3, Pt3s},
    pt4::{Pt4, Pt4s},
    rng::MersenneTwister,
};

/// Returns the sine of degrees
#[inline(always)]
pub fn dsin(degrees: f64) -> f64 {
    degrees.to_radians().sin()
}

/// Returns the cosine of degrees
#[inline(always)]
pub fn dcos(degrees: f64) -> f64 {
    degrees.to_radians().cos()
}

/// Returns the tangent of degrees
#[inline(always)]
pub fn dtan(degrees: f64) -> f64 {
    degrees.to_radians().tan()
}

/// Returns the arc-sine of degrees
#[inline(always)]
pub fn dasin(degrees: f64) -> f64 {
    degrees.to_radians().asin()
}

/// Returns the arc-cosine of degrees
#[inline(always)]
pub fn dacos(degrees: f64) -> f64 {
    degrees.to_radians().acos()
}

/// Returns the arc-tangent of degrees
#[inline(always)]
pub fn datan(degrees: f64) -> f64 {
    degrees.to_radians().atan()
}

/// Returns true if a and b are within epsilon
#[inline(always)]
pub fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}
