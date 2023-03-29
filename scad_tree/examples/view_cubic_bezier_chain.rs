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

//! In this example we use the Viewer struct to visualize a cubic bezier chain.
//! This example is 2d but the viewer is not limited to 2d. A 3D bezier chain
//! example would be the same with dim2 changed to dim3, Pt2 changed to Pt3 etc.
//!
//! The curve segments will be white with gray points while the control handles
//! are shown in green.

use scad_tree::prelude::*;

fn main() {
    let mut curve = dim2::CubicBezierChain2D::new(
        Pt2::new(0.0, 0.0),
        Pt2::new(40.0, -20.0),
        Pt2::new(40.0, 80.0),
        Pt2::new(0.0, 40.0),
        24,
    );
    curve.add(20.0, Pt2::new(-30.0, 50.0), Pt2::new(-50.0, 40.0), 12);

    let mut viewer = Viewer::new(0.5, 0.25, 6);
    viewer.add_cubic_bezier_chain2d(&curve);
    scad_file!("output/view_cubic_bezier_chain.scad",
        viewer.into_scad();
    );
}
