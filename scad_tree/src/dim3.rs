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

use crate::{dim2, triangulate2d, triangulate2d_rev, Faces, Indices, Mt4, Pt2s, Pt3, Pt3s};

pub struct Polyhedron {
    pub points: Pt3s,
    pub faces: Faces,
}

impl Polyhedron {
    pub fn translate(&mut self, point: Pt3) {
        self.points.translate(point);
    }

    pub fn apply_matrix(&mut self, matrix: &Mt4) {
        self.points.apply_matrix(matrix);
    }
}

pub fn linear_extrude(points: &Pt2s, height: f64) -> Polyhedron {
    let indices = triangulate2d_rev(points);
    let mut vertices = Pt3s::with_capacity(points.len() * 2);
    for point in points.iter() {
        vertices.push(point.as_pt3(0.0));
    }

    let mut faces = Faces::with_capacity((points.len() - 2) * 2 + points.len() - 1);
    for i in (0..indices.len()).step_by(3) {
        faces.push(Indices::from_indices(vec![
            indices[i],
            indices[i + 1],
            indices[i + 2],
        ]));
    }

    let mut end_points = points.iter().map(|p| p.as_pt3(height)).collect();
    vertices.append(&mut end_points);
    let indices = triangulate2d(points);
    for i in (0..indices.len()).step_by(3) {
        faces.push(Indices::from_indices(vec![
            indices[i] + points.len() as u64,
            indices[i + 1] + points.len() as u64,
            indices[i + 2] + points.len() as u64,
        ]));
    }

    for i in 0..points.len() {
        let p0 = i;
        let p1 = (i + 1) % points.len();
        let p2 = (i + 1) % points.len() + points.len();
        let p3 = i + points.len();

        faces.push(Indices::from_indices(vec![
            p0 as u64, p1 as u64, p2 as u64, p3 as u64,
        ]));
    }

    Polyhedron {
        points: vertices,
        faces,
    }
}

pub fn cylinder(radius: f64, height: f64, segments: u64) -> Polyhedron {
    linear_extrude(&dim2::circle(radius, segments), height)
}

pub fn quadratic_bezier(start: Pt3, control: Pt3, end: Pt3, segments: u64) -> Pt3s {
    let delta = 1.0 / segments as f64;
    let mut points = Pt3s::new();
    for i in 0..(segments + 1) {
        let t = i as f64 * delta;
        points.push(start * (1.0 - t) * (1.0 - t) + control * t * (1.0 - t) * 2.0 + end * t * t);
    }
    points
}

pub fn cubic_bezier(start: Pt3, control1: Pt3, control2: Pt3, end: Pt3, segments: u64) -> Pt3s {
    let delta = 1.0 / segments as f64;
    let mut points = Pt3s::new();
    for i in 0..(segments + 1) {
        let t = i as f64 * delta;
        points.push(
            start * (1.0 - t) * (1.0 - t) * (1.0 - t)
                + control1 * t * (1.0 - t) * (1.0 - t) * 3.0
                + control2 * t * t * (1.0 - t) * 3.0
                + end * t * t * t,
        );
    }
    points
}

#[derive(Clone, Copy)]
pub struct QuadraticBezier3D {
    pub start: Pt3,
    pub control: Pt3,
    pub end: Pt3,
    pub segments: u64,
}

impl QuadraticBezier3D {
    pub fn new(start: Pt3, control: Pt3, end: Pt3, segments: u64) -> Self {
        Self {
            start,
            control,
            end,
            segments,
        }
    }

    pub fn gen_points(&self) -> Pt3s {
        quadratic_bezier(self.start, self.control, self.end, self.segments)
    }
}

#[derive(Clone, Copy)]
pub struct CubicBezier3D {
    pub start: Pt3,
    pub control1: Pt3,
    pub control2: Pt3,
    pub end: Pt3,
    pub segments: u64,
}

impl CubicBezier3D {
    pub fn new(start: Pt3, control1: Pt3, control2: Pt3, end: Pt3, segments: u64) -> Self {
        Self {
            start,
            control1,
            control2,
            end,
            segments,
        }
    }

    pub fn gen_points(&self) -> Pt3s {
        cubic_bezier(
            self.start,
            self.control1,
            self.control2,
            self.end,
            self.segments,
        )
    }
}

#[derive(Clone)]
pub struct CubicBezierChain3D {
    pub curves: Vec<CubicBezier3D>,
    closed: bool,
}

impl CubicBezierChain3D {
    pub fn new(start: Pt3, control1: Pt3, control2: Pt3, end: Pt3, segments: u64) -> Self {
        Self {
            curves: vec![CubicBezier3D {
                start,
                control1,
                control2,
                end,
                segments,
            }],
            closed: false,
        }
    }

    pub fn add(
        &mut self,
        control1_length: f64,
        control2: Pt3,
        end: Pt3,
        segments: u64,
    ) -> &mut Self {
        let chain_end = &self.curves[self.curves.len() - 1];
        self.curves.push(CubicBezier3D {
            start: chain_end.end,
            control1: chain_end.end
                + (chain_end.end - chain_end.control2).normalized() * control1_length,
            control2,
            end,
            segments,
        });
        self
    }

    pub fn close(
        &mut self,
        control1_length: f64,
        control2: Pt3,
        start_control1_len: f64,
        segments: u64,
    ) {
        self.closed = true;
        self.add(control1_length, control2, self.curves[0].start, segments);
        let chain_end = &self.curves[self.curves.len() - 1];
        self.curves[0].control1 =
            chain_end.end + (chain_end.end - chain_end.control2).normalized() * start_control1_len;
    }

    pub fn gen_points(&self) -> Pt3s {
        let mut pts = Pt3s::from_pt3s(vec![Pt3::new(0.0, 0.0, 0.0)]);
        for i in 0..self.curves.len() {
            pts.pop();
            pts.append(&mut cubic_bezier(
                self.curves[i].start,
                self.curves[i].control1,
                self.curves[i].control2,
                self.curves[i].end,
                self.curves[i].segments,
            ));
        }
        if self.closed {
            pts.pop();
        }
        pts
    }
}
