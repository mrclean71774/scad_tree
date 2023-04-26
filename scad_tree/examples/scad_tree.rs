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

use scad_tree::prelude::*;

fn main() {
    // The Scad struct and the ScadOp enum are the main types in the library.
    // This tree is the difference of a cube and a sphere but it's a little
    // unwieldy to write.
    Scad {
        op: ScadOp::Difference,
        children: vec![
            Scad {
                op: ScadOp::Cube {
                    size: Pt3::new(2.0, 2.0, 2.0),
                    center: false,
                },
                children: Vec::new(),
            },
            Scad {
                op: ScadOp::Sphere {
                    radius: 1.0,
                    fa: None,
                    fs: None,
                    fn_: Some(24),
                },
                children: Vec::new(),
            },
        ],
    }
    .save("output/scad_tree1.scad");

    // Thats where the macros come in. All the operations from the 2D, 3D, and transformations
    // sections of the OpenSCAD cheatsheet (https://openscad.org/cheatsheet) are covered by macros.
    // All of the macros except scad_file expand to a Scad struct literal like above. The scad_file
    // macro specifies the file to save and allows setting $fa, $fs, and $fn globally.

    // This snippet of macro code produces the tree above but is a bit easier to read and write.
    // If you squint hard enough it resembles OpenSCAD code!
    scad_file!(32,
        "output/scad_tree2.scad",
        difference!(
            cube!(2.0);
            sphere!(1.0, fn=24);
        );
    );

    // Maybe your not a fan of OpenSCAD structured code. Since each macro expands to part of a tree
    // it's easy to save to variables or return a Scad from a funtion. This code produces the same
    // output as the above.
    let cube = cube!(2.0);
    let sphere = sphere!(1.0, fn=24);
    let difference = difference!(cube; sphere;);
    difference.save("output/scad_tree3.scad");

    // Maybe you want it to look like math!
    let cube = cube!(2.0);
    let sphere = sphere!(1.0, fn=24);
    (cube - sphere).save("output/scad_tree4.scad");
}
