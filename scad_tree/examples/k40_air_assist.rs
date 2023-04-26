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

//! In this example we use the Pipe structs associated functions and scad macros
//! to make an air assist for a K40 CO2 laser.

use scad_tree::prelude::*;

fn main() {
    scad_file!(32, "output/k40_air_assist.scad",
        union!(
            translate!([0.0, 0.0, 50.8],
                Pipe::straight(30.0, 2.0, 8.0, false, 50);
                translate!([18.5, 0.0, 0.0],
                    Pipe::straight(11.0, 2.0, 8.0, false, 50);
                );
            );
            translate!([18.5, 0.0, 50.8],
                rotate!([180.0, 0.0, 0.0],
                    Pipe::curved(11.0, 2.0, 20.35, 0.0, 50);
                );
            );
            difference!(
                rotate!([0.0, 20.4, 0.0],
                    Pipe::tapered(5.0, 11.0, 2.0, 52.2, false, 50);
                );
                cube!(20.0, true);
            );
        );
    );
}
