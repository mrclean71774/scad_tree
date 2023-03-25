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

use crate::{dcos, dsin, Pt2, Pt2s};

pub fn arc(start: Pt2, degrees: f64, segments: u64) -> Pt2s {
    assert!(degrees <= 360.0);
    let n_pts = if degrees == 360.0 {
        segments
    } else {
        segments + 1
    };
    let mut pts = Pt2s::with_capacity(n_pts as usize);
    for i in 0..n_pts {
        let a = i as f64 * -degrees / segments as f64;
        pts.push(start.rotated(a));
    }
    pts
}

pub fn circle(radius: f64, segments: u64) -> Pt2s {
    arc(Pt2::new(radius, 0.0), 360.0, segments)
}

/// radius: the radius of the circle surrounding the polygon
pub fn inscribed_polygon(n_sides: u64, radius: f64) -> Pt2s {
    circle(radius, n_sides)
}

/// radius: the radius of the circle inside the polygon
pub fn circumscribed_polygon(n_sides: u64, radius: f64) -> Pt2s {
    let radius = radius / dcos(180.0 / n_sides as f64);
    inscribed_polygon(n_sides, radius)
}

pub fn rounded_rect(width: f64, height: f64, radius: f64, segments: u64, center: bool) -> Pt2s {
    let mut tr = arc(Pt2::new(0.0, radius), 90.0, segments);
    tr.translate(Pt2::new(width - radius, height - radius));
    let mut br = arc(Pt2::new(radius, 0.0), 90.0, segments);
    br.translate(Pt2::new(width - radius, radius));
    let mut bl = arc(Pt2::new(-0.0, -radius), 90.0, segments);
    bl.translate(Pt2::new(radius, radius));
    let mut tl = arc(Pt2::new(-radius, 0.0), 90.0, segments);
    tl.translate(Pt2::new(radius, height - radius));

    tr.append(&mut br);
    tr.append(&mut bl);
    tr.append(&mut tl);

    if center {
        tr.translate(Pt2::new(-width / 2.0, -height / 2.0));
    }
    tr
}

pub fn chamfer(size: f64, oversize: f64) -> Pt2s {
    Pt2s::from_pt2s(vec![
        Pt2::new(0.0, size + oversize),
        Pt2::new(oversize, size + oversize),
        Pt2::new(oversize, size),
        Pt2::new(size, oversize),
        Pt2::new(size + oversize, oversize),
        Pt2::new(oversize + size, 0.0),
        Pt2::new(0.0, 0.0),
    ])
}

pub fn quadratic_bezier(start: Pt2, control: Pt2, end: Pt2, segments: u64) -> Pt2s {
    let delta = 1.0 / segments as f64;
    let mut points = Pt2s::new();
    for i in 0..(segments + 1) {
        let t = i as f64 * delta;
        points.push(start * (1.0 - t) * (1.0 - t) + control * t * (1.0 - t) * 2.0 + end * t * t);
    }
    points
}

pub fn cubic_bezier(start: Pt2, control1: Pt2, control2: Pt2, end: Pt2, segments: u64) -> Pt2s {
    let delta = 1.0 / segments as f64;
    let mut points = Pt2s::new();
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

pub fn star(n_points: usize, inner_radius: f64, outer_radius: f64) -> Pt2s {
    let angle = -360.0 / n_points as f64;
    let mut points = Pt2s::new();
    for i in 0..n_points {
        points.push(Pt2::new(
            dcos(angle * i as f64) * inner_radius,
            dsin(angle * i as f64) * inner_radius,
        ));
        points.push(Pt2::new(
            dcos(angle * (i as f64 + 0.5)) * outer_radius,
            dsin(angle * (i as f64 + 0.5)) * outer_radius,
        ));
    }
    points
}

pub fn bezier_star(
    n_points: u64,
    inner_radius: f64,
    inner_handle_length: f64,
    outer_radius: f64,
    outer_handle_length: f64,
    segments: u64,
) -> Pt2s {
    let mut controls = Vec::new();
    let mut knots = Vec::new();

    let angle = 360.0 / n_points as f64;
    for i in 0..n_points {
        knots.push(Pt2::new(
            dcos(angle * i as f64) * outer_radius,
            dsin(angle * i as f64) * outer_radius,
        ));
        knots.push(Pt2::new(
            dcos(angle * (i as f64 + 0.5)) * inner_radius,
            dsin(angle * (i as f64 + 0.5)) * inner_radius,
        ));
    }
    let n_knots = knots.len();
    for i in 0..n_knots {
        controls.push(
            knots[(i + 1) % n_knots]
                - (knots[(i + 2) % n_knots] - knots[i]).normalized()
                    * if i % 2 == 0 {
                        inner_handle_length
                    } else {
                        outer_handle_length
                    },
        );
    }

    let mut chain = CubicBezierChain2D::new(knots[0], controls[0], controls[0], knots[1], segments);
    for i in 1..(n_knots - 1) {
        chain.add(
            if i % 2 == 0 {
                outer_handle_length
            } else {
                inner_handle_length
            },
            controls[i],
            knots[i + 1],
            segments,
        );
    }
    chain.close(
        inner_handle_length,
        controls[n_knots - 1],
        outer_handle_length,
        segments,
    );

    chain.gen_points()
}

#[derive(Clone, Copy)]
pub struct QuadraticBezier2D {
    pub start: Pt2,
    pub control: Pt2,
    pub end: Pt2,
    pub segments: u64,
}

impl QuadraticBezier2D {
    pub fn new(start: Pt2, control: Pt2, end: Pt2, segments: u64) -> Self {
        Self {
            start,
            control,
            end,
            segments,
        }
    }

    pub fn gen_points(&self) -> Pt2s {
        quadratic_bezier(self.start, self.control, self.end, self.segments)
    }
}

#[derive(Clone, Copy)]
pub struct CubicBezier2D {
    pub start: Pt2,
    pub control1: Pt2,
    pub control2: Pt2,
    pub end: Pt2,
    pub segments: u64,
}

impl CubicBezier2D {
    pub fn new(start: Pt2, control1: Pt2, control2: Pt2, end: Pt2, segments: u64) -> Self {
        Self {
            start,
            control1,
            control2,
            end,
            segments,
        }
    }

    pub fn gen_points(&self) -> Pt2s {
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
pub struct CubicBezierChain2D {
    pub curves: Vec<CubicBezier2D>,
    closed: bool,
}

impl CubicBezierChain2D {
    pub fn new(start: Pt2, control1: Pt2, control2: Pt2, end: Pt2, segments: u64) -> Self {
        Self {
            curves: vec![CubicBezier2D {
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
        control2: Pt2,
        end: Pt2,
        segments: u64,
    ) -> &mut Self {
        let chain_end = &self.curves[self.curves.len() - 1];
        self.curves.push(CubicBezier2D {
            start: chain_end.end,
            control1: chain_end.end
                + (chain_end.end - chain_end.control2).normalized() * control1_length,
            control2: control2,
            end: end,
            segments,
        });
        self
    }

    pub fn close(
        &mut self,
        control1_length: f64,
        control2: Pt2,
        start_control1_len: f64,
        segments: u64,
    ) {
        self.closed = true;
        self.add(control1_length, control2, self.curves[0].start, segments);
        let chain_end = &self.curves[self.curves.len() - 1];
        self.curves[0].control1 =
            chain_end.end + (chain_end.end - chain_end.control2).normalized() * start_control1_len;
    }

    pub fn gen_points(&self) -> Pt2s {
        let mut pts = Pt2s::from_pt2s(vec![Pt2::new(0.0, 0.0)]);
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
