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

//! This example makes a cap and bottle that can be 3D printed.

use scad_tree::prelude::*;

fn main() {
    make_bottle();
    make_cap();
}

fn make_cap() {
    let cylinder = Polyhedron::cylinder(23.0, 12.0, 128);
    let cylinder = polyhedron!(cylinder.points, cylinder.faces);
    let mut tap = metric_thread::tap(40, 14.0, 128, false, false);
    tap = translate!([0.0, 0.0, 2.0], tap;);

    let cap = cylinder - tap;
    cap.save("output/bottle_cap.scad");
}

fn make_bottle() {
    let mut outside_profile = Pt2s::new();
    outside_profile.push(Pt2::new(0.0, 125.0));
    outside_profile.append(&mut dim2::cubic_bezier(
        Pt2::new(19.0, 125.0),
        Pt2::new(22.5, 110.0),
        Pt2::new(32.5, 105.0),
        Pt2::new(32.5, 100.0),
        6,
    ));
    outside_profile.append(&mut dim2::quadratic_bezier(
        Pt2::new(32.5, 5.0),
        Pt2::new(32.5, 0.0),
        Pt2::new(27.5, 0.0),
        6,
    ));
    outside_profile.push(Pt2::new(0.0, 0.0));

    let mut inside_profile = Pt2s::new();
    inside_profile.push(Pt2::new(0.0, 140.0));
    inside_profile.push(Pt2::new(16.0, 140.0));
    inside_profile.append(&mut dim2::cubic_bezier(
        Pt2::new(16.0, 125.0),
        Pt2::new(20.5, 110.0),
        Pt2::new(30.5, 105.0),
        Pt2::new(30.5, 100.0),
        6,
    ));
    inside_profile.append(&mut dim2::quadratic_bezier(
        Pt2::new(30.5, 7.0),
        Pt2::new(30.5, 2.0),
        Pt2::new(25.5, 2.0),
        6,
    ));
    inside_profile.push(Pt2::new(0.0, 2.0));

    let outside = rotate_extrude!(angle=360.0, convexity=10, fn=128, polygon!(outside_profile););
    let inside = rotate_extrude!(angle=360.0, convexity=10, fn=128, polygon!(inside_profile););

    let threaded_rod = translate!([0.0,0.0,120.0], metric_thread::threaded_rod(40, 15.0, 128, 0.0, 180.0, false, false););

    let bottle = outside + threaded_rod - inside;

    bottle.save("output/bottle.scad");
}
