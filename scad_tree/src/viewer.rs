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

use scad_tree_math::Mt4;

use crate::{prelude::*, CubicBezier2D};

pub struct Viewer {
    point_radius: f64,
    edge_radius: f64,
    segments: u64,
    scad: Option<Scad>,
}

impl Viewer {
    pub fn new(point_radius: f64, edge_radius: f64, segments: u64) -> Self {
        Self {
            point_radius,
            edge_radius,
            segments,
            scad: None,
        }
    }

    pub fn add_point(&mut self, point: Pt3, color: ScadColor) {
        let s = translate!([point.x, point.y, point.z],
            color!(c=color,
                sphere!(self.point_radius, fn=self.segments);
            );
        );
        if let Some(scad) = &mut self.scad {
            self.scad = Some(scad.clone() + s);
        } else {
            self.scad = Some(s);
        }
    }

    pub fn add_pt2s(&mut self, points: &Pt2s, color: ScadColor) {
        let mut children = Vec::with_capacity(points.len());
        for point in points.iter() {
            let s = translate!([point.x, point.y, 0.0],
                sphere!(self.point_radius, fn=self.segments);
            );
            children.push(s);
        }
        let child = Scad {
            op: ScadOp::Color {
                rgba: None,
                color: Some(color),
                hex: None,
                alpha: Some(1.0),
            },
            children,
        };
        if let Some(scad) = &mut self.scad {
            self.scad = Some(Scad {
                op: ScadOp::Union,
                children: vec![scad.clone(), child],
            });
        } else {
            self.scad = Some(Scad {
                op: ScadOp::Union,
                children: vec![child],
            });
        }
    }

    pub fn add_edges(&mut self, edges: &Vec<(Pt3, Pt3)>, color: ScadColor) {
        let mut children = Vec::new();
        for (start, end) in edges {
            let matrix = Mt4::look_at_matrix_rh(*start, *end, Pt3::new(0.0, 0.0, 1.0));
            let mut c = dim3::cylinder(self.edge_radius, (*end - *start).len(), self.segments);
            c.apply_matrix(&matrix);
            c.translate(*end);

            children.push(polyhedron!(c.points, c.faces));
        }
        let child = Scad {
            op: ScadOp::Color {
                rgba: None,
                color: Some(color),
                hex: None,
                alpha: Some(1.0),
            },
            children,
        };
        if let Some(scad) = &mut self.scad {
            self.scad = Some(Scad {
                op: ScadOp::Union,
                children: vec![scad.clone(), child],
            });
        } else {
            self.scad = Some(Scad {
                op: ScadOp::Union,
                children: vec![child],
            });
        }
    }

    pub fn add_cubic_bezier2d(&mut self, curve: &CubicBezier2D) {
        let points = curve.gen_points();
        self.add_pt2s(&points, ScadColor::DarkSlateGray);

        let edge_count = points.len() - 1;
        let mut edges = Vec::with_capacity(edge_count + 2); // 2 extra for handles
        for i in 0..points.len() - 1 {
            edges.push((points[i].as_pt3(0.0), points[i + 1].as_pt3(0.0)));
        }

        self.add_edges(&edges, ScadColor::White);
        self.add_edges(
            &vec![
                (curve.start.as_pt3(0.0), curve.control1.as_pt3(0.0)),
                (curve.end.as_pt3(0.0), curve.control2.as_pt3(0.0)),
            ],
            ScadColor::Green,
        );
        self.add_point(curve.control1.as_pt3(0.0), ScadColor::Green);
        self.add_point(curve.control2.as_pt3(0.0), ScadColor::Green);
    }

    pub fn into_scad(self) -> Scad {
        self.scad.unwrap()
    }
}
