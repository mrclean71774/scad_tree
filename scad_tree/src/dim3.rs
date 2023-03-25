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

use crate::{
    dcos, dim2, dsin, triangulate2d, triangulate2d_rev, triangulate3d, triangulate3d_rev, Faces,
    Indices, Mt4, Pt2, Pt2s, Pt3, Pt3s,
};

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

    pub fn rotate_x(&mut self, degrees: f64) -> &mut Self {
        self.points.rotate_x(degrees);
        self
    }

    pub fn rotate_y(&mut self, degrees: f64) -> &mut Self {
        self.points.rotate_y(degrees);
        self
    }

    pub fn rotate_z(&mut self, degrees: f64) -> &mut Self {
        self.points.rotate_z(degrees);
        self
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

    pub fn rotate_extrude(profile: &Pt2s, degrees: f64, segments: usize) -> Self {
        assert!(degrees >= 0.0 && degrees <= 360.0);
        assert!(segments >= 3);
        let not_closed = degrees != 360.0;
        let profile: Pt3s =
            Pt3s::from_pt3s(profile.iter().map(|p| Pt3::new(p.x, 0.0, p.y)).collect());
        let profile_len = profile.len();
        let a = degrees / segments as f64;
        let mut points = profile.clone();
        let mut faces = Faces::new();

        if not_closed {
            // triangulate the starting face
            let triangles = triangulate3d(&profile, Pt3::new(0.0, -1.0, 0.0));
            for i in (0..triangles.len()).step_by(3) {
                faces.push(Indices::from_indices(vec![
                    triangles[i] as u64,
                    triangles[i + 1] as u64,
                    triangles[i + 2] as u64,
                ]));
            }
        }

        for segment in 1..segments {
            let s = dsin(a * segment as f64);
            let c = dcos(a * segment as f64);
            for p in 0..profile_len {
                points.push(Pt3::new(profile[p].x * c, profile[p].x * s, profile[p].z));
                let p0 = (segment - 1) * profile_len + p;
                let p1 = (segment - 1) * profile_len + ((p + 1) % profile_len);
                let p2 = segment * profile_len + ((p + 1) % profile_len);
                let p3 = segment * profile_len + p;
                faces.push(Indices::from_indices(vec![
                    p0 as u64, p1 as u64, p2 as u64, p3 as u64,
                ]));
            }
        }

        if not_closed {
            let s = dsin(a * segments as f64);
            let c = dcos(a * segments as f64);
            for p in 0..profile_len {
                points.push(Pt3::new(profile[p].x * c, profile[p].x * s, profile[p].z));
                let p0 = (segments - 1) * profile_len + p;
                let p1 = (segments - 1) * profile_len + ((p + 1) % profile_len);
                let p2 = segments * profile_len + ((p + 1) % profile_len);
                let p3 = segments * profile_len + p;
                faces.push(Indices::from_indices(vec![
                    p0 as u64, p1 as u64, p2 as u64, p3 as u64,
                ]));
            }
            let nml = Pt3::new(0.0, -1.0, 0.0).rotated_z(degrees + 180.0);
            let triangles = triangulate3d_rev(&profile, nml);
            for i in (0..triangles.len()).step_by(3) {
                faces.push(Indices::from_indices(vec![
                    triangles[i] as u64 + (segments * profile_len) as u64,
                    triangles[i + 1] as u64 + (segments * profile_len) as u64,
                    triangles[i + 2] as u64 + (segments * profile_len) as u64,
                ]));
            }
        } else {
            for p in 0..profile_len {
                let p0 = (segments - 1) * profile_len + p;
                let p1 = (segments - 1) * profile_len + ((p + 1) % profile_len);
                let p2 = (p + 1) % profile_len;
                let p3 = p;
                faces.push(Indices::from_indices(vec![
                    p0 as u64, p1 as u64, p2 as u64, p3 as u64,
                ]));
            }
        }
        Polyhedron { points, faces }
    }

    pub fn cylinder(radius: f64, height: f64, segments: u64) -> Self {
        Self::linear_extrude(&dim2::circle(radius, segments), height)
    }

    /// Creates a linear shape that can be used to chamfer a corner.
    ///
    /// size: The height and width of the angled part of the chamfer.
    ///
    /// oversize: How much non-angled part there is on the chamfer.
    ///
    /// return: The mesh.
    pub fn chamfer(size: f64, length: f64, oversize: f64) -> Self {
        Self::linear_extrude(&dim2::chamfer(size, oversize), length)
    }

    /// Creates a curved chamfer shape.
    ///
    /// size: The size of the angled part of the chamfer profile.
    ///
    /// oversize: How much non-angled part there is on the chamfer.
    ///
    /// radius: The radius of the arc that the chamfer takes.
    ///
    /// degrees: The degrees of the arc that the chamfer is extruded through.
    ///
    /// segments: The number of segments in a circle.
    ///
    /// return: The mesh.
    pub fn external_circle_chamfer(
        size: f64,
        oversize: f64,
        radius: f64,
        degrees: f64,
        segments: usize,
    ) -> Self {
        let mut points2 = dim2::chamfer(size, oversize);
        points2
            .rotate(90.0)
            .translate(Pt2::new(radius + size / 2.0 + oversize / 2.0, -oversize));
        Self::rotate_extrude(&points2, degrees, segments)
    }

    /// Creates two external circle chamfers for chamfering a cylinder.
    ///
    /// size: The size of the angled part of the chamfer profile.
    ///
    /// oversize: How much non-angled part there is on the chamfer.
    ///
    /// radius: The radius of the cylinder to be chamfered.
    ///
    /// height: The height of the cylinder to be chamfered.
    ///
    /// segments: The number of segments in a circle.
    ///
    /// return: The mesh.
    pub fn external_cylinder_chamfer(
        size: f64,
        oversize: f64,
        radius: f64,
        height: f64,
        segments: usize,
        center: bool,
    ) -> (Self, Self) {
        let mut result = Self::external_circle_chamfer(size, oversize, radius, 360.0, segments);
        let mut result1 = Self::external_circle_chamfer(size, oversize, radius, 360.0, segments);
        result1.rotate_x(180.0);
        result1.translate(Pt3::new(0.0, 0.0, height));
        if center {
            result.translate(Pt3::new(0.0, 0.0, -height / 2.0));
            result1.translate(Pt3::new(0.0, 0.0, -height / 2.0));
        }
        (result, result1)
    }
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
