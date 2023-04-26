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

use crate::prelude::*;

/// Viewer struct is used to view points, edges, and curves in OpenSCAD.
pub struct Viewer {
    point_radius: f64,
    edge_radius: f64,
    segments: u64,
    scad: Option<Box<Scad>>,
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

    pub fn add_pt2(&mut self, point: Pt2, color: ScadColor) {
        let s = translate!([point.x, point.y, 0.0],
            color!(c=color,
                sphere!(self.point_radius, fn=self.segments);
            );
        );
        if let Some(scad) = &mut self.scad {
            self.scad = Some(Box::new(*scad.clone() + s));
        } else {
            self.scad = Some(Box::new(s));
        }
    }

    pub fn add_pt3(&mut self, point: Pt3, color: ScadColor) {
        let s = translate!([point.x, point.y, point.z],
            color!(c=color,
                sphere!(self.point_radius, fn=self.segments);
            );
        );
        if let Some(scad) = &mut self.scad {
            self.scad = Some(Box::new(*scad.clone() + s));
        } else {
            self.scad = Some(Box::new(s));
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
            self.scad = Some(Box::new(Scad {
                op: ScadOp::Union,
                children: vec![*scad.clone(), child],
            }));
        } else {
            self.scad = Some(Box::new(Scad {
                op: ScadOp::Union,
                children: vec![child],
            }));
        }
    }

    pub fn add_pt3s(&mut self, points: &Pt3s, color: ScadColor) {
        let mut children = Vec::with_capacity(points.len());
        for point in points.iter() {
            let s = translate!([point.x, point.y, point.z],
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
            self.scad = Some(Box::new(Scad {
                op: ScadOp::Union,
                children: vec![*scad.clone(), child],
            }));
        } else {
            self.scad = Some(Box::new(Scad {
                op: ScadOp::Union,
                children: vec![child],
            }));
        }
    }

    pub fn add_lines2d(&mut self, edges: &Vec<(Pt2, Pt2)>, color: ScadColor) {
        let mut children = Vec::new();
        for (start, end) in edges {
            let matrix =
                Mt4::look_at_matrix_lh(start.as_pt3(0.0), end.as_pt3(0.0), Pt3::new(0.0, 0.0, 1.0));
            let mut c =
                Polyhedron::cylinder(self.edge_radius, (*end - *start).len(), self.segments);
            c.apply_matrix(&matrix);
            c.translate(start.as_pt3(0.0));

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
            self.scad = Some(Box::new(Scad {
                op: ScadOp::Union,
                children: vec![*scad.clone(), child],
            }));
        } else {
            self.scad = Some(Box::new(Scad {
                op: ScadOp::Union,
                children: vec![child],
            }));
        }
    }

    pub fn add_lines3d(&mut self, edges: &Vec<(Pt3, Pt3)>, color: ScadColor) {
        let mut children = Vec::new();
        for (start, end) in edges {
            let matrix = Mt4::look_at_matrix_lh(*start, *end, Pt3::new(0.0, 0.0, 1.0));
            let mut c =
                Polyhedron::cylinder(self.edge_radius, (*end - *start).len(), self.segments);
            c.apply_matrix(&matrix);
            c.translate(*start);

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
            self.scad = Some(Box::new(Scad {
                op: ScadOp::Union,
                children: vec![*scad.clone(), child],
            }));
        } else {
            self.scad = Some(Box::new(Scad {
                op: ScadOp::Union,
                children: vec![child],
            }));
        }
    }

    pub fn add_quadratic_bezier2d(&mut self, curve: &QuadraticBezier2D) {
        let points = curve.gen_points();
        self.add_pt2s(&points, ScadColor::DarkSlateGray);

        let edge_count = points.len() - 1;
        let mut edges = Vec::with_capacity(edge_count + 2); // 2 extra for handles
        for i in 0..points.len() - 1 {
            edges.push((points[i], points[i + 1]));
        }

        self.add_lines2d(&edges, ScadColor::White);
        self.add_lines2d(
            &vec![(curve.start, curve.control), (curve.end, curve.control)],
            ScadColor::Green,
        );
        self.add_pt2(curve.control, ScadColor::Green);
    }

    pub fn add_quadratic_bezier3d(&mut self, curve: &QuadraticBezier3D) {
        let points = curve.gen_points();
        self.add_pt3s(&points, ScadColor::DarkSlateGray);

        let edge_count = points.len() - 1;
        let mut edges = Vec::with_capacity(edge_count + 2); // 2 extra for handles
        for i in 0..points.len() - 1 {
            edges.push((points[i], points[i + 1]));
        }

        self.add_lines3d(&edges, ScadColor::White);
        self.add_lines3d(
            &vec![(curve.start, curve.control), (curve.end, curve.control)],
            ScadColor::Green,
        );
        self.add_pt3(curve.control, ScadColor::Green);
    }

    pub fn add_cubic_bezier2d(&mut self, curve: &CubicBezier2D) {
        let points = curve.gen_points();
        self.add_pt2s(&points, ScadColor::DarkSlateGray);

        let edge_count = points.len() - 1;
        let mut edges = Vec::with_capacity(edge_count + 2); // 2 extra for handles
        for i in 0..points.len() - 1 {
            edges.push((points[i], points[i + 1]));
        }

        self.add_lines2d(&edges, ScadColor::White);
        self.add_lines2d(
            &vec![(curve.start, curve.control1), (curve.end, curve.control2)],
            ScadColor::Green,
        );
        self.add_pt2(curve.control1, ScadColor::Green);
        self.add_pt2(curve.control2, ScadColor::Green);
    }

    pub fn add_cubic_bezier3d(&mut self, curve: &CubicBezier3D) {
        let points = curve.gen_points();
        self.add_pt3s(&points, ScadColor::DarkSlateGray);

        let edge_count = points.len() - 1;
        let mut edges = Vec::with_capacity(edge_count + 2); // 2 extra for handles
        for i in 0..points.len() - 1 {
            edges.push((points[i], points[i + 1]));
        }

        self.add_lines3d(&edges, ScadColor::White);
        self.add_lines3d(
            &vec![(curve.start, curve.control1), (curve.end, curve.control2)],
            ScadColor::Green,
        );
        self.add_pt3(curve.control1, ScadColor::Green);
        self.add_pt3(curve.control2, ScadColor::Green);
    }

    pub fn add_cubic_bezier_chain2d(&mut self, curve: &CubicBezierChain2D) {
        for c in &curve.curves {
            self.add_cubic_bezier2d(c);
        }
    }

    pub fn add_cubic_bezier_chain3d(&mut self, curve: &CubicBezierChain3D) {
        for c in &curve.curves {
            self.add_cubic_bezier3d(c);
        }
    }

    pub fn add_bezier_star(&mut self, star: &BezierStar) {
        self.add_cubic_bezier_chain2d(&star.chain);
    }

    pub fn into_scad(self) -> Scad {
        *self.scad.unwrap()
    }
}
