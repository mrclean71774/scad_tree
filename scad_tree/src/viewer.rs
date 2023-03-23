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

use crate::prelude::*;

pub struct Viewer {
    point_radius: f64,
    segments: u64,
    scad: Option<Scad>,
}

impl Viewer {
    pub fn new(point_radius: f64, segments: u64) -> Self {
        Self {
            point_radius,
            segments,
            scad: None,
        }
    }

    pub fn add_point(&mut self, point: Pt3) {
        let s = translate!([point.x, point.y, point.z],
            sphere!(self.point_radius, fn=self.segments);
        );
        if let Some(scad) = &mut self.scad {
            self.scad = Some(scad.clone() + s);
        } else {
            self.scad = Some(s);
        }
    }

    pub fn add_pt2s(&mut self, points: Pt2s) {
        for point in points.iter() {
            self.add_point(point.as_pt3(0.0));
        }
    }

    pub fn into_scad(self) -> Scad {
        self.scad.unwrap()
    }
}
