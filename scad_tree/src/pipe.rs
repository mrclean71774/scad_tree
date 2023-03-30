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

use crate::prelude::*;

pub struct Pipe;

impl Pipe {
    /// Create a straight pipe.
    pub fn straight(od: f64, wall_thickness: f64, length: f64, center: bool, fn_: u64) -> Scad {
        assert!(od - wall_thickness * 2.0 > 0.0);

        difference!(
            cylinder!(h=length, d1=od, d2=od, center=center, fn=fn_);
            translate!([0.0, 0.0, -1.0],
                cylinder!(h=length + 2.0, d1=od - wall_thickness * 2.0,
                    d2=od - wall_thickness * 2.0, center=center, fn=fn_);
            );
        )
    }

    /// Create a solid straight pipe.
    pub fn straight_solid(od: f64, length: f64, center: bool, fn_: u64) -> Scad {
        cylinder!(h=length, d1=od, d2=od, center=center, fn=fn_)
    }

    /// Create a curved pipe.
    ///
    /// #params
    ///
    /// od: The outside diameter of the pipe.
    ///
    /// wall_thickness: The wall thickness of the pipe.
    ///
    /// degrees: The total angle of the curve.
    ///
    /// radius: The radius of the curve at the center of the pipe.
    ///
    /// fn_: The $fn value for OpenSCAD
    ///
    /// return: A Scad struct literal.
    pub fn curved(od: f64, wall_thickness: f64, degrees: f64, radius: f64, fn_: u64) -> Scad {
        assert!(od - wall_thickness * 2.0 > 0.0);
        assert!(degrees > 0.0 && degrees <= 360.0);

        translate!([-od / 2.0 - radius, 0.0, 0.0],
            rotate!([90.0, 0.0, 0.0],
                rotate_extrude!(angle=degrees, convexity=4, fn=fn_,
                    translate!([od /2.0 + radius, 0.0, 0.0],
                        difference!(
                            circle!(d=od, fn=fn_);
                            circle!(d=od - wall_thickness * 2.0, fn=fn_);
                        );
                    );
                );
            );
        )
    }

    /// Create a curved solid pipe.
    ///
    /// #params
    ///
    /// od: The outside diameter of the pipe.
    ///
    /// wall_thickness: The wall thickness of the pipe.
    ///
    /// degrees: The total angle of the curve.
    ///
    /// radius: The radius of the curve at the center of the pipe.
    ///
    /// fn_: The $fn value for OpenSCAD
    ///
    /// return: A Scad struct literal.
    pub fn curved_solid(od: f64, degrees: f64, radius: f64, fn_: u64) -> Scad {
        assert!(degrees > 0.0 && degrees <= 360.0);

        translate!([od / 2.0 - radius, 0.0, 0.0],
            rotate!([90.0, 0.0, 0.0],
                rotate_extrude!(angle=degrees, convexity=4, fn=fn_,
                    translate!([od /2.0 + radius, 0.0, 0.0],
                        circle!(d=od, fn=fn_);
                    );
                );
            );
        )
    }

    /// Create a tapered pipe.
    pub fn tapered(
        od1: f64,
        od2: f64,
        wall_thickness: f64,
        length: f64,
        center: bool,
        fn_: u64,
    ) -> Scad {
        assert!(od1 - wall_thickness * 2.0 > 0.0);
        assert!(od2 - wall_thickness * 2.0 > 0.0);

        difference!(
            cylinder!(h=length, d1=od1, d2=od2, center=center, fn=fn_);
            translate!([0.0, 0.0, -0.001],
                cylinder!(h=length + 0.002, d1=od1 - wall_thickness * 2.0,
                    d2=od2 - wall_thickness * 2.0, center=center, fn=fn_);
            );
        )
    }

    /// Create a tapered solid pipe.
    pub fn tapered_solid(od1: f64, od2: f64, length: f64, center: bool, fn_: u64) -> Scad {
        cylinder!(h=length, d1=od1, d2=od2, center=center, fn=fn_)
    }
}
