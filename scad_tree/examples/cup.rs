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

//! This example creates a cup.

use scad_tree::prelude::*;

fn main() {
    let segments: u64 = 72;
    let cup_segments: u64 = 144;
    let slices: u64 = 48;

    let mut cup_blank_profile = Pt2s::new();
    cup_blank_profile.push(Pt2::new(0.0, 0.0));
    cup_blank_profile.append(&mut dim2::cubic_bezier(
        Pt2::new(40.0, 0.0),
        Pt2::new(40.0, 33.0),
        Pt2::new(60.0, 66.0),
        Pt2::new(60.0, 100.0),
        slices,
    ));
    cup_blank_profile.push(Pt2::new(0.0, 100.0));

    let cup_blank = rotate_extrude!(angle=360.0, convexity=2, fn=cup_segments,
        polygon!(cup_blank_profile);
    );

    let mut cup_inner_profile = Pt2s::new();
    cup_inner_profile.push(Pt2::new(0.0, 3.0));
    cup_inner_profile.append(&mut dim2::cubic_bezier(
        Pt2::new(37.0, 3.0),
        Pt2::new(37.0, 33.0),
        Pt2::new(57.0, 66.0),
        Pt2::new(57.0, 103.0),
        slices,
    ));
    cup_inner_profile.push(Pt2::new(0.0, 103.0));

    let cup_inner = rotate_extrude!(angle=360.0, convexity=1, fn=cup_segments,
        polygon!(cup_inner_profile);
    );

    let handle_path = dim3::cubic_bezier(
        Pt3::new(37.0, 20.0, 0.0),
        Pt3::new(70.0, 30.0, 0.0),
        Pt3::new(120.0, 90.0, 0.0),
        Pt3::new(57.0, 90.0, 0.0),
        segments,
    );

    let handle_profile = dim2::rounded_rect(8.0, 20.0, 2.5, segments, true);
    let mut handle = Polyhedron::sweep(&handle_profile, &handle_path, 0.0, false);
    handle.rotate_x(90.0);

    let cup = cup_blank + handle.into_scad_with_convexity(2) - cup_inner;

    cup.save("output/cup.scad");
}
