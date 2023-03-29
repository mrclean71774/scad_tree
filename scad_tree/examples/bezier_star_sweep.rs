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

//! This example uses the BezierStar struct to create a profile and a path
//! for the Polyhedron::sweep function. Both of the stars are the same shape
//! but the path is ten times larger than the profile. The profile is swept
//! along the path, spinning seven times along the way.
//!
//! The output_viewer boolean controls whether we output the view of the curves
//! or the shape created by the sweep. Outputting the shape will most likely stall
//! OpenSCAD for several minutes.
//!
//! We use two Viewer(s) in this example so we can have different sized points and
//! lines.

use scad_tree::prelude::*;

fn main() {
    let output_viewer = false;
    let path = BezierStar::new(7, 20.0, 9.0, 40.0, 15.0, 12);
    let profile = BezierStar::new(7, 2.0, 0.9, 4.0, 1.5, 12);

    if output_viewer {
        let mut viewer = Viewer::new(0.5, 0.25, 6);
        viewer.add_bezier_star(&path);
        let mut small_viewer = Viewer::new(0.05, 0.025, 6);
        small_viewer.add_bezier_star(&profile);
        scad_file!("output/bezier_star_sweep.scad",
            small_viewer.into_scad() + viewer.into_scad();
        );
    } else {
        let path = Pt3s::from_pt3s(path.gen_points().iter().map(|p| p.as_pt3(0.0)).collect());
        let profile = profile.gen_points();
        let star_swept = Polyhedron::sweep(&profile, &path, 7.0 * 360.0, true);
        scad_file!("output/bezier_star_sweep.scad",
            star_swept.into_scad();
        );
    }
}
