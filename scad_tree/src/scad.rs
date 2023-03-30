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

use {crate::prelude::*, std::io::Write};

/// The supported OpenSCAD operations.
#[derive(Clone, PartialEq)]
pub enum ScadOp {
    Union,
    Difference,
    Intersection,
    Circle {
        radius: f64,
        fa: Option<f64>,
        fs: Option<f64>,
        fn_: Option<u64>,
    },
    Square {
        size: Pt2,
        center: bool, // default false
    },
    Polygon {
        points: Pt2s,
        paths: Option<Paths>, // default None undef
        convexity: u64,       // default 1
    },
    Text {
        text: String,
        size: f64,                // default 10
        font: String,             // default "Liberation Sans"
        halign: TextHalign,       // default left
        valign: TextValign,       // default baseline
        spacing: f64,             // default 1
        direction: TextDirection, // default ltr
        language: String,         // default "en"
        script: String,           // default "latin"
        fn_: Option<u64>,         // None
    },
    Import {
        file: String,
        convexity: u64,
    },
    Projection {
        cut: bool,
    },
    Sphere {
        radius: f64,
        fa: Option<f64>,
        fs: Option<f64>,
        fn_: Option<u64>,
    },
    Cube {
        size: Pt3,
        center: bool,
    },
    Cylinder {
        height: f64,
        radius1: f64,
        radius2: f64,
        center: bool,
        fa: Option<f64>,
        fs: Option<f64>,
        fn_: Option<u64>,
    },
    Polyhedron {
        points: Pt3s,
        faces: Faces,
        convexity: u64,
    },
    LinearExtrude {
        height: f64,
        center: bool,
        convexity: u64,
        twist: f64,
        scale: Pt2,
        slices: Option<u64>,
        fn_: Option<u64>,
    },
    RotateExtrude {
        angle: f64,
        convexity: u64,
        fa: Option<f64>,
        fs: Option<f64>,
        fn_: Option<u64>,
    },
    Surface {
        file: String,
        center: bool,
        invert: bool,
        convexity: u64,
    },
    Translate {
        v: Pt3,
    },
    Rotate {
        a: Option<f64>,
        a_is_scalar: bool,
        v: Pt3,
    },
    Scale {
        v: Pt3,
    },
    Resize {
        newsize: Pt3,
        auto: bool,
        auto_is_vec: bool,
        autovec: (bool, bool, bool),
        convexity: u64,
    },
    Mirror {
        v: Pt3,
    },
    Color {
        rgba: Option<Pt4>,
        color: Option<ScadColor>,
        hex: Option<String>,
        alpha: Option<f64>,
    },
    Offset {
        r: Option<f64>,
        delta: Option<f64>,
        chamfer: bool,
    },
    Hull,
    Minkowski {
        convexity: u64,
    },
}

/// A tree of OpenSCAD operations.
///
/// Should not need to construct manually in end user code. We
/// have macros and functions to do it for us.
#[derive(Clone, PartialEq)]
pub struct Scad {
    pub op: ScadOp,
    pub children: Vec<Scad>,
}

impl Scad {
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
        segments: u64,
    ) -> Self {
        rotate_extrude!(angle=degrees, convexity=5, fn=segments,
            translate!([radius + size / 2.0 + oversize / 2.0, -oversize, 0.0],
                rotate!(90.0,
                    polygon!(dim2::chamfer(size, oversize));
                );
            );
        )
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
        segments: u64,
        center: bool,
    ) -> Self {
        let mut result = union!(
            Self::external_circle_chamfer(size, oversize, radius, 360.0, segments);
            translate!([0.0, 0.0, height],
                rotate!([180.0, 0.0, 0.0],
                    Self::external_circle_chamfer(size, oversize, radius, 360.0, segments);
                );
            );
        );
        if center {
            result = translate!([0.0, 0.0, -height/2.0], result;);
        }
        result
    }

    pub fn save(&self, path: &str) {
        let s = format!("{}", self);
        let mut file = std::fs::File::create(path).unwrap();
        file.write(s.as_bytes()).unwrap();
        file.flush().unwrap();
    }
}

impl std::ops::Sub for Scad {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        difference!(self; rhs;)
    }
}

impl std::ops::Add for Scad {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        union!(self; rhs;)
    }
}

/// Since we are outputting text we leverage the Display trait to format output.
impl std::fmt::Display for Scad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.op {
            ScadOp::Union => {
                write!(f, "union() {{\n")?;
            }
            ScadOp::Difference => {
                write!(f, "difference() {{\n")?;
            }
            ScadOp::Intersection => {
                write!(f, "intersection() {{\n")?;
            }
            ScadOp::Circle {
                radius,
                fa,
                fs,
                fn_,
            } => {
                write!(f, "circle(r={}", radius)?;
                if let Some(fa) = fa {
                    write!(f, ", $fa={}", fa)?;
                }
                if let Some(fs) = fs {
                    write!(f, ", $fs={}", fs)?;
                }
                if let Some(fn_) = fn_ {
                    write!(f, ", $fn={}", fn_)?;
                }
                write!(f, ");")?;
            }
            ScadOp::Square { size, center } => {
                write!(
                    f,
                    "square(size=[{}, {}], center={});",
                    size.x, size.y, center
                )?;
            }
            ScadOp::Polygon {
                points,
                paths,
                convexity,
            } => {
                if let Some(paths) = paths {
                    write!(
                        f,
                        "polygon(points={}, paths={} convexity={});",
                        points, paths, convexity
                    )?;
                } else {
                    write!(
                        f,
                        "polygon(points={}, paths=undef, convexity={});",
                        points, convexity
                    )?;
                }
            }
            ScadOp::Text {
                text,
                size,
                font,
                halign,
                valign,
                spacing,
                direction,
                language,
                script,
                fn_,
            } => {
                write!(f, "text(text={:?}, ", text)?;
                write!(f, "size={}, ", size)?;
                write!(f, "font={:?}, ", font)?;
                write!(f, "halign=\"{:?}\", ", halign)?;
                write!(f, "valign=\"{:?}\", ", valign)?;
                write!(f, "spacing={}, ", spacing)?;
                write!(f, "direction=\"{:?}\", ", direction)?;
                write!(f, "language={:?}, ", language)?;
                write!(f, "script={:?}", script)?;
                if let Some(fn_) = fn_ {
                    write!(f, ", $fn={}", fn_)?;
                }
                write!(f, ");")?;
            }
            ScadOp::Import { file, convexity } => {
                write!(f, "import({:?}, {});", file, convexity)?;
            }
            ScadOp::Projection { cut } => {
                write!(f, "projection(cut={}) {{\n", cut)?;
            }
            ScadOp::Sphere {
                radius,
                fa,
                fs,
                fn_,
            } => {
                write!(f, "sphere(r={}", radius)?;
                if let Some(fa) = fa {
                    write!(f, ", $fa={}", fa)?;
                }
                if let Some(fs) = fs {
                    write!(f, ", $fs={}", fs)?;
                }
                if let Some(fn_) = fn_ {
                    write!(f, ", $fn={}", fn_)?;
                }
                write!(f, ");")?;
            }
            ScadOp::Cube { size, center } => {
                write!(f, "cube(size={}, center={});", size, center)?;
            }
            ScadOp::Cylinder {
                height,
                radius1,
                radius2,
                center,
                fa,
                fs,
                fn_,
            } => {
                write!(
                    f,
                    "cylinder(h={}, r1={}, r2={}, center={}",
                    height, radius1, radius2, center
                )?;
                if let Some(fa) = fa {
                    write!(f, ", $fa={}", fa)?;
                }
                if let Some(fs) = fs {
                    write!(f, ", $fs={}", fs)?;
                }
                if let Some(fn_) = fn_ {
                    write!(f, ", $fn={}", fn_)?;
                }
                write!(f, ");")?;
            }
            ScadOp::Polyhedron {
                points,
                faces,
                convexity,
            } => {
                write!(
                    f,
                    "polyhedron(points={}, faces={}, convexity={});",
                    points, faces, convexity
                )?;
            }
            ScadOp::LinearExtrude {
                height,
                center,
                convexity,
                twist,
                scale,
                slices,
                fn_,
            } => {
                write!(
                    f,
                    "linear_extrude(height={}, center={}, convexity={}, twist={}, scale={}",
                    height, center, convexity, twist, scale
                )?;
                if let Some(slices) = slices {
                    write!(f, ", slices={}", slices)?;
                }
                if let Some(fn_) = fn_ {
                    write!(f, ", $fn={}", fn_)?;
                }
                write!(f, ") {{\n")?;
            }
            ScadOp::RotateExtrude {
                angle,
                convexity,
                fa,
                fs,
                fn_,
            } => {
                write!(f, "rotate_extrude(angle={}, convexity={}", angle, convexity)?;
                if let Some(fa) = fa {
                    write!(f, ", $fa={}", fa)?;
                }
                if let Some(fs) = fs {
                    write!(f, ", $fs={}", fs)?;
                }
                if let Some(fn_) = fn_ {
                    write!(f, ", $fn={}", fn_)?;
                }
                write!(f, ") {{\n")?;
            }
            ScadOp::Surface {
                file,
                center,
                invert,
                convexity,
            } => {
                write!(
                    f,
                    "surface(file={:?}, center={}, invert={}, convexity={});",
                    file, center, invert, convexity
                )?;
            }
            ScadOp::Translate { v } => {
                write!(f, "translate(v={}) {{\n", v)?;
            }
            ScadOp::Rotate { a, a_is_scalar, v } => {
                if let Some(a) = a {
                    if *a_is_scalar {
                        write!(f, "rotate(a={}) {{\n", a)?;
                    } else {
                        write!(f, "rotate(a={}, v={}) {{\n", a, v)?;
                    }
                } else {
                    write!(f, "rotate(a={}) {{\n", v)?;
                }
            }
            ScadOp::Scale { v } => {
                write!(f, "scale(v={}) {{\n", v)?;
            }
            ScadOp::Resize {
                newsize,
                auto,
                auto_is_vec,
                autovec,
                convexity,
            } => {
                if *auto_is_vec {
                    write!(
                        f,
                        "resize(newsize={}, auto={}, convexity={}) {{\n",
                        newsize, auto, convexity
                    )?;
                } else {
                    write!(
                        f,
                        "resize(newsize={}, auto=[{}, {}, {}], convexity={}) {{\n",
                        newsize, autovec.0, autovec.1, autovec.2, convexity
                    )?;
                }
            }
            ScadOp::Mirror { v } => {
                write!(f, "mirror(v={}) {{\n", v)?;
            }
            ScadOp::Color {
                rgba,
                color,
                hex,
                alpha,
            } => {
                if let Some(rgba) = rgba {
                    write!(f, "color(c={}) {{\n", rgba)?;
                } else if let Some(color) = color {
                    write!(f, "color(\"{:?}\"", color)?;
                    if let Some(alpha) = alpha {
                        write!(f, ", alpha={}", alpha)?;
                    }
                    write!(f, ") {{\n")?;
                } else if let Some(hex) = hex {
                    write!(f, "color({:?}) {{\n", hex)?;
                }
            }
            ScadOp::Offset { r, delta, chamfer } => {
                if let Some(r) = r {
                    write!(f, "offset(r={}) {{\n", r)?;
                } else if let Some(delta) = delta {
                    write!(f, "offset(delta={}, chamfer={}) {{\n", delta, chamfer)?;
                }
            }
            ScadOp::Hull => {
                write!(f, "hull() {{\n")?;
            }
            ScadOp::Minkowski { convexity } => {
                write!(f, "minkowski(convexity={}) {{\n", convexity)?;
            }
        } // end match
        for i in 0..self.children.len() {
            write!(f, "{}", self.children[i])?;
        }
        if self.children.len() > 0 {
            write!(f, "}}")?;
        }
        write!(f, "\n")
    }
}

/// Enum of all the named OpenSCAD colors
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScadColor {
    Lavender,
    Thistle,
    Plum,
    Violet,
    Orchid,
    Fuchsia,
    Magenta,
    MediumOrchid,
    MediumPurple,
    BlueViolet,
    DarkViolet,
    DarkOrchid,
    DarkMagenta,
    Purple,
    Indigo,
    DarkSlateBlue,
    SlateBlue,
    MediumSlateBlue,
    Pink,
    LightPink,
    HotPink,
    DeepPink,
    MediumVioletRed,
    PaleVioletRed,
    Aqua,
    Cyan,
    LightCyan,
    PaleTurquoise,
    Aquamarine,
    Turquoise,
    MediumTurquoise,
    DarkTurquoise,
    CadetBlue,
    SteelBlue,
    LightSteelBlue,
    PowderBlue,
    LightBlue,
    SkyBlue,
    LightSkyBlue,
    DeepSkyBlue,
    DodgerBlue,
    CornflowerBlue,
    RoyalBlue,
    Blue,
    MediumBlue,
    DarkBlue,
    Navy,
    MidnightBlue,
    IndianRed,
    LightCoral,
    Salmon,
    DarkSalmon,
    LightSalmon,
    Red,
    Crimson,
    FireBrick,
    DarkRed,
    GreenYellow,
    Chartreuse,
    LawnGreen,
    Lime,
    LimeGreen,
    PaleGreen,
    LightGreen,
    MediumSpringGreen,
    SpringGreen,
    MediumSeaGreen,
    SeaGreen,
    ForestGreen,
    Green,
    DarkGreen,
    YellowGreen,
    OliveDrab,
    Olive,
    DarkOliveGreen,
    MediumAquamarine,
    DarkSeaGreen,
    LightSeaGreen,
    DarkCyan,
    Teal,
    Coral,
    Tomato,
    OrangeRed,
    DarkOrange,
    Orange,
    Gold,
    Yellow,
    LightYellow,
    LemonChiffon,
    LightGoldenrodYellow,
    PapayaWhip,
    Moccasin,
    PeachPuff,
    PaleGoldenrod,
    Khaki,
    DarkKhaki,
    Browns,
    Cornsilk,
    BlanchedAlmond,
    Bisque,
    NavajoWhite,
    Wheat,
    BurlyWood,
    Tan,
    RosyBrown,
    SandyBrown,
    Goldenrod,
    DarkGoldenrod,
    Peru,
    Chocolate,
    SaddleBrown,
    Sienna,
    Brown,
    Maroon,
    White,
    Snow,
    Honeydew,
    MintCream,
    Azure,
    AliceBlue,
    GhostWhite,
    WhiteSmoke,
    Seashell,
    Beige,
    OldLace,
    FloralWhite,
    Ivory,
    AntiqueWhite,
    Linen,
    LavenderBlush,
    MistyRose,
    Gainsboro,
    LightGrey,
    Silver,
    DarkGray,
    Gray,
    DimGray,
    LightSlateGray,
    SlateGray,
    DarkSlateGray,
    Black,
}

/// The ways for horizontal alignment of text.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TextHalign {
    left, // default
    center,
    right,
}

/// The ways for vertical alignment of text.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TextValign {
    top,
    center,
    baseline, // default
    bottom,
}

/// The possible directions of text.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TextDirection {
    ltr, // left to right default
    rtl, // right to left
    ttb, // top to bottom
    btt, // bottom to top
}

/// Text macro parameters
///
/// There are numerous parameters that can be passed to the text macro. This struct
/// can be passed instead for convenience.
#[derive(Clone)]
pub struct TextParams {
    pub text: String,
    pub size: f64,
    pub font: String,
    pub halign: TextHalign,
    pub valign: TextValign,
    pub spacing: f64,
    pub direction: TextDirection,
    pub language: String,
    pub script: String,
    pub fn_: Option<u64>,
}

impl Default for TextParams {
    fn default() -> Self {
        Self {
            text: Default::default(),
            size: 10.0,
            font: "Liberation Sans".to_string(),
            halign: TextHalign::left,
            valign: TextValign::baseline,
            spacing: 1.0,
            direction: TextDirection::ltr,
            language: "en".to_string(),
            script: "latin".to_string(),
            fn_: None,
        }
    }
}

/// Saves Scad objects to a file.
///
/// Allows setting global $fa, $fs, or $fn. $fn overrides $fa and
/// $fs so cannot be specified with $fa or $fs.
///
/// #params
///
/// path: The path of the file to save.
///
/// fa: The minimum angle between segments or faces.
///
/// fs: The minimum length of a segment or face.
///
/// fn: The number of segments or faces in a circle.
///
/// children: A list of one or more Scad objects separated and terminated with a semicolon.
///
/// #patterns
///
/// scad_file!('path: &str', 'children: Scad';);
///
/// scad_file!('path: &str', fa='fa: f64', 'children: Scad';);
///
/// scad_file!('path: &str', fs='fs: f64', 'children: Scad';);
///
/// scad_file!('path: &str', fa='fa: f64', fs='fs: f64', 'children: Scad';);
///
/// scad_file!('path: &str', fn='fn: u64', 'children: Scad';);
#[macro_export]
macro_rules! scad_file {
    ($path:expr, fa=$fa:expr, fs=$fs:expr, $($child:expr);+;) => {
        let t = fat_thread!({
            let children = vec![$($child,)+];
            let mut file = std::fs::File::create($path).unwrap();
            file.write(format!("$fa={};\n", $fa).as_bytes()).unwrap();
            file.write(format!("$fs={};\n", $fs).as_bytes()).unwrap();
            for child in children {
                let s = format!("{}", child);
                file.write(s.as_bytes()).unwrap();
            }
            file.flush().unwrap();
        });
        t.join().unwrap();
    };
    ($path:expr, fn=$fn:expr, $($child:expr);+;) => {
        let t = fat_thread!({
            let children = vec![$($child,)+];
            let mut file = std::fs::File::create($path).unwrap();
            file.write(format!("$fn={};\n", $fn).as_bytes()).unwrap();
            for child in children {
                let s = format!("{}", child);
                file.write(s.as_bytes()).unwrap();
            }
            file.flush().unwrap();
        });
        t.join().unwrap();
    };
    ($path:expr, fs=$fs:expr, $($child:expr);+;) => {
        let t = fat_thread!({
            let children = vec![$($child,)+];
            let mut file = std::fs::File::create($path).unwrap();
            file.write(format!("$fs={};\n", $fs).as_bytes()).unwrap();
            for child in children {
                let s = format!("{}", child);
                file.write(s.as_bytes()).unwrap();
            }
            file.flush().unwrap();
        });
        t.join().unwrap();
    };
    ($path:expr, fa=$fa:expr, $($child:expr);+;) => {
        let t = fat_thread!({
            let children = vec![$($child,)+];
            let mut file = std::fs::File::create($path).unwrap();
            file.write(format!("$fa={};\n", $fa).as_bytes()).unwrap();
            for child in children {
                let s = format!("{}", child);
                file.write(s.as_bytes()).unwrap();
            }
            file.flush().unwrap();
        });
        t.join().unwrap();
    };
    ($path:expr, $($child:expr);+;) => {
        let t = fat_thread!({
            let children = vec![$($child,)+];
            let mut file = std::fs::File::create($path).unwrap();
            for child in children {
                let s = format!("{}", child);
                file.write(s.as_bytes()).unwrap();
            }
            file.flush().unwrap();
        });
        t.join().unwrap();
    };
}

/// Constructive Solid Geometry union operation.
///
/// Combines multiple shapes into one.
///
/// #params
///
/// Scad structs seperated by and ending with a seimicolon.
#[macro_export]
macro_rules! union {
    ($($child:expr);+;) => {
        Scad {
            op: ScadOp::Union,
            children: vec![$($child,)+],
        }
    };
}

/// Constructive Solid Geometry difference operation.
///
/// Subracts all subsequent shapes from the first shape.
///
/// #params
///
/// Scad structs seperated by and ending with a seimicolon.
#[macro_export]
macro_rules! difference {
  ($($child:expr);+;) => {
    Scad {
      op: ScadOp::Difference,
      children: vec![$($child,)+],
    }
  };
}

/// Constructive Solid Geometry intersection operation.
///
/// Yields the overlapping area of the given shapes.
///
/// #params
///
/// Scad structs seperated by and ending with a seimicolon.
#[macro_export]
macro_rules! intersection {
    ($($child:expr);+;) => {
        Scad {
            op: ScadOp::Intersection,
            children: vec![$($child,)+],
        }
    };
}

/// Creates a circle.
///
/// #params
///
/// diameter: The diameter of the circle.
///
/// radius: The radius of the circle.
///
/// fa: The minimum angle between segments.
///
/// fs: The minimum length of a segment.
///
/// fn: The number of segments in the circle.
///
/// expansion: Scad struct literal.
///
/// #patterns
///
/// circle!('radius: f64');
///
/// circle!('radius: f64', fn='fn: u64');
///
/// circle!('radius: f64', fa='fa: f64');
///
/// circle!('radius: f64', fs='fs: f64');
///
/// circle!('radius: f64', fa='fa: f64', fs='fs: f64');
///
/// circle!(d='diameter: f64');
///
/// circle!(d='diameter: f64', fn='fn: u64');
///
/// circle!(d='diameter: f64', fa='fa: f64');
///
/// circle!(d='diameter: f64', fs='fs: f64');
///
/// circle!(d='diameter: f64', fa='fa: f64', fs='fs: f64');
///
/// circle!(r='radius: f64');
///
/// circle!(r='radius: f64', fn='fn: u64');
///
/// circle!(r='radius: f64', fa='fa: f64');
///
/// circle!(r='radius: f64', fs='fs: f64');
///
/// circle!(r='radius: f64', fa='fa: f64', fs='fs: f64');
#[macro_export]
macro_rules! circle {
    (d=$dia:expr) => {
        Scad {
            op: ScadOp::Circle {
                radius: $dia / 2.0,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (d=$dia:expr, fn=$fn:expr) => {
        Scad {
            op: ScadOp::Circle {
                radius: $dia / 2.0,
                fa: None,
                fs: None,
                fn_: Some($fn),
            },
            children: Vec::new(),
        }
    };
    (d=$dia:expr, fa=$fa:expr) => {
        Scad {
            op: ScadOp::Circle {
                radius: $dia / 2.0,
                fa: Some($fa),
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (d=$dia:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Circle {
                radius: $dia / 2.0,
                fa: None,
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (d=$dia:expr, fa=$fa:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Circle {
                radius: $dia / 2.0,
                fa: Some($fa),
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (r=$r:expr) => {
        Scad {
            op: ScadOp::Circle {
                radius: $r,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (r=$r:expr, fn=$fn:expr) => {
        Scad {
            op: ScadOp::Circle {
                radius: $r,
                fa: None,
                fs: None,
                fn_: Some($fn),
            },
            children: Vec::new(),
        }
    };
    (r=$r:expr, fa=$fa:expr) => {
        Scad {
            op: ScadOp::Circle {
                radius: $r,
                fa: Some($fa),
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (r=$r:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Circle {
                radius: $r,
                fa: None,
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (r=$r:expr, fa=$fa:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Circle {
                radius: $r,
                fa: Some($fa),
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($r:expr, fa=$fa:expr) => {
        Scad {
            op: ScadOp::Circle {
                radius: $r,
                fa: Some($fa),
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($r:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Circle {
                radius: $r,
                fa: None,
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($r:expr, fa=$fa:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Circle {
                radius: $r,
                fa: Some($fa),
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($r:expr) => {
        Scad {
            op: ScadOp::Circle {
                radius: $r,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($r:expr, fn=$fn:expr) => {
        Scad {
            op: ScadOp::Circle {
                radius: $r,
                fa: None,
                fs: None,
                fn_: Some($fn),
            },
            children: Vec::new(),
        }
    };
}

/// Creates a square or rectangle.
///
/// #params
///
/// x: The x dimensions.
///
/// y: The y dimensions.
///
/// size: The size of a side for a square.
///
/// center: Whether to center the square or leave it in the 1st quadrant.
///
/// expansion: Scad struct literal.
///
/// #patterns
///
/// square!(\['x: f64', 'y: f64'\]);
///
/// square!(\['x: f64', 'y: f64'\], 'center: bool');
///
/// square!('size: f64');
///
/// square!('size: f64, 'center: bool');
#[macro_export]
macro_rules! square {
    ([$x:expr, $y:expr]) => {
        Scad {
            op: ScadOp::Square {
                size: Pt2::new($x, $y),
                center: false,
            },
            children: Vec::new(),
        }
    };
    ([$x:expr, $y:expr], $center:expr) => {
        Scad {
            op: ScadOp::Square {
                size: Pt2::new($x, $y),
                center: $center,
            },
            children: Vec::new(),
        }
    };
    ($size:expr) => {
        Scad {
            op: ScadOp::Square {
                size: Pt2::new($size, $size),
                center: false,
            },
            children: Vec::new(),
        }
    };
    ($size:expr, $center:expr) => {
        Scad {
            op: ScadOp::Square {
                size: Pt2::new($size, $size),
                center: $center,
            },
            children: Vec::new(),
        }
    };
}

/// Creates a polygon.
///
/// #params
///
/// points: The points that make up the polygon.
///
/// paths: The order of the points.
///
/// convexity: Number of inward curves, only for the preview.
///
/// expansion: The Scad struct literal.
///
/// #patterns
///
/// polygon!('points: Pt2s');
///
/// polygon!('points: Pt2s', 'paths: Paths');
///
/// polygon!('points: Pt2s', 'paths: Paths', 'convexity: u64');
///
/// polygon!('points: Pt2s', convexity='convexity: u64');
#[macro_export]
macro_rules! polygon {
    ($points:expr, convexity=$convexity:expr) => {
        Scad {
            op: ScadOp::Polygon {
                points: $points,
                paths: None,
                convexity: $convexity,
            },
            children: Vec::new(),
        }
    };
    ($points:expr, $paths:expr, $convexity:expr) => {
        Scad {
            op: ScadOp::Polygon {
                points: $points,
                paths: Some($paths),
                convexity: $convexity,
            },
            children: Vec::new(),
        }
    };
    ($points:expr, $paths:expr) => {
        Scad {
            op: ScadOp::Polygon {
                points: $points,
                paths: Some($paths),
                convexity: 1,
            },
            children: Vec::new(),
        }
    };
    ($points:expr) => {
        Scad {
            op: ScadOp::Polygon {
                points: $points,
                paths: None,
                convexity: 1,
            },
            children: Vec::new(),
        }
    };
}

/// Creates text.
///
/// #params
///
/// text: The text to display.
///
/// size: The size of the text.
///
/// font: The font for the text.
///
/// halign: Horizontal alignment of text.
///
/// valign: Vertical alignment of text.
///
/// spacing: The space between characters.
///
/// language: The language for the text "en" default.
///
/// script: The script for the text "latin" default.
///
/// fn: The number of segments in a circle.
///
/// text_params: A TextParams struct with the above members.
///
/// expansion: Scad struct literal.
///
/// #patterns
///
/// text!('text: &str');
///
/// text!(text_params='text_params: TextParams');
///
/// text!('text: &str', 'size: f64');
///
/// text!('text: &str', 'size: f64', 'font: &str');
///
/// text!('text: &str', fn='fn: u64');
///
/// text!('text: &str', 'size: f64', fn='fn: u64');
///
/// text!('text: &str', 'size: f64', 'font: &str', fn='fn: u64');
///
/// text!('text: &str', 'size: f64', 'font: &str', 'halign: TextHalign', 'valign: TextValign', 'direction: TextDirection');
///
/// text!('text: &str', 'size: f64', 'font: &str', 'halign: TextHalign', 'valign: TextValign', 'direction: TextDirection', fn='fn: u64');
///
/// text!('text: &str', 'size: f64', 'font: &str', 'halign: TextHalign', 'valign: TextValign', 'spacing: f64', 'direction: TextDirection', 'language: &str', 'script: &str', 'fn: u64');
#[macro_export]
macro_rules! text {
    (text_params=$params:expr) => {
        Scad {
            op: ScadOp::Text {
                text: $params.text,
                size: $params.size,
                font: $params.font,
                halign: $params.halign,
                valign: $params.valign,
                spacing: $params.spacing,
                direction: $params.direction,
                language: $params.language,
                script: $params.script,
                fn_: $params.fn_,
            },
            children: Vec::new(),
        }
    };
    ($text:expr, $size:expr, $font:expr, $halign:expr, $valign:expr, $spacing:expr, $direction:expr, $language:expr, $script:expr, $fn:expr) => {
        Scad {
            op: ScadOp::Text {
                text: $text.to_string(),
                size: $size,
                font: $font.to_string(),
                halign: $halign,
                valign: $valign,
                spacing: $spacing,
                direction: $direction,
                language: $language.to_string(),
                script: $script.to_string(),
                fn_: Some($fn),
            },
            children: Vec::new(),
        }
    };
    ($text:expr, $size:expr, $font:expr, $halign:expr, $valign:expr, $direction:expr, fn=$fn:expr) => {
        Scad {
            op: ScadOp::Text {
                text: $text.to_string(),
                size: $size,
                font: $font.to_string(),
                halign: $halign,
                valign: $valign,
                spacing: 1.0,
                direction: $direction,
                language: "en".to_string(),
                script: "latin".to_string(),
                fn_: Some($fn),
            },
            children: Vec::new(),
        }
    };
    ($text:expr, $size:expr, $font:expr, fn=$fn:expr) => {
        Scad {
            op: ScadOp::Text {
                text: $text.to_string(),
                size: $size,
                font: $font.to_string(),
                halign: TextHalign::left,
                valign: TextValign::baseline,
                spacing: 1.0,
                direction: TextDirection::ltr,
                language: "en".to_string(),
                script: "latin".to_string(),
                fn_: Some($fn),
            },
            children: Vec::new(),
        }
    };
    ($text:expr, $size:expr, fn=$fn:expr) => {
        Scad {
            op: ScadOp::Text {
                text: $text.to_string(),
                size: $size,
                font: "Liberation Sans".to_string(),
                halign: TextHalign::left,
                valign: TextValign::baseline,
                spacing: 1.0,
                direction: TextDirection::ltr,
                language: "en".to_string(),
                script: "latin".to_string(),
                fn_: Some($fn),
            },
            children: Vec::new(),
        }
    };
    ($text:expr, fn=$fn:expr) => {
        Scad {
            op: ScadOp::Text {
                text: $text.to_string(),
                size: 10.0,
                font: "Liberation Sans".to_string(),
                halign: TextHalign::left,
                valign: TextValign::baseline,
                spacing: 1.0,
                direction: TextDirection::ltr,
                language: "en".to_string(),
                script: "latin".to_string(),
                fn_: Some($fn),
            },
            children: Vec::new(),
        }
    };
    ($text:expr, $size:expr, $font:expr, $halign:expr, $valign:expr, $direction:expr) => {
        Scad {
            op: ScadOp::Text {
                text: $text.to_string(),
                size: $size,
                font: $font.to_string(),
                halign: $halign,
                valign: $valign,
                spacing: 1.0,
                direction: $direction,
                language: "en".to_string(),
                script: "latin".to_string(),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($text:expr, $size:expr, $font:expr) => {
        Scad {
            op: ScadOp::Text {
                text: $text.to_string(),
                size: $size,
                font: $font.to_string(),
                halign: TextHalign::left,
                valign: TextValign::baseline,
                spacing: 1.0,
                direction: TextDirection::ltr,
                language: "en".to_string(),
                script: "latin".to_string(),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($text:expr, $size:expr) => {
        Scad {
            op: ScadOp::Text {
                text: $text.to_string(),
                size: $size,
                font: "Liberation Sans".to_string(),
                halign: TextHalign::left,
                valign: TextValign::baseline,
                spacing: 1.0,
                direction: TextDirection::ltr,
                language: "en".to_string(),
                script: "latin".to_string(),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($text:expr) => {
        Scad {
            op: ScadOp::Text {
                text: $text.to_string(),
                size: 10.0,
                font: "Liberation Sans".to_string(),
                halign: TextHalign::left,
                valign: TextValign::baseline,
                spacing: 1.0,
                direction: TextDirection::ltr,
                language: "en".to_string(),
                script: "latin".to_string(),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
}

/// Import a file for use in OpenSCAD
///
/// #params
///
/// file: The path of the file to import.
///
/// convexity: Number of outside faces a ray could encouter when passing through object. Preview only.
///
/// expansion: Scad struct literal.
///
/// #patterns
///
/// import!('file: &str');
///
/// import!('file: &str', 'convexity: u64');
#[macro_export]
macro_rules! import {
    ($file:expr) => {
        Scad {
            op: ScadOp::Import {
                file: $file.to_string(),
                convexity: 1,
            },
            children: Vec::new(),
        }
    };
    ($file:expr, $convexity:expr) => {
        Scad {
            op: ScadOp::Import {
                file: $file.to_string(),
                convexity: $convexity,
            },
            children: Vec::new(),
        }
    };
}

/// Create a 2D projection of a 3D object.
///
/// #params
///
/// cut: When true the 2D shape is the part of the 3D object that is on the xy plane.
/// When false the 2D shape is the 'shadow' of the 3D object on the xy plane.
///
/// children: A list of 1 or more Scad structs separated by and ending with a semicolon.
///
/// expansion: Scad struct literal
///
/// #patterns
///
/// projection!('child: Scad'; ...;);
///
/// projection!(cut='cut: bool', 'child: Scad'; ...;);
#[macro_export]
macro_rules! projection {
  (cut=$cut:expr, $($child:expr);+;) => {
    Scad {
      op: ScadOp::Projection { cut: $cut },
      children: vec![$($child,)+],
    }
  };
  ($($child:expr);+;) => {
    Scad {
      op: ScadOp::Projection { cut: false },
      children: vec![$($child,)+],
    }
  };
}

/// Creates a sphere.
///
/// #params
///
/// diameter: The diameter of the sphere.
///
/// radius: The radius of the sphere.
///
/// fa: The minimum angle between segments.
///
/// fs: The minimum length of a segment.
///
/// fn: The number of segments in the circle.
///
/// expansion: Scad struct literal.
///
/// #patterns
///
/// sphere!('radius: f64');
///
/// sphere!('radius: f64', fn='fn: u64');
///
/// sphere!('radius: f64', fa='fa: f64');
///
/// sphere!('radius: f64', fs='fs: f64');
///
/// sphere!('radius: f64', fa='fa: f64', fs='fs: f64');
///
/// sphere!(d='diameter: f64');
///
/// sphere!(d='diameter: f64', fn='fn: u64');
///
/// sphere!(d='diameter: f64', fa='fa: f64');
///
/// sphere!(d='diameter: f64', fs='fs: f64');
///
/// sphere!(d='diameter: f64', fa='fa: f64', fs='fs: f64');
///
/// sphere!(r='radius: f64');
///
/// sphere!(r='radius: f64', fn='fn: u64');
///
/// sphere!(r='radius: f64', fa='fa: f64');
///
/// sphere!(r='radius: f64', fs='fs: f64');
///
/// sphere!(r='radius: f64', fa='fa: f64', fs='fs: f64');
#[macro_export]
macro_rules! sphere {
    (d=$dia:expr) => {
        Scad {
            op: ScadOp::Sphere {
                radius: $dia / 2.0,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (d=$dia:expr, fn=$fn:expr) => {
        Scad {
            op: ScadOp::Sphere {
                radius: $dia / 2.0,
                fa: None,
                fs: None,
                fn_: Some($fn),
            },
            children: Vec::new(),
        }
    };
    (d=$dia:expr, fa=$fa:expr) => {
        Scad {
            op: ScadOp::Sphere {
                radius: $dia / 2.0,
                fa: Some($fa),
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (d=$dia:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Sphere {
                radius: $dia / 2.0,
                fa: None,
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (d=$dia:expr, fa=$fa:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Sphere {
                radius: $dia / 2.0,
                fa: Some($fa),
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (r=$r:expr) => {
        Scad {
            op: ScadOp::Sphere {
                radius: $r,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (r=$r:expr, fn=$fn:expr) => {
        Scad {
            op: ScadOp::Sphere {
                radius: $r,
                fa: None,
                fs: None,
                fn_: Some($fn),
            },
            children: Vec::new(),
        }
    };
    (r=$r:expr, fa=$fa:expr) => {
        Scad {
            op: ScadOp::Sphere {
                radius: $r,
                fa: Some($fa),
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (r=$r:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Sphere {
                radius: $r,
                fa: None,
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (r=$r:expr, fa=$fa:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Sphere {
                radius: $r,
                fa: Some($fa),
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($r:expr, fa=$fa:expr) => {
        Scad {
            op: ScadOp::Sphere {
                radius: $r,
                fa: Some($fa),
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($r:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Sphere {
                radius: $r,
                fa: None,
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($r:expr, fa=$fa:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Sphere {
                radius: $r,
                fa: Some($fa),
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($r:expr) => {
        Scad {
            op: ScadOp::Sphere {
                radius: $r,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($r:expr, fn=$fn:expr) => {
        Scad {
            op: ScadOp::Sphere {
                radius: $r,
                fa: None,
                fs: None,
                fn_: Some($fn),
            },
            children: Vec::new(),
        }
    };
}

/// Create a cube.
///
/// #params
///
/// size: The size of a side of the cube.
///
/// center: Whether to center the cube or leave in the first octant.
///
/// [x, y, z]: The dimensions of the cube.
///
/// expansion: Scad struct literal.
///
/// #patterns
///
/// cube!('size: f64');
///
/// cube!('size: f64', 'center: bool');
///
/// cube!(\['x: f64', 'y: f64', 'z: f64'\]);
///
/// cube!(\['x: f64', 'y: f64', 'z: f64'\], 'center: bool');
#[macro_export]
macro_rules! cube {
    ([$x:expr, $y:expr, $z:expr], $center:expr) => {
        Scad {
            op: ScadOp::Cube {
                size: Pt3::new($x, $y, $z),
                center: $center,
            },
            children: Vec::new(),
        }
    };
    ([$x:expr, $y:expr, $z:expr]) => {
        Scad {
            op: ScadOp::Cube {
                size: Pt3::new($x, $y, $z),
                center: false,
            },
            children: Vec::new(),
        }
    };
    ($size:expr, $center:expr) => {
        Scad {
            op: ScadOp::Cube {
                size: Pt3::new($size, $size, $size),
                center: $center,
            },
            children: Vec::new(),
        }
    };
    ($size:expr) => {
        Scad {
            op: ScadOp::Cube {
                size: Pt3::new($size, $size, $size),
                center: false,
            },
            children: Vec::new(),
        }
    };
}

/// Creates a cylinder.
///
/// #params
///
/// height: The height of the cylinder.
///
/// radius: The radius of the cylinder.
///
/// radius1: The radius at the bottom.
///
/// radius2: The radius at the top.
///
/// diameter: The diameter of the cylinder.
///
/// diameter1: The diameter at the bottom.
///
/// diameter2: The diameter at the top.
///
/// center: When true the cylinder is centered at the world origin. When false the
/// cylinder 'sits' on the world origin.
///
/// fa: The minimum angle between segments.
///
/// fs: The minimum length of a segment.
///
/// fn: The number of segments in the cylinder.
///
/// expansion: Scad struct literal.
///
/// #patterns
///
/// cylinder!('height: f64', 'radius: f64')
///
/// cylinder!('height: f64', 'radius1: f64', 'radius2: f64')
///
/// cylinder!('height: f64', 'radius1: f64', 'radius2: f64', 'center: bool')
///
/// cylinder!('height: f64', 'radius1: f64', 'radius2: f64', 'center: bool', fa='fa: f64')
///
/// cylinder!('height: f64', 'radius1: f64', 'radius2: f64', 'center: bool', fs='fs: f64')
///
/// cylinder!('height: f64', 'radius1: f64', 'radius2: f64', 'center: bool', fa='fa: f64', fs='fs: f64')
///
/// cylinder!('height: f64', 'radius1: f64', 'radius2: f64', 'center: bool', fn='fn: u64')
///
/// cylinder!('height: f64', d='diameter: f64')
///
/// cylinder!('height: f64', d1='diameter1: f64', d2='diameter2: f64')
///
/// cylinder!('height: f64', d1='diameter1: f64', d2='diameter2: f64', center='center: bool')
///
/// cylinder!('height: f64', d1='diameter1: f64', d2='diameter2: f64', center='center: bool', fa='fa: f64')
///
/// cylinder!('height: f64', d1='diameter1: f64', d2='diameter2: f64', center='center: bool', fs='fs: f64')
///
/// cylinder!('height: f64', d1='diameter1: f64', d2='diameter2: f64', center='center: bool', fa='fa: f64', fs='fs: f64')
///
/// cylinder!('height: f64', d1='diameter1: f64', d2='diameter2: f64', center='center: bool', fn='fn: u64')
///
/// cylinder!(h='height: f64', r='radius: f64')
///
/// cylinder!(h='height: f64', r1='radius1: f64', r2='radius2: f64')
///
/// cylinder!(h='height: f64', r1='radius1: f64', r2='radius2: f64', center='center: bool')
///
/// cylinder!(h='height: f64', r1='radius1: f64', r2='radius2: f64', center='center: bool', fa='fa: f64')
///
/// cylinder!(h='height: f64', r1='radius1: f64', r2='radius2: f64', center='center: bool', fs='fs: f64')
///
/// cylinder!(h='height: f64', r1='radius1: f64', r2='radius2: f64', center='center: bool', fa='fa: f64', fs='fs: f64')
///
/// cylinder!(h='height: f64', r1='radius1: f64', r2='radius2: f64', center='center: bool', fn='fn: u64')
///
/// cylinder!(h='height: f64', d='diameter: f64')
///
/// cylinder!(h='height: f64', d1='diameter1: f64', d2='diameter2: f64')
///
/// cylinder!(h='height: f64', d1='diameter1: f64', d2='diameter2: f64', center='center: bool')
///
/// cylinder!(h='height: f64', d1='diameter1: f64', d2='diameter2: f64', center='center: bool', fa='fa: f64')
///
/// cylinder!(h='height: f64', d1='diameter1: f64', d2='diameter2: f64', center='center: bool', fs='fs: f64')
///
/// cylinder!(h='height: f64', d1='diameter1: f64', d2='diameter2: f64', center='center: bool', fa='fa: f64', fs='fs: f64')
///
/// cylinder!(h='height: f64', d1='diameter1: f64', d2='diameter2: f64', center='center: bool', fn='fn: u64')
#[macro_export]
macro_rules! cylinder {
    (h=$height:expr, d1=$diameter1:expr, d2=$diameter2:expr, center=$center:expr, fa=$fa:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $diameter1 / 2.0,
                radius2: $diameter2 / 2.0,
                center: $center,
                fa: Some($fa),
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (h=$height:expr, d1=$diameter1:expr, d2=$diameter2:expr, center=$center:expr, fa=$fa:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $diameter1 / 2.0,
                radius2: $diameter2 / 2.0,
                center: $center,
                fa: Some($fa),
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (h=$height:expr, d1=$diameter1:expr, d2=$diameter2:expr, center=$center:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $diameter1 / 2.0,
                radius2: $diameter2 / 2.0,
                center: $center,
                fa: None,
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (h=$height:expr, d1=$diameter1:expr, d2=$diameter2:expr, center=$center:expr, fn=$fn:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $diameter1 / 2.0,
                radius2: $diameter2 / 2.0,
                center: $center,
                fa: None,
                fs: None,
                fn_: Some($fn),
            },
            children: Vec::new(),
        }
    };
    (h=$height:expr, d1=$diameter1:expr, d2=$diameter2:expr, center=$center:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $diameter1 / 2.0,
                radius2: $diameter2 / 2.0,
                center: $center,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (h=$height:expr, d1=$diameter1:expr, d2=$diameter2:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $diameter1 / 2.0,
                radius2: $diameter2 / 2.0,
                center: false,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (h=$height:expr, d=$diameter:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $diameter / 2.0,
                radius2: $diameter / 2.0,
                center: false,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (h=$height:expr, r1=$radius1:expr, r2=$radius2:expr, center=$center:expr, fa=$fa:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $radius1,
                radius2: $radius2,
                center: $center,
                fa: Some($fa),
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (h=$height:expr, r1=$radius1:expr, r2=$radius2:expr, center=$center:expr, fa=$fa:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $radius1,
                radius2: $radius2,
                center: $center,
                fa: Some($fa),
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (h=$height:expr, r1=$radius1:expr, r2=$radius2:expr, center=$center:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $radius1,
                radius2: $radius2,
                center: $center,
                fa: None,
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (h=$height:expr, r1=$radius1:expr, r2=$radius2:expr, center=$center:expr, fn=$fn:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $radius1,
                radius2: $radius2,
                center: $center,
                fa: None,
                fs: None,
                fn_: Some($fn),
            },
            children: Vec::new(),
        }
    };
    (h=$height:expr, r1=$radius1:expr, r2=$radius2:expr, center=$center:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $radius1,
                radius2: $radius2,
                center: $center,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (h=$height:expr, r1=$radius1:expr, r2=$radius2:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $radius1,
                radius2: $radius2,
                center: false,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    (h=$height:expr, r=$radius:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $radius,
                radius2: $radius,
                center: false,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($height:expr, d1=$diameter1:expr, d2=$diameter2:expr, center=$center:expr, fa=$fa:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $diameter1 / 2.0,
                radius2: $diameter2 / 2.0,
                center: $center,
                fa: Some($fa),
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($height:expr, d1=$diameter1:expr, d2=$diameter2:expr, center=$center:expr, fa=$fa:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $diameter1 / 2.0,
                radius2: $diameter2 / 2.0,
                center: $center,
                fa: Some($fa),
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($height:expr, d1=$diameter1:expr, d2=$diameter2:expr, center=$center:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $diameter1 / 2.0,
                radius2: $diameter2 / 2.0,
                center: $center,
                fa: None,
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($height:expr, d1=$diameter1:expr, d2=$diameter2:expr, center=$center:expr, fn=$fn:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $diameter1 / 2.0,
                radius2: $diameter2 / 2.0,
                center: $center,
                fa: None,
                fs: None,
                fn_: Some($fn),
            },
            children: Vec::new(),
        }
    };
    ($height:expr, d1=$diameter1:expr, d2=$diameter2:expr, center=$center:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $diameter1 / 2.0,
                radius2: $diameter2 / 2.0,
                center: $center,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($height:expr, d1=$diameter1:expr, d2=$diameter2:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $diameter1 / 2.0,
                radius2: $diameter2 / 2.0,
                center: false,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($height:expr, d=$diameter:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $diameter / 2.0,
                radius2: $diameter / 2.0,
                center: false,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($height:expr, $radius1:expr, $radius2:expr, $center:expr, fa=$fa:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $radius1,
                radius2: $radius2,
                center: $center,
                fa: Some($fa),
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($height:expr, $radius1:expr, $radius2:expr, $center:expr, fa=$fa:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $radius1,
                radius2: $radius2,
                center: $center,
                fa: Some($fa),
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($height:expr, $radius1:expr, $radius2:expr, $center:expr, fs=$fs:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $radius1,
                radius2: $radius2,
                center: $center,
                fa: None,
                fs: Some($fs),
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($height:expr, $radius1:expr, $radius2:expr, $center:expr, fn=$fn:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $radius1,
                radius2: $radius2,
                center: $center,
                fa: None,
                fs: None,
                fn_: Some($fn),
            },
            children: Vec::new(),
        }
    };
    ($height:expr, $radius1:expr, $radius2:expr, $center:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $radius1,
                radius2: $radius2,
                center: $center,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($height:expr, $radius1:expr, $radius2:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $radius1,
                radius2: $radius2,
                center: false,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
    ($height:expr, $radius:expr) => {
        Scad {
            op: ScadOp::Cylinder {
                height: $height,
                radius1: $radius,
                radius2: $radius,
                center: false,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: Vec::new(),
        }
    };
}

/// Creates a polyhedron.
///
/// #params
///
/// points: The vertices of the polyhedron.
///
/// faces: A list of lists of indices into points.
///
/// convexity: The number of outside faces a ray intersecting the polyhedron might encounter. Preview only.
///
/// expansion: A Scad struct literal.
///
/// #patterns
///
/// polyhedron!('points: Pt3s', 'faces: Faces');
///
/// polyhedron!('points: Pt3s', 'faces: Faces', 'convexity: u64');
///
/// polyhedron!(points='points: Pt3s', faces='faces: Faces', convexity='convexity: u64');
#[macro_export]
macro_rules! polyhedron {
    (points=$points:expr, faces=$faces:expr, convexity=$convexity:expr) => {
        Scad {
            op: ScadOp::Polyhedron {
                points: $points,
                faces: $faces,
                convexity: $convexity,
            },
            children: Vec::new(),
        }
    };
    ($points:expr, $faces:expr, $convexity:expr) => {
        Scad {
            op: ScadOp::Polyhedron {
                points: $points,
                faces: $faces,
                convexity: $convexity,
            },
            children: Vec::new(),
        }
    };
    ($points:expr, $faces:expr) => {
        Scad {
            op: ScadOp::Polyhedron {
                points: $points,
                faces: $faces,
                convexity: 1,
            },
            children: Vec::new(),
        }
    };
}

/// Extrude a 2D profile along the Z axis creating a 3D shape.
///
/// #params
///
/// height: The height of the extrusion.
///
/// center: Centered on world origin or 'sitting' on world origin.
///
/// convexity: The number of outside faces a ray might encounter. Preview only.
///
/// twist: Degrees of rotation along the extrusion.
///
/// scale: Scale at the end of the extrusion. May have separate X and Y coordinates e.g. [1.5, 0.5].
///
/// slices: Resolution of the extrusion. Seems to have the same effect as $fn.
///
/// fn: Same as slices.
///
/// children: A list of Scad objects to apply the extrusion to. Separated and ending with a semicolon.
///
/// expansion: A Scad struct literal.
///
/// #patterns
///
/// linear_extrude!('height: f64', 'children: Scad';);
///
/// linear_extrude!(height='height: f64', center='center: bool', convexity='convexity: u64', twist='twist: f64', scale='scale: f64', slices='slices: u64', 'children: Scad';);
///
/// linear_extrude!(height='height: f64', center='center: bool', convexity='convexity: u64', twist='twist: f64', scale=\['scale_x: f64', 'scale_y: f64'\], slices='slices: u64', 'children: Scad';);
///
/// linear_extrude!(height='height: f64', center='center: bool', convexity='convexity: u64', twist='twist: f64', scale='scale: f64', fn='fn: u64', 'children: Scad';);
///
/// linear_extrude!(height='height: f64', center='center: bool', convexity='convexity: u64', twist='twist: f64', scale=\['scale_x: f64', 'scale_y: f64'\], fn='fn: u64', 'children: Scad';);
#[macro_export]
macro_rules! linear_extrude {
    (height=$height:expr, center=$center:expr, convexity=$convexity:expr, twist=$twist:expr, scale=[$scale_x:expr, $scale_y:expr], fn=$fn:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::LinearExtrude {
                height: $height,
                center: $center,
                convexity: $convexity,
                twist: $twist,
                scale: Pt2::new($scale_x, $scale_y),
                slices: None,
                fn_: Some($fn),
            },
            children: vec![$($child,)+],
        }
    };
    (height=$height:expr, center=$center:expr, convexity=$convexity:expr, twist=$twist:expr, scale=[$scale_x:expr, $scale_y:expr], slices=$slices:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::LinearExtrude {
                height: $height,
                center: $center,
                convexity: $convexity,
                twist: $twist,
                scale: Pt2::new($scale_x, $scale_y),
                slices: Some($slices),
                fn_: None,
            },
            children: vec![$($child,)+],
        }
    };
    (height=$height:expr, center=$center:expr, convexity=$convexity:expr, twist=$twist:expr, scale=$scale:expr, fn=$fn:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::LinearExtrude {
                height: $height,
                center: $center,
                convexity: $convexity,
                twist: $twist,
                scale: Pt2::new($scale, $scale),
                slices: None,
                fn_: Some($fn),
            },
            children: vec![$($child,)+],
        }
    };
    (height=$height:expr, center=$center:expr, convexity=$convexity:expr, twist=$twist:expr, scale=$scale:expr, slices=$slices:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::LinearExtrude {
                height: $height,
                center: $center,
                convexity: $convexity,
                twist: $twist,
                scale: Pt2::new($scale, $scale),
                slices: Some($slices),
                fn_: None,
            },
            children: vec![$($child,)+],
        }
    };
    ($height:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::LinearExtrude {
                height: $height,
                center: false,
                convexity: 1,
                twist: 0.0,
                scale: Pt2::new(1.0,1.0),
                slices: None,
                fn_: None,
            },
            children: vec![$($child,)+],
        }
    };
}

/// Create a 3D shape by rotating a 2D profile around the Z axis.
///
/// #params
///
/// angle: The angle in degrees to extrude through.
///
/// convexity: The number of outside faces a ray could pass through when intersecting the extrusion. Preview only.
///
/// fa: The minimum angle between segments.
///
/// fs: The minimum length of a segment.
///
/// fn: The number of segments in the cylinder.
///
/// children: A list of Scad objects to apply the extrusion to. Separated and ending with a semicolon.
///
/// expansion: A Scad struct literal.
///
/// rotate_extrude!('children: Scad';);
///
/// rotate_extrude!(angle='angle: f64', 'children: Scad';);
///
/// rotate_extrude!(angle='angle: f64', convexity='convexity: u64', 'children: Scad';);
///
/// rotate_extrude!(angle='angle: f64', convexity='convexity: u64', fa='fa: f64', 'children: Scad';);
///
/// rotate_extrude!(angle='angle: f64', convexity='convexity: u64', fs='fs: f64', 'children: Scad';);
///
/// rotate_extrude!(angle='angle: f64', convexity='convexity: u64', fa='fa: f64', fs='fs: f64', 'children: Scad';);

/// rotate_extrude!(angle='angle: f64', convexity='convexity: u64', fn='fn: f64', 'children: Scad';);
#[macro_export]
macro_rules! rotate_extrude {
    (angle=$angle:expr, convexity=$convexity:expr, fn=$fn:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::RotateExtrude {
                angle: $angle,
                convexity: $convexity,
                fa: None,
                fs: None,
                fn_: Some($fn),
            },
            children: vec![$($child,)+],
        }
    };
    (angle=$angle:expr, convexity=$convexity:expr, fa=$fa:expr, fs=$fs:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::RotateExtrude {
                angle: $angle,
                convexity: $convexity,
                fa: Some($fa),
                fs: Some($fs),
                fn_: None,
            },
            children: vec![$($child,)+],
        }
    };
    (angle=$angle:expr, convexity=$convexity:expr, fs=$fs:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::RotateExtrude {
                angle: $angle,
                convexity: $convexity,
                fa: None,
                fs: Some($fs),
                fn_: None,
            },
            children: vec![$($child,)+],
        }
    };
    (angle=$angle:expr, convexity=$convexity:expr, fa=$fa:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::RotateExtrude {
                angle: $angle,
                convexity: $convexity,
                fa: Some($fa),
                fs: None,
                fn_: None,
            },
            children: vec![$($child,)+],
        }
    };
    (angle=$angle:expr, convexity=$convexity:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::RotateExtrude {
                angle: $angle,
                convexity: $convexity,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: vec![$($child,)+],
        }
    };
    (angle=$angle:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::RotateExtrude {
                angle: $angle,
                convexity: 1,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: vec![$($child,)+],
        }
    };
    ($($child:expr);+;) => {
        Scad {
            op: ScadOp::RotateExtrude {
                angle: 360.0,
                convexity: 1,
                fa: None,
                fs: None,
                fn_: None,
            },
            children: vec![$($child,)+],
        }
    };
}

/// Loads a height map from a file.
///
/// #params
///
/// file: The path to the file to load.
///
/// center: Whether to center object or leave in first octant.
///
/// invert: Whether to invert the data.
///
/// convexity: The number of outside faces a ray could pass through when intersecting the object. Preview only.
///
/// expansion: A Scad struct literal.
///
/// #patterns
///
/// surface!('file: &str');
///
/// surface!(file='file: &str', center='center: bool', invert='invert: bool', convexity='convexity: u64');
#[macro_export]
macro_rules! surface {
    (file=$file:expr, center=$center:expr, invert=$invert:expr, convexity=$convexity:expr) => {
        Scad {
            op: ScadOp::Surface {
                file: $file.to_string(),
                center: $center,
                invert: $invert,
                convexity: $convexity,
            },
            children: Vec::new(),
        }
    };
    ($file:expr) => {
        Scad {
            op: ScadOp::Surface {
                file: $file.to_string(),
                center: false,
                invert: false,
                convexity: 1,
            },
            children: Vec::new(),
        }
    };
}

/// Translates children.
///
/// #params
///
/// v: The x, y, and z coordinates to translate by e.g. [1.0, 2.0, 3.0].
///
/// children: A list of Scad objects separated and ending with a semicolon.
///
/// expansion: A Scad struct literal.
///
/// #patterns:
///
/// translate!(\['x: f64', 'y: f64', 'z: f64'\], 'children: Scad';);
///
/// translate!(v=\['x: f64', 'y: f64', 'z: f64'\], 'children: Scad';);
#[macro_export]
macro_rules! translate {
    (v=[$x:expr, $y:expr, $z:expr], $($child:expr);+;) => {
        Scad {
            op: ScadOp::Translate {
                v: Pt3::new($x, $y, $z),
            },
            children: vec![$($child,)+],
        }
    };
    ([$x:expr, $y:expr, $z:expr], $($child:expr);+;) => {
        Scad {
            op: ScadOp::Translate {
                v: Pt3::new($x, $y, $z),
            },
            children: vec![$($child,)+],
        }
    };
}

/// Rotates children.
///
/// #params
///
/// a: Degrees of rotation around v when v is given else a vector of degrees for rotation around
/// the x, y, and z axis or a scalar for 2D rotations.
///
/// v: Axis to rotate around.
///
/// children: A list of Scad objects separated and ending with a semicolon.
///
/// expansion: A Scad struct literal.
///
/// #patterns
///
/// rotate!(\['x: f64', 'y: f64', 'z: f64'\], 'children: Scad';);
///
/// rotate!(a=\['x: f64', 'y: f64', 'z: f64'\], 'children: Scad';);
///
/// rotate!(a='z: f64', 'children: Scad';);
///
/// rotate!('z: f64', 'children: Scad';);
///
/// rotate!('a: f64, \['x: f64', 'y: f64', 'z: f64'\], 'children: Scad';);

/// rotate!(a='a: f64, v=\['x: f64', 'y: f64', 'z: f64'\], 'children: Scad';);
#[macro_export]
macro_rules! rotate {
    (a=$a:expr, v=[$x:expr, $y:expr, $z:expr], $($child:expr);+;) => {
        Scad {
            op: ScadOp::Rotate {
                a: Some($a),
                a_is_scalar: false,
                v: Pt3::new($x, $y, $z),
            },
            children: vec![$($child,)+],
        }
    };
    ($a:expr, [$x:expr, $y:expr, $z:expr], $($child:expr);+;) => {
        Scad {
            op: ScadOp::Rotate {
                a: Some($a),
                a_is_scalar: false,
                v: Pt3::new($x, $y, $z),
            },
            children: vec![$($child,)+],
        }
    };
    (a=[$x:expr, $y:expr, $z:expr], $($child:expr);+;) => {
        Scad {
            op: ScadOp::Rotate {
                a: None,
                a_is_scalar: false,
                v: Pt3::new($x, $y, $z),
            },
            children: vec![$($child,)+],
        }
    };
    ([$x:expr, $y:expr, $z:expr], $($child:expr);+;) => {
        Scad {
            op: ScadOp::Rotate {
                a: None,
                a_is_scalar: false,
                v: Pt3::new($x, $y, $z),
            },
            children: vec![$($child,)+],
        }
    };
    (a=$a:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::Rotate {
                a: Some($a),
                a_is_scalar: true,
                v: Pt3::new(0.0, 0.0, 0.0),
            },
            children: vec![$($child,)+],
        }
    };
    ($a:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::Rotate {
                a: Some($a),
                a_is_scalar: true,
                v: Pt3::new(0.0, 0.0, 0.0),
            },
            children: vec![$($child,)+],
        }
    };
}

/// Scale an object.
///
/// #params
///
/// v: Vector of scale factors for x, y, and z axis respectively.
///
/// children: A list of Scad objects separated and ending with a semicolon.
///
/// expansion: A Scad struct literal.
///
/// #patterns
///
/// scale!(\['x: f64', 'y: f64', 'z: f64'\], 'children: Scad';);
///
/// scale!(v=\['x: f64', 'y: f64', 'z: f64'\], 'children: Scad';);
#[macro_export]
macro_rules! scale {
    (v=[$x:expr, $y:expr, $z:expr], $($child:expr);+;) => {
        Scad {
            op: ScadOp::Scale {
                v: Pt3::new($x, $y, $z),
            },
            children: vec![$($child,)+],
        }
    };
    ([$x:expr, $y:expr, $z:expr], $($child:expr);+;) => {
        Scad {
            op: ScadOp::Scale {
                v: Pt3::new($x, $y, $z),
            },
            children: vec![$($child,)+],
        }
    };
}

/// Resize an object.
///
/// #params
///
/// newsize: The new x, y, and z dimensions.
///
/// auto: When true zero's in newsize will scale the proportionaltely to the given axis. Auto can be a vector
/// of three bools to exclude an axis from the auto.
///
/// convexity: The number of outside edges a ray might encounter when passing through the object.
///
/// children: A list of Scad objects separated and ending with a semicolon.
///
/// expansion: A Scad struct literal.
///
/// #patterns
///
/// resize!(\['x: f64', 'y: f64', 'z: f64'\], 'children: Scad';);
///
/// resize!(\['x: f64', 'y: f64', 'z: f64'\], 'auto: bool', 'children: Scad';);
///
/// resize!(\['x: f64', 'y: f64', 'z: f64'\], \['auto_x: bool', 'auto_y: bool', 'auto_z: bool'\], 'children: Scad';);
///
/// resize!(\['x: f64', 'y: f64', 'z: f64'\], 'auto: bool', 'convexity: u64', 'children: Scad';);
///
/// resize!(\['x: f64', 'y: f64', 'z: f64'\], \['auto_x: bool', 'auto_y: bool', 'auto_z: bool'\], 'convexity: u64', 'children: Scad';);
#[macro_export]
macro_rules! resize {
    (newsize=[$x:expr, $y:expr, $z:expr], auto=[$auto_x:expr, $auto_y:expr, $auto_z:expr], convexity=$convexity:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::Resize {
                newsize: Pt3::new($x, $y, $z),
                auto: false,
                auto_is_vec: true,
                autovec: ($auto_x, $auto_y, $auto_z),
                convexity: $convexity,
            },
            children: vec![$($child,)+],
        }
    };
    (newsize=[$x:expr, $y:expr, $z:expr], auto=$auto:expr, convexity=$convexity:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::Resize {
                newsize: Pt3::new($x, $y, $z),
                auto: $auto,
                auto_is_vec: false,
                autovec: (false, false, false),
                convexity: $convexity,
            },
            children: vec![$($child,)+],
        }
    };
    (newsize=[$x:expr, $y:expr, $z:expr], auto=[$auto_x:expr, $auto_y:expr, $auto_z:expr], $($child:expr);+;) => {
        Scad {
            op: ScadOp::Resize {
                newsize: Pt3::new($x, $y, $z),
                auto: false,
                auto_is_vec: true,
                autovec: ($auto_x, $auto_y, $auto_z),
                convexity: 1,
            },
            children: vec![$($child,)+],
        }
    };
    (newsize=[$x:expr, $y:expr, $z:expr], auto=$auto:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::Resize {
                newsize: Pt3::new($x, $y, $z),
                auto: $auto,
                auto_is_vec: false,
                autovec: (false, false, false),
                convexity: 1,
            },
            children: vec![$($child,)+],
        }
    };
    (newsize=[$x:expr, $y:expr, $z:expr], $($child:expr);+;) => {
        Scad {
            op: ScadOp::Resize {
                newsize: Pt3::new($x, $y, $z),
                auto: false,
                auto_is_vec: false,
                autovec: (false, false, false),
                convexity: 1,
            },
            children: vec![$($child,)+],
        }
    };
    ([$x:expr, $y:expr, $z:expr], [$auto_x:expr, $auto_y:expr, $auto_z:expr], $convexity:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::Resize {
                newsize: Pt3::new($x, $y, $z),
                auto: false,
                auto_is_vec: true,
                autovec: ($auto_x, $auto_y, $auto_z),
                convexity: $convexity,
            },
            children: vec![$($child,)+],
        }
    };
    ([$x:expr, $y:expr, $z:expr], $auto:expr, $convexity:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::Resize {
                newsize: Pt3::new($x, $y, $z),
                auto: $auto,
                auto_is_vec: false,
                autovec: (false, false, false),
                convexity: $convexity,
            },
            children: vec![$($child,)+],
        }
    };
    ([$x:expr, $y:expr, $z:expr], [$auto_x:expr, $auto_y:expr, $auto_z:expr], $($child:expr);+;) => {
        Scad {
            op: ScadOp::Resize {
                newsize: Pt3::new($x, $y, $z),
                auto: false,
                auto_is_vec: true,
                autovec: ($auto_x, $auto_y, $auto_z),
                convexity: 1,
            },
            children: vec![$($child,)+],
        }
    };
    ([$x:expr, $y:expr, $z:expr], $auto:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::Resize {
                newsize: Pt3::new($x, $y, $z),
                auto: $auto,
                auto_is_vec: false,
                autovec: (false, false, false),
                convexity: 1,
            },
            children: vec![$($child,)+],
        }
    };
    ([$x:expr, $y:expr, $z:expr], $($child:expr);+;) => {
        Scad {
            op: ScadOp::Resize {
                newsize: Pt3::new($x, $y, $z),
                auto: false,
                auto_is_vec: false,
                autovec: (false, false, false),
                convexity: 1,
            },
            children: vec![$($child,)+],
        }
    };
}

/// Mirror object(s).
///
/// #params
///
/// v: Normal of the mirror plane.
///
/// children: A list of Scad objects separated and ending with a semicolon.
///
/// expansion: A Scad struct literal.
///
/// #patterns
///
/// mirror!(\['x: f64', 'y: f64', 'z: f64'\], 'children: Scad';);
///
/// mirror!(v=\['x: f64', 'y: f64', 'z: f64'\], 'children: Scad';);
#[macro_export]
macro_rules! mirror {
    (v=[$x:expr, $y:expr, $z:expr], $($child:expr);+;) => {
        Scad {
            op: ScadOp::Mirror {
                v: Pt3::new($x, $y, $z),
            },
            children: vec![$($child,)+],
        }
    };
    ([$x:expr, $y:expr, $z:expr], $($child:expr);+;) => {
        Scad {
            op: ScadOp::Mirror {
                v: Pt3::new($x, $y, $z),
            },
            children: vec![$($child,)+],
        }
    };
}

/// Colors children.
///
/// #params
///
/// [r, g, b, a]: 4 floats between 0.0 and 1.0 for red, green, blue, and alpha.
///
/// "#rrggbbaa": &str hex code.
///
/// c: ScadColor enum member.
///
/// alpha: The alpha channel for c.
///
/// children: A list of Scad objects separated and ending with a semicolon.
///
/// expansion: A Scad struct literal.
///
/// #patterns
///
/// color!(\['r: f64', 'g: f64', 'b: f64', 'a: f64'\], 'children: Scad';);
///
/// color!('"#rrggbbaa": &str', 'children: Scad';);
///
/// color!(c='c: ScadColor', 'children: Scad';);
///
/// color!(c='c: ScadColor', alpha='alpha: f64', 'children: Scad';);
#[macro_export]
macro_rules! color {
    (c=$color:expr, alpha=$alpha:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::Color {
                rgba: None,
                color: Some($color),
                hex: None,
                alpha: Some($alpha),
            },
            children: vec![$($child,)+],
        }
    };
    (c=$color:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::Color {
                rgba: None,
                color: Some($color),
                hex: None,
                alpha: None,
            },
            children: vec![$($child,)+],
        }
    };
    ([$r:expr, $g:expr, $b:expr, $a:expr], $($child:expr);+;) => {
        Scad {
            op: ScadOp::Color {
                rgba: Some(Pt4::new($r, $g, $b, $a)),
                color: None,
                hex: None,
                alpha: None,
            },
            children: vec![$($child,)+],
        }
    };
    ($hex:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::Color {
                rgba: None,
                color: None,
                hex: Some($hex.to_string()),
                alpha: None,
            },
            children: vec![$($child,)+],
        }
    };
}

/// Offsets a 2D shape.
///
/// #params
///
/// r: The radius of the offset.
///
/// delta: Used instead of r for sharp corners.
///
/// chamfer: Whether to extend a corner to a point (false) or chamfer it (true).
///
/// children: A list of Scad objects separated and ending with a semicolon.
///
/// expansion: A Scad struct literal.
///
/// #patterns
///
/// offset!('r: f64', 'children: Scad';);
///
/// offset!(delta='delta: f64', chamfer='chamfer: bool', 'children: Scad';);
#[macro_export]
macro_rules! offset {
    (delta=$delta:expr, chamfer=$chamfer:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::Offset {
                r: None,
                delta: Some($delta),
                chamfer: $chamfer,
            },
            children: vec![$($child,)+],
        }
    };
    ($r:expr, $($child:expr);+;) => {
        Scad {
            op: ScadOp::Offset {
                r: Some($r),
                delta: None,
                chamfer: false,
            },
            children: vec![$($child,)+],
        }
    };
}

/// Constructive Solid Geometry hull operation.
///
/// Combines multiple shapes into one.
///
/// #params
///
/// Scad structs seperated by and ending with a seimicolon.
#[macro_export]
macro_rules! hull {
  ($($child:expr);+;) => {
    Scad {
        op: ScadOp::Hull,
        children: vec![$($child,)+],
    }
  };
}

/// Minkowski sum.
///
/// #params
///
/// convexity: The number of outside edges a ray might encounter when passing through the object.
///
/// children: A list of Scad objects separated and ending with a semicolon.
///
/// expansion: A Scad struct literal.
///
/// #patterns
///
/// minkowski!('children: Scad';);
///
/// minkowski!('convexity: u64', 'children: Scad';);
#[macro_export]
macro_rules! minkowski {
  ($convexity:expr, $($child:expr);+;) => {
    Scad {
        op: ScadOp::Minkowski {
            convexity: $convexity
        },
        children: vec![$($child,)+],
    }
  };
  ($($child:expr);+;) => {
    Scad {
        op: ScadOp::Minkowski {
            convexity:1
        },
        children: vec![$($child,)+],
    }
  };
}
/***********************************************************
* TESTING 1, 2, 3...
***********************************************************/

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn union_of_1() {
        let res = union!(circle!(1.0););
        assert!(
            res == Scad {
                op: ScadOp::Union,
                children: vec![Scad {
                    op: ScadOp::Circle {
                        radius: 1.0,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }],
            }
        );
    }

    #[test]
    fn union_of_2() {
        let res = union!(circle!(1.0);square!(1.0););
        assert!(
            res == Scad {
                op: ScadOp::Union,
                children: vec![
                    Scad {
                        op: ScadOp::Circle {
                            radius: 1.0,
                            fa: None,
                            fs: None,
                            fn_: None,
                        },
                        children: Vec::new(),
                    },
                    Scad {
                        op: ScadOp::Square {
                            size: Pt2::new(1.0, 1.0),
                            center: false,
                        },
                        children: Vec::new(),
                    }
                ],
            }
        );
    }

    #[test]
    fn difference_of_2() {
        let res = difference!(circle!(1.0);square!(1.0););
        assert!(
            res == Scad {
                op: ScadOp::Difference,
                children: vec![
                    Scad {
                        op: ScadOp::Circle {
                            radius: 1.0,
                            fa: None,
                            fs: None,
                            fn_: None,
                        },
                        children: Vec::new(),
                    },
                    Scad {
                        op: ScadOp::Square {
                            size: Pt2::new(1.0, 1.0),
                            center: false
                        },
                        children: Vec::new(),
                    }
                ],
            }
        );
    }

    #[test]
    fn intersection_of_2() {
        let res = intersection!(circle!(1.0);square!(1.0););
        assert!(
            res == Scad {
                op: ScadOp::Intersection,
                children: vec![
                    Scad {
                        op: ScadOp::Circle {
                            radius: 1.0,
                            fa: None,
                            fs: None,
                            fn_: None,
                        },
                        children: Vec::new(),
                    },
                    Scad {
                        op: ScadOp::Square {
                            size: Pt2::new(1.0, 1.0),
                            center: false,
                        },
                        children: Vec::new(),
                    }
                ],
            }
        );
    }

    #[test]
    fn circle_from_nradius() {
        let circle = circle!(r = 2.0);
        assert!(
            circle
                == Scad {
                    op: ScadOp::Circle {
                        radius: 2.0,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn circle_from_nradius_fn() {
        let circle = circle!(r=2.0, fn=4);
        assert!(
            circle
                == Scad {
                    op: ScadOp::Circle {
                        radius: 2.0,
                        fa: None,
                        fs: None,
                        fn_: Some(4)
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn circle_from_radius() {
        let circle = circle!(2.0);
        assert!(
            circle
                == Scad {
                    op: ScadOp::Circle {
                        radius: 2.0,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn circle_from_radius_fn() {
        let circle = circle!(2.0, fn=4);
        assert!(
            circle
                == Scad {
                    op: ScadOp::Circle {
                        radius: 2.0,
                        fa: None,
                        fs: None,
                        fn_: Some(4),
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn circle_from_radius_fa() {
        let circle = circle!(2.0, fa = 4.0);
        assert!(
            circle
                == Scad {
                    op: ScadOp::Circle {
                        radius: 2.0,
                        fa: Some(4.0),
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn circle_from_radius_fs() {
        let circle = circle!(2.0, fs = 4.0);
        assert!(
            circle
                == Scad {
                    op: ScadOp::Circle {
                        radius: 2.0,
                        fa: None,
                        fs: Some(4.0),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn circle_from_radius_fa_fs() {
        let circle = circle!(2.0, fa = 2.0, fs = 4.0);
        assert!(
            circle
                == Scad {
                    op: ScadOp::Circle {
                        radius: 2.0,
                        fa: Some(2.0),
                        fs: Some(4.0),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn circle_from_nradius_fa() {
        let circle = circle!(r = 2.0, fa = 4.0);
        assert!(
            circle
                == Scad {
                    op: ScadOp::Circle {
                        radius: 2.0,
                        fa: Some(4.0),
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn circle_from_nradius_fs() {
        let circle = circle!(r = 2.0, fs = 4.0);
        assert!(
            circle
                == Scad {
                    op: ScadOp::Circle {
                        radius: 2.0,
                        fa: None,
                        fs: Some(4.0),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn circle_from_nradius_fa_fs() {
        let circle = circle!(r = 2.0, fa = 2.0, fs = 4.0);
        assert!(
            circle
                == Scad {
                    op: ScadOp::Circle {
                        radius: 2.0,
                        fa: Some(2.0),
                        fs: Some(4.0),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn circle_from_diameter() {
        let circle = circle!(d = 2.0);
        assert!(
            circle
                == Scad {
                    op: ScadOp::Circle {
                        radius: 1.0,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn circle_from_diameter_fn() {
        let circle = circle!(d = 2.0, fn=4);
        assert!(
            circle
                == Scad {
                    op: ScadOp::Circle {
                        radius: 1.0,
                        fa: None,
                        fs: None,
                        fn_: Some(4),
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn circle_from_diameter_fa() {
        let circle = circle!(d = 2.0, fa = 4.0);
        assert!(
            circle
                == Scad {
                    op: ScadOp::Circle {
                        radius: 1.0,
                        fa: Some(4.0),
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn circle_from_diameter_fs() {
        let circle = circle!(d = 2.0, fs = 4.0);
        assert!(
            circle
                == Scad {
                    op: ScadOp::Circle {
                        radius: 1.0,
                        fa: None,
                        fs: Some(4.0),
                        fn_: None,
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn circle_from_diameter_fa_fs() {
        let circle = circle!(d = 2.0, fa = 2.0, fs = 4.0);
        assert!(
            circle
                == Scad {
                    op: ScadOp::Circle {
                        radius: 1.0,
                        fa: Some(2.0),
                        fs: Some(4.0),
                        fn_: None,
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn square_from_size() {
        let square = square!(10.0);
        assert!(
            square
                == Scad {
                    op: ScadOp::Square {
                        size: Pt2::new(10.0, 10.0),
                        center: false
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn square_from_size_center() {
        let square = square!(10.0, true);
        assert!(
            square
                == Scad {
                    op: ScadOp::Square {
                        size: Pt2::new(10.0, 10.0),
                        center: true,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn square_from_point_literal() {
        let square = square!([10.0, 10.0]);
        assert!(
            square
                == Scad {
                    op: ScadOp::Square {
                        size: Pt2::new(10.0, 10.0),
                        center: false,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn square_from_point_literal_center() {
        let square = square!([10.0, 10.0], true);
        assert!(
            square
                == Scad {
                    op: ScadOp::Square {
                        size: Pt2::new(10.0, 10.0),
                        center: true,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn polygon_from_points() {
        let points = Pt2s::from_pt2s(vec![
            Pt2::new(0.0, 0.0),
            Pt2::new(1.0, 1.0),
            Pt2::new(2.0, 2.0),
        ]);
        let polygon = polygon!(points.clone());
        assert!(
            polygon
                == Scad {
                    op: ScadOp::Polygon {
                        points: points,
                        paths: None,
                        convexity: 1
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn polygon_from_points_paths() {
        let points = Pt2s::from_pt2s(vec![
            Pt2::new(0.0, 0.0),
            Pt2::new(1.0, 1.0),
            Pt2::new(2.0, 2.0),
            Pt2::new(3.0, 3.0),
            Pt2::new(4.0, 4.0),
            Pt2::new(5.0, 5.0),
        ]);
        let paths = Paths::from_paths(vec![
            Indices::from_indices(vec![0, 1, 2]),
            Indices::from_indices(vec![3, 4, 5]),
        ]);
        let polygon = polygon!(points.clone(), paths.clone());
        assert!(
            polygon
                == Scad {
                    op: ScadOp::Polygon {
                        points: points,
                        paths: Some(paths),
                        convexity: 1
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn polygon_from_points_paths_convexity() {
        let points = Pt2s::from_pt2s(vec![
            Pt2::new(0.0, 0.0),
            Pt2::new(1.0, 1.0),
            Pt2::new(2.0, 2.0),
            Pt2::new(3.0, 3.0),
            Pt2::new(4.0, 4.0),
            Pt2::new(5.0, 5.0),
        ]);
        let paths = Paths::from_paths(vec![
            Indices::from_indices(vec![0, 1, 2]),
            Indices::from_indices(vec![3, 4, 5]),
        ]);
        let polygon = polygon!(points.clone(), paths.clone(), 2);
        assert!(
            polygon
                == Scad {
                    op: ScadOp::Polygon {
                        points: points,
                        paths: Some(paths),
                        convexity: 2
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn polygon_from_points_convexity() {
        let points = Pt2s::from_pt2s(vec![
            Pt2::new(0.0, 0.0),
            Pt2::new(1.0, 1.0),
            Pt2::new(2.0, 2.0),
        ]);
        let polygon = polygon!(points.clone(), convexity = 2);
        assert!(
            polygon
                == Scad {
                    op: ScadOp::Polygon {
                        points: points,
                        paths: None,
                        convexity: 2
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn text_from_string() {
        let text = text!("Text");
        assert!(
            text == Scad {
                op: ScadOp::Text {
                    text: "Text".to_string(),
                    size: 10.0,
                    font: "Liberation Sans".to_string(),
                    halign: TextHalign::left,
                    valign: TextValign::baseline,
                    spacing: 1.0,
                    direction: TextDirection::ltr,
                    language: "en".to_string(),
                    script: "latin".to_string(),
                    fn_: None,
                },
                children: Vec::new(),
            }
        )
    }

    #[test]
    fn text_from_string_size() {
        let text = text!("Text", 20.0);
        assert!(
            text == Scad {
                op: ScadOp::Text {
                    text: "Text".to_string(),
                    size: 20.0,
                    font: "Liberation Sans".to_string(),
                    halign: TextHalign::left,
                    valign: TextValign::baseline,
                    spacing: 1.0,
                    direction: TextDirection::ltr,
                    language: "en".to_string(),
                    script: "latin".to_string(),
                    fn_: None,
                },
                children: Vec::new(),
            }
        )
    }

    #[test]
    fn text_from_string_size_font() {
        let text = text!("Text", 20.0, "Courier New");
        assert!(
            text == Scad {
                op: ScadOp::Text {
                    text: "Text".to_string(),
                    size: 20.0,
                    font: "Courier New".to_string(),
                    halign: TextHalign::left,
                    valign: TextValign::baseline,
                    spacing: 1.0,
                    direction: TextDirection::ltr,
                    language: "en".to_string(),
                    script: "latin".to_string(),
                    fn_: None,
                },
                children: Vec::new(),
            }
        )
    }

    #[test]
    fn text_from_string_fn() {
        let text = text!("Text", fn=20);
        assert!(
            text == Scad {
                op: ScadOp::Text {
                    text: "Text".to_string(),
                    size: 10.0,
                    font: "Liberation Sans".to_string(),
                    halign: TextHalign::left,
                    valign: TextValign::baseline,
                    spacing: 1.0,
                    direction: TextDirection::ltr,
                    language: "en".to_string(),
                    script: "latin".to_string(),
                    fn_: Some(20),
                },
                children: Vec::new(),
            }
        )
    }

    #[test]
    fn text_from_string_size_fn() {
        let text = text!("Text", 20.0, fn=20);
        assert!(
            text == Scad {
                op: ScadOp::Text {
                    text: "Text".to_string(),
                    size: 20.0,
                    font: "Liberation Sans".to_string(),
                    halign: TextHalign::left,
                    valign: TextValign::baseline,
                    spacing: 1.0,
                    direction: TextDirection::ltr,
                    language: "en".to_string(),
                    script: "latin".to_string(),
                    fn_: Some(20),
                },
                children: Vec::new(),
            }
        )
    }

    #[test]
    fn text_from_string_size_font_fn() {
        let text = text!("Text", 20.0, "Courier New", fn=20);
        assert!(
            text == Scad {
                op: ScadOp::Text {
                    text: "Text".to_string(),
                    size: 20.0,
                    font: "Courier New".to_string(),
                    halign: TextHalign::left,
                    valign: TextValign::baseline,
                    spacing: 1.0,
                    direction: TextDirection::ltr,
                    language: "en".to_string(),
                    script: "latin".to_string(),
                    fn_: Some(20),
                },
                children: Vec::new(),
            }
        )
    }

    #[test]
    fn text_from_string_size_font_halign_valign_direction() {
        let text = text!(
            "Text",
            20.0,
            "Courier New",
            TextHalign::center,
            TextValign::center,
            TextDirection::ttb
        );
        assert!(
            text == Scad {
                op: ScadOp::Text {
                    text: "Text".to_string(),
                    size: 20.0,
                    font: "Courier New".to_string(),
                    halign: TextHalign::center,
                    valign: TextValign::center,
                    spacing: 1.0,
                    direction: TextDirection::ttb,
                    language: "en".to_string(),
                    script: "latin".to_string(),
                    fn_: None,
                },
                children: Vec::new(),
            }
        )
    }

    #[test]
    fn text_from_string_size_font_halign_valign_direction_fn() {
        let text = text!(
          "Text",
          20.0,
          "Courier New",
          TextHalign::center,
          TextValign::center,
          TextDirection::ttb,
          fn=20
        );
        assert!(
            text == Scad {
                op: ScadOp::Text {
                    text: "Text".to_string(),
                    size: 20.0,
                    font: "Courier New".to_string(),
                    halign: TextHalign::center,
                    valign: TextValign::center,
                    spacing: 1.0,
                    direction: TextDirection::ttb,
                    language: "en".to_string(),
                    script: "latin".to_string(),
                    fn_: Some(20),
                },
                children: Vec::new(),
            }
        )
    }

    #[test]
    fn text_from_all_params() {
        let text = text!(
            "Text",
            20.0,
            "Courier New",
            TextHalign::center,
            TextValign::center,
            2.0,
            TextDirection::ttb,
            "en",
            "latin",
            20
        );
        assert!(
            text == Scad {
                op: ScadOp::Text {
                    text: "Text".to_string(),
                    size: 20.0,
                    font: "Courier New".to_string(),
                    halign: TextHalign::center,
                    valign: TextValign::center,
                    spacing: 2.0,
                    direction: TextDirection::ttb,
                    language: "en".to_string(),
                    script: "latin".to_string(),
                    fn_: Some(20),
                },
                children: Vec::new(),
            }
        )
    }

    #[test]
    fn text_from_text_params() {
        let text_params = TextParams::default();
        let text = text!(text_params = text_params);
        assert!(
            text == Scad {
                op: ScadOp::Text {
                    text: "".to_string(),
                    size: 10.0,
                    font: "Liberation Sans".to_string(),
                    halign: TextHalign::left,
                    valign: TextValign::baseline,
                    spacing: 1.0,
                    direction: TextDirection::ltr,
                    language: "en".to_string(),
                    script: "latin".to_string(),
                    fn_: None,
                },
                children: Vec::new(),
            }
        )
    }

    #[test]
    fn import_from_file() {
        let import = import!("monkey");
        assert!(
            import
                == Scad {
                    op: ScadOp::Import {
                        file: "monkey".to_string(),
                        convexity: 1
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn import_from_file_convexity() {
        let import = import!("monkey", 3);
        assert!(
            import
                == Scad {
                    op: ScadOp::Import {
                        file: "monkey".to_string(),
                        convexity: 3
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn projection_from_child() {
        let res = projection!(square!(10.0););
        assert!(
            res == Scad {
                op: ScadOp::Projection { cut: false },
                children: vec![Scad {
                    op: ScadOp::Square {
                        size: Pt2::new(10.0, 10.0),
                        center: false
                    },
                    children: Vec::new()
                }]
            }
        )
    }

    #[test]
    fn projection_from_children() {
        let res = projection!(square!(10.0);circle!(10.0););
        assert!(
            res == Scad {
                op: ScadOp::Projection { cut: false },
                children: vec![
                    Scad {
                        op: ScadOp::Square {
                            size: Pt2::new(10.0, 10.0),
                            center: false
                        },
                        children: Vec::new()
                    },
                    Scad {
                        op: ScadOp::Circle {
                            radius: 10.0,
                            fa: None,
                            fs: None,
                            fn_: None,
                        },
                        children: Vec::new(),
                    }
                ]
            }
        )
    }

    #[test]
    fn projection_from_cut_child() {
        let res = projection!(cut=true,square!(10.0););
        assert!(
            res == Scad {
                op: ScadOp::Projection { cut: true },
                children: vec![Scad {
                    op: ScadOp::Square {
                        size: Pt2::new(10.0, 10.0),
                        center: false
                    },
                    children: Vec::new()
                }]
            }
        )
    }

    #[test]
    fn projection_from_cut_children() {
        let res = projection!(cut=true,square!(10.0);circle!(10.0););
        assert!(
            res == Scad {
                op: ScadOp::Projection { cut: true },
                children: vec![
                    Scad {
                        op: ScadOp::Square {
                            size: Pt2::new(10.0, 10.0),
                            center: false
                        },
                        children: Vec::new()
                    },
                    Scad {
                        op: ScadOp::Circle {
                            radius: 10.0,
                            fa: None,
                            fs: None,
                            fn_: None,
                        },
                        children: Vec::new(),
                    }
                ]
            }
        )
    }

    #[test]
    fn sphere_from_nradius() {
        let sphere = sphere!(r = 2.0);
        assert!(
            sphere
                == Scad {
                    op: ScadOp::Sphere {
                        radius: 2.0,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn sphere_from_nradius_fn() {
        let sphere = sphere!(r=2.0, fn=4);
        assert!(
            sphere
                == Scad {
                    op: ScadOp::Sphere {
                        radius: 2.0,
                        fa: None,
                        fs: None,
                        fn_: Some(4)
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn sphere_from_radius() {
        let sphere = sphere!(2.0);
        assert!(
            sphere
                == Scad {
                    op: ScadOp::Sphere {
                        radius: 2.0,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn sphere_from_radius_fn() {
        let sphere = sphere!(2.0, fn=4);
        assert!(
            sphere
                == Scad {
                    op: ScadOp::Sphere {
                        radius: 2.0,
                        fa: None,
                        fs: None,
                        fn_: Some(4),
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn sphere_from_radius_fa() {
        let sphere = sphere!(2.0, fa = 4.0);
        assert!(
            sphere
                == Scad {
                    op: ScadOp::Sphere {
                        radius: 2.0,
                        fa: Some(4.0),
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn sphere_from_radius_fs() {
        let sphere = sphere!(2.0, fs = 4.0);
        assert!(
            sphere
                == Scad {
                    op: ScadOp::Sphere {
                        radius: 2.0,
                        fa: None,
                        fs: Some(4.0),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn sphere_from_radius_fa_fs() {
        let sphere = sphere!(2.0, fa = 2.0, fs = 4.0);
        assert!(
            sphere
                == Scad {
                    op: ScadOp::Sphere {
                        radius: 2.0,
                        fa: Some(2.0),
                        fs: Some(4.0),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn sphere_from_nradius_fa() {
        let sphere = sphere!(r = 2.0, fa = 4.0);
        assert!(
            sphere
                == Scad {
                    op: ScadOp::Sphere {
                        radius: 2.0,
                        fa: Some(4.0),
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn sphere_from_nradius_fs() {
        let sphere = sphere!(r = 2.0, fs = 4.0);
        assert!(
            sphere
                == Scad {
                    op: ScadOp::Sphere {
                        radius: 2.0,
                        fa: None,
                        fs: Some(4.0),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn sphere_from_nradius_fa_fs() {
        let sphere = sphere!(r = 2.0, fa = 2.0, fs = 4.0);
        assert!(
            sphere
                == Scad {
                    op: ScadOp::Sphere {
                        radius: 2.0,
                        fa: Some(2.0),
                        fs: Some(4.0),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn sphere_from_diameter() {
        let sphere = sphere!(d = 2.0);
        assert!(
            sphere
                == Scad {
                    op: ScadOp::Sphere {
                        radius: 1.0,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn sphere_from_diameter_fn() {
        let sphere = sphere!(d = 2.0, fn=4);
        assert!(
            sphere
                == Scad {
                    op: ScadOp::Sphere {
                        radius: 1.0,
                        fa: None,
                        fs: None,
                        fn_: Some(4),
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn sphere_from_diameter_fa() {
        let sphere = sphere!(d = 2.0, fa = 4.0);
        assert!(
            sphere
                == Scad {
                    op: ScadOp::Sphere {
                        radius: 1.0,
                        fa: Some(4.0),
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn sphere_from_diameter_fs() {
        let sphere = sphere!(d = 2.0, fs = 4.0);
        assert!(
            sphere
                == Scad {
                    op: ScadOp::Sphere {
                        radius: 1.0,
                        fa: None,
                        fs: Some(4.0),
                        fn_: None,
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn sphere_from_diameter_fa_fs() {
        let sphere = sphere!(d = 2.0, fa = 2.0, fs = 4.0);
        assert!(
            sphere
                == Scad {
                    op: ScadOp::Sphere {
                        radius: 1.0,
                        fa: Some(2.0),
                        fs: Some(4.0),
                        fn_: None,
                    },
                    children: Vec::new()
                }
        )
    }

    #[test]
    fn cube_from_size() {
        let cube = cube!(10.0);
        assert!(
            cube == Scad {
                op: ScadOp::Cube {
                    size: Pt3::new(10.0, 10.0, 10.0),
                    center: false,
                },
                children: Vec::new(),
            }
        )
    }

    #[test]
    fn cube_from_size_center() {
        let cube = cube!(10.0, true);
        assert!(
            cube == Scad {
                op: ScadOp::Cube {
                    size: Pt3::new(10.0, 10.0, 10.0),
                    center: true,
                },
                children: Vec::new(),
            }
        )
    }

    #[test]
    fn cube_from_point() {
        let cube = cube!([10.0, 9.0, 8.0]);
        assert!(
            cube == Scad {
                op: ScadOp::Cube {
                    size: Pt3::new(10.0, 9.0, 8.0),
                    center: false,
                },
                children: Vec::new(),
            }
        )
    }

    #[test]
    fn cube_from_point_center() {
        let cube = cube!([1.0, 2.0, 3.0], true);
        assert!(
            cube == Scad {
                op: ScadOp::Cube {
                    size: Pt3::new(1.0, 2.0, 3.0),
                    center: true,
                },
                children: Vec::new(),
            }
        )
    }

    #[test]
    fn cylinder_from_height_radius() {
        let cylinder = cylinder!(12.0, 2.0);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 2.0,
                        center: false,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn cylinder_from_height_radius1_radius2() {
        let cylinder = cylinder!(12.0, 2.0, 1.0);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: false,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn cylinder_from_height_radius1_radius2_center() {
        let cylinder = cylinder!(12.0, 2.0, 1.0, true);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: true,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_height_radius1_radius2_center_fa() {
        let cylinder = cylinder!(12.0, 2.0, 1.0, true, fa = 4.0);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: true,
                        fa: Some(4.0),
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_height_radius1_radius2_center_fs() {
        let cylinder = cylinder!(12.0, 2.0, 1.0, true, fs = 0.25);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: true,
                        fa: None,
                        fs: Some(0.25),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_height_radius1_radius2_center_fa_fs() {
        let cylinder = cylinder!(12.0, 2.0, 1.0, true, fa = 2.0, fs = 0.1);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: true,
                        fa: Some(2.0),
                        fs: Some(0.1),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_height_radius1_radius2_center_fn() {
        let cylinder = cylinder!(12.0, 2.0, 1.0, true, fn=12);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: true,
                        fa: None,
                        fs: None,
                        fn_: Some(12),
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn cylinder_from_height_diameter() {
        let cylinder = cylinder!(12.0, d = 2.0);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 1.0,
                        radius2: 1.0,
                        center: false,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn cylinder_from_height_diameter1_diameter2() {
        let cylinder = cylinder!(12.0, d1 = 4.0, d2 = 2.0);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: false,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn cylinder_from_height_diameter1_diameter2_center() {
        let cylinder = cylinder!(12.0, d1 = 2.0, d2 = 1.0, center = true);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 1.0,
                        radius2: 0.5,
                        center: true,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_height_diameter1_diameter2_center_fa() {
        let cylinder = cylinder!(12.0, d1 = 4.0, d2 = 2.0, center = true, fa = 4.0);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: true,
                        fa: Some(4.0),
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_height_diameter1_diameter2_center_fs() {
        let cylinder = cylinder!(12.0, d1 = 4.0, d2 = 2.0, center = true, fs = 0.25);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: true,
                        fa: None,
                        fs: Some(0.25),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_height_diameter1_diameter2_center_fa_fs() {
        let cylinder = cylinder!(12.0, d1 = 2.0, d2 = 1.0, center = true, fa = 2.0, fs = 0.1);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 1.0,
                        radius2: 0.5,
                        center: true,
                        fa: Some(2.0),
                        fs: Some(0.1),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_height_diameter1_diameter2_center_fn() {
        let cylinder = cylinder!(12.0, d1=2.0, d2=1.0, center=true, fn=12);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 1.0,
                        radius2: 0.5,
                        center: true,
                        fa: None,
                        fs: None,
                        fn_: Some(12),
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn cylinder_from_nheight_radius() {
        let cylinder = cylinder!(h = 12.0, r = 2.0);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 2.0,
                        center: false,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn cylinder_from_nheight_radius1_radius2() {
        let cylinder = cylinder!(h = 12.0, r1 = 2.0, r2 = 1.0);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: false,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn cylinder_from_nheight_radius1_radius2_center() {
        let cylinder = cylinder!(h = 12.0, r1 = 2.0, r2 = 1.0, center = true);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: true,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_nheight_radius1_radius2_center_fa() {
        let cylinder = cylinder!(h = 12.0, r1 = 2.0, r2 = 1.0, center = true, fa = 4.0);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: true,
                        fa: Some(4.0),
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_nheight_radius1_radius2_center_fs() {
        let cylinder = cylinder!(h = 12.0, r1 = 2.0, r2 = 1.0, center = true, fs = 0.25);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: true,
                        fa: None,
                        fs: Some(0.25),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_nheight_radius1_radius2_center_fa_fs() {
        let cylinder = cylinder!(
            h = 12.0,
            r1 = 2.0,
            r2 = 1.0,
            center = true,
            fa = 2.0,
            fs = 0.1
        );
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: true,
                        fa: Some(2.0),
                        fs: Some(0.1),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_nheight_radius1_radius2_center_fn() {
        let cylinder = cylinder!(h=12.0, r1=2.0, r2=1.0, center=true, fn=12);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: true,
                        fa: None,
                        fs: None,
                        fn_: Some(12),
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn cylinder_from_nheight_diameter() {
        let cylinder = cylinder!(h = 12.0, d = 2.0);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 1.0,
                        radius2: 1.0,
                        center: false,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn cylinder_from_nheight_diameter1_diameter2() {
        let cylinder = cylinder!(h = 12.0, d1 = 4.0, d2 = 2.0);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: false,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn cylinder_from_nheight_diameter1_diameter2_center() {
        let cylinder = cylinder!(h = 12.0, d1 = 2.0, d2 = 1.0, center = true);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 1.0,
                        radius2: 0.5,
                        center: true,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_nheight_diameter1_diameter2_center_fa() {
        let cylinder = cylinder!(h = 12.0, d1 = 4.0, d2 = 2.0, center = true, fa = 4.0);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: true,
                        fa: Some(4.0),
                        fs: None,
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_nheight_diameter1_diameter2_center_fs() {
        let cylinder = cylinder!(h = 12.0, d1 = 4.0, d2 = 2.0, center = true, fs = 0.25);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 2.0,
                        radius2: 1.0,
                        center: true,
                        fa: None,
                        fs: Some(0.25),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_nheight_diameter1_diameter2_center_fa_fs() {
        let cylinder = cylinder!(
            h = 12.0,
            d1 = 2.0,
            d2 = 1.0,
            center = true,
            fa = 2.0,
            fs = 0.1
        );
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 1.0,
                        radius2: 0.5,
                        center: true,
                        fa: Some(2.0),
                        fs: Some(0.1),
                        fn_: None,
                    },
                    children: Vec::new(),
                }
        )
    }
    #[test]
    fn cylinder_from_nheight_diameter1_diameter2_center_fn() {
        let cylinder = cylinder!(h=12.0, d1=2.0, d2=1.0, center=true, fn=12);
        assert!(
            cylinder
                == Scad {
                    op: ScadOp::Cylinder {
                        height: 12.0,
                        radius1: 1.0,
                        radius2: 0.5,
                        center: true,
                        fa: None,
                        fs: None,
                        fn_: Some(12),
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn polyhedron_from_points_faces() {
        let points = Pt3s::from_pt3s(vec![
            Pt3::new(0.0, 0.0, 0.0),
            Pt3::new(10.0, 0.0, 0.0),
            Pt3::new(10.0, 7.0, 0.0),
            Pt3::new(0.0, 7.0, 0.0),
            Pt3::new(0.0, 0.0, 5.0),
            Pt3::new(10.0, 0.0, 5.0),
            Pt3::new(10.0, 7.0, 5.0),
            Pt3::new(0.0, 7.0, 5.0),
        ]);
        let faces = Faces::from_faces(vec![
            Indices::from_indices(vec![0, 1, 2, 3]),
            Indices::from_indices(vec![4, 5, 1, 0]),
            Indices::from_indices(vec![7, 6, 5, 4]),
            Indices::from_indices(vec![5, 6, 2, 1]),
            Indices::from_indices(vec![6, 7, 3, 2]),
            Indices::from_indices(vec![7, 4, 0, 3]),
        ]);
        let polyhedron = polyhedron!(points.clone(), faces.clone());
        assert!(
            polyhedron
                == Scad {
                    op: ScadOp::Polyhedron {
                        points,
                        faces,
                        convexity: 1
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn polyhedron_from_points_faces_convexity() {
        let points = Pt3s::from_pt3s(vec![
            Pt3::new(0.0, 0.0, 0.0),
            Pt3::new(10.0, 0.0, 0.0),
            Pt3::new(10.0, 7.0, 0.0),
            Pt3::new(0.0, 7.0, 0.0),
            Pt3::new(0.0, 0.0, 5.0),
            Pt3::new(10.0, 0.0, 5.0),
            Pt3::new(10.0, 7.0, 5.0),
            Pt3::new(0.0, 7.0, 5.0),
        ]);
        let faces = Faces::from_faces(vec![
            Indices::from_indices(vec![0, 1, 2, 3]),
            Indices::from_indices(vec![4, 5, 1, 0]),
            Indices::from_indices(vec![7, 6, 5, 4]),
            Indices::from_indices(vec![5, 6, 2, 1]),
            Indices::from_indices(vec![6, 7, 3, 2]),
            Indices::from_indices(vec![7, 4, 0, 3]),
        ]);
        let polyhedron = polyhedron!(points.clone(), faces.clone(), 4);
        assert!(
            polyhedron
                == Scad {
                    op: ScadOp::Polyhedron {
                        points,
                        faces,
                        convexity: 4
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn polyhedron_from_npoints_faces_convexity() {
        let points = Pt3s::from_pt3s(vec![
            Pt3::new(0.0, 0.0, 0.0),
            Pt3::new(10.0, 0.0, 0.0),
            Pt3::new(10.0, 7.0, 0.0),
            Pt3::new(0.0, 7.0, 0.0),
            Pt3::new(0.0, 0.0, 5.0),
            Pt3::new(10.0, 0.0, 5.0),
            Pt3::new(10.0, 7.0, 5.0),
            Pt3::new(0.0, 7.0, 5.0),
        ]);
        let faces = Faces::from_faces(vec![
            Indices::from_indices(vec![0, 1, 2, 3]),
            Indices::from_indices(vec![4, 5, 1, 0]),
            Indices::from_indices(vec![7, 6, 5, 4]),
            Indices::from_indices(vec![5, 6, 2, 1]),
            Indices::from_indices(vec![6, 7, 3, 2]),
            Indices::from_indices(vec![7, 4, 0, 3]),
        ]);
        let polyhedron = polyhedron!(
            points = points.clone(),
            faces = faces.clone(),
            convexity = 4
        );
        assert!(
            polyhedron
                == Scad {
                    op: ScadOp::Polyhedron {
                        points,
                        faces,
                        convexity: 4
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn linear_extrude_from_height_children() {
        let child = cube!([10.0, 3.0, 7.5]);
        let linear_extrude = linear_extrude!(10.0,
            cube!([10.0, 3.0, 7.5]);
        );
        assert!(
            linear_extrude
                == Scad {
                    op: ScadOp::LinearExtrude {
                        height: 10.0,
                        center: false,
                        convexity: 1,
                        twist: 0.0,
                        scale: Pt2::new(1.0, 1.0),
                        slices: None,
                        fn_: None,
                    },
                    children: vec![child],
                }
        )
    }

    #[test]
    fn linear_extrude_from_all_slices_children() {
        let child = cube!([10.0, 3.0, 7.5]);
        let linear_extrude = linear_extrude!(height=10.0, center=false, convexity=1, twist=0.0, scale=1.0, slices=10,
            cube!([10.0, 3.0, 7.5]);
        );
        assert!(
            linear_extrude
                == Scad {
                    op: ScadOp::LinearExtrude {
                        height: 10.0,
                        center: false,
                        convexity: 1,
                        twist: 0.0,
                        scale: Pt2::new(1.0, 1.0),
                        slices: Some(10),
                        fn_: None,
                    },
                    children: vec![child],
                }
        )
    }

    #[test]
    fn linear_extrude_from_all_fn_children() {
        let child = cube!([10.0, 3.0, 7.5]);
        let linear_extrude = linear_extrude!(height=10.0, center=false, convexity=1, twist=0.0, scale=1.0, fn=10,
            cube!([10.0, 3.0, 7.5]);
        );
        assert!(
            linear_extrude
                == Scad {
                    op: ScadOp::LinearExtrude {
                        height: 10.0,
                        center: false,
                        convexity: 1,
                        twist: 0.0,
                        scale: Pt2::new(1.0, 1.0),
                        slices: None,
                        fn_: Some(10),
                    },
                    children: vec![child],
                }
        )
    }

    #[test]
    fn linear_extrude_from_all_separate_scale_slices_children() {
        let child = cube!([10.0, 3.0, 7.5]);
        let linear_extrude = linear_extrude!(height=10.0, center=false, convexity=1, twist=0.0, scale=[2.0, 1.0], slices=10,
            cube!([10.0, 3.0, 7.5]);
        );
        assert!(
            linear_extrude
                == Scad {
                    op: ScadOp::LinearExtrude {
                        height: 10.0,
                        center: false,
                        convexity: 1,
                        twist: 0.0,
                        scale: Pt2::new(2.0, 1.0),
                        slices: Some(10),
                        fn_: None,
                    },
                    children: vec![child],
                }
        )
    }

    #[test]
    fn linear_extrude_from_all_separate_scale_fn_children() {
        let child = cube!([10.0, 3.0, 7.5]);
        let linear_extrude = linear_extrude!(height=10.0, center=false, convexity=1, twist=0.0, scale=[1.0, 2.0], fn=10,
            cube!([10.0, 3.0, 7.5]);
        );
        assert!(
            linear_extrude
                == Scad {
                    op: ScadOp::LinearExtrude {
                        height: 10.0,
                        center: false,
                        convexity: 1,
                        twist: 0.0,
                        scale: Pt2::new(1.0, 2.0),
                        slices: None,
                        fn_: Some(10),
                    },
                    children: vec![child],
                }
        )
    }

    #[test]
    fn rotate_extrude_from_children() {
        let rotate_extrude = rotate_extrude!(square!(1.0););
        assert!(
            rotate_extrude
                == Scad {
                    op: ScadOp::RotateExtrude {
                        angle: 360.0,
                        convexity: 1,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: vec![square!(1.0)],
                }
        )
    }

    #[test]
    fn rotate_extrude_from_angle() {
        let rotate_extrude = rotate_extrude!(angle=45.0, square!(1.0););
        assert!(
            rotate_extrude
                == Scad {
                    op: ScadOp::RotateExtrude {
                        angle: 45.0,
                        convexity: 1,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: vec![square!(1.0)],
                }
        )
    }

    #[test]
    fn rotate_extrude_from_angle_convexity() {
        let rotate_extrude = rotate_extrude!(angle=45.0, convexity=12, square!(1.0););
        assert!(
            rotate_extrude
                == Scad {
                    op: ScadOp::RotateExtrude {
                        angle: 45.0,
                        convexity: 12,
                        fa: None,
                        fs: None,
                        fn_: None,
                    },
                    children: vec![square!(1.0)],
                }
        )
    }

    #[test]
    fn rotate_extrude_from_angle_convexity_fa() {
        let rotate_extrude = rotate_extrude!(angle=45.0, convexity=12, fa=2.0, square!(1.0););
        assert!(
            rotate_extrude
                == Scad {
                    op: ScadOp::RotateExtrude {
                        angle: 45.0,
                        convexity: 12,
                        fa: Some(2.0),
                        fs: None,
                        fn_: None,
                    },
                    children: vec![square!(1.0)],
                }
        )
    }

    #[test]
    fn rotate_extrude_from_angle_convexity_fs() {
        let rotate_extrude = rotate_extrude!(angle=45.0, convexity=12, fs=2.0, square!(1.0););
        assert!(
            rotate_extrude
                == Scad {
                    op: ScadOp::RotateExtrude {
                        angle: 45.0,
                        convexity: 12,
                        fa: None,
                        fs: Some(2.0),
                        fn_: None,
                    },
                    children: vec![square!(1.0)],
                }
        )
    }

    #[test]
    fn rotate_extrude_from_angle_convexity_fa_fs() {
        let rotate_extrude =
            rotate_extrude!(angle=45.0, convexity=12, fa=1.5, fs=2.0, square!(1.0););
        assert!(
            rotate_extrude
                == Scad {
                    op: ScadOp::RotateExtrude {
                        angle: 45.0,
                        convexity: 12,
                        fa: Some(1.5),
                        fs: Some(2.0),
                        fn_: None,
                    },
                    children: vec![square!(1.0)],
                }
        )
    }

    #[test]
    fn rotate_extrude_from_angle_convexity_fn() {
        let rotate_extrude = rotate_extrude!(angle=45.0, convexity=12, fn=6, square!(1.0););
        assert!(
            rotate_extrude
                == Scad {
                    op: ScadOp::RotateExtrude {
                        angle: 45.0,
                        convexity: 12,
                        fa: None,
                        fs: None,
                        fn_: Some(6),
                    },
                    children: vec![square!(1.0)],
                }
        )
    }

    #[test]
    fn surface_from_file() {
        let surface = surface!("test.data");
        assert!(
            surface
                == Scad {
                    op: ScadOp::Surface {
                        file: "test.data".to_string(),
                        center: false,
                        invert: false,
                        convexity: 1,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn surface_from_all() {
        let surface = surface!(
            file = "test.data",
            center = true,
            invert = true,
            convexity = 12
        );
        assert!(
            surface
                == Scad {
                    op: ScadOp::Surface {
                        file: "test.data".to_string(),
                        center: true,
                        invert: true,
                        convexity: 12,
                    },
                    children: Vec::new(),
                }
        )
    }

    #[test]
    fn translate_from_point_children() {
        let translate = translate!([1.0, 2.0, 3.0],
            circle!(10.0);
        );
        assert!(
            translate
                == Scad {
                    op: ScadOp::Translate {
                        v: Pt3::new(1.0, 2.0, 3.0)
                    },
                    children: vec![circle!(10.0)],
                }
        )
    }

    #[test]
    fn translate_from_npoint_children() {
        let translate = translate!(v=[1.0, 2.0, 3.0],
            circle!(10.0);
        );
        assert!(
            translate
                == Scad {
                    op: ScadOp::Translate {
                        v: Pt3::new(1.0, 2.0, 3.0)
                    },
                    children: vec![circle!(10.0)],
                }
        )
    }

    #[test]
    fn rotate_from_point_children() {
        let rotate = rotate!([0.0, 180.0, 0.0], square!(1.0););
        assert!(
            rotate
                == Scad {
                    op: ScadOp::Rotate {
                        a: None,
                        a_is_scalar: false,
                        v: Pt3::new(0.0, 180.0, 0.0),
                    },
                    children: vec![square!(1.0)]
                }
        )
    }

    #[test]
    fn rotate_from_npoint_children() {
        let rotate = rotate!(a=[0.0, 180.0, 0.0], square!(1.0););
        assert!(
            rotate
                == Scad {
                    op: ScadOp::Rotate {
                        a: None,
                        a_is_scalar: false,
                        v: Pt3::new(0.0, 180.0, 0.0),
                    },
                    children: vec![square!(1.0)]
                }
        )
    }

    #[test]
    fn rotate_from_scalar_children() {
        let rotate = rotate!(180.0, square!(1.0););
        assert!(
            rotate
                == Scad {
                    op: ScadOp::Rotate {
                        a: Some(180.0),
                        a_is_scalar: true,
                        v: Pt3::new(0.0, 0.0, 0.0),
                    },
                    children: vec![square!(1.0)]
                }
        )
    }

    #[test]
    fn rotate_from_nscalar_children() {
        let rotate = rotate!(a=180.0, square!(1.0););
        assert!(
            rotate
                == Scad {
                    op: ScadOp::Rotate {
                        a: Some(180.0),
                        a_is_scalar: true,
                        v: Pt3::new(0.0, 0.0, 0.0),
                    },
                    children: vec![square!(1.0)]
                }
        )
    }

    #[test]
    fn rotate_from_angle_axis_children() {
        let rotate = rotate!(180.0, [0.0, 1.0, 0.0], square!(1.0););
        assert!(
            rotate
                == Scad {
                    op: ScadOp::Rotate {
                        a: Some(180.0),
                        a_is_scalar: false,
                        v: Pt3::new(0.0, 1.0, 0.0),
                    },
                    children: vec![square!(1.0)]
                }
        )
    }

    #[test]
    fn rotate_from_nangle_axis_children() {
        let rotate = rotate!(a=180.0, v=[0.0, 1.0, 0.0], square!(1.0););
        assert!(
            rotate
                == Scad {
                    op: ScadOp::Rotate {
                        a: Some(180.0),
                        a_is_scalar: false,
                        v: Pt3::new(0.0, 1.0, 0.0),
                    },
                    children: vec![square!(1.0)]
                }
        )
    }

    #[test]
    fn scale_from_vector_children() {
        let scale = scale!([2.0, 1.0, 2.0], square!(1.0););
        assert!(
            scale
                == Scad {
                    op: ScadOp::Scale {
                        v: Pt3::new(2.0, 1.0, 2.0),
                    },
                    children: vec![square!(1.0)]
                }
        )
    }

    #[test]
    fn scale_from_nvector_children() {
        let scale = scale!(v=[2.0, 1.0, 2.0], square!(1.0););
        assert!(
            scale
                == Scad {
                    op: ScadOp::Scale {
                        v: Pt3::new(2.0, 1.0, 2.0),
                    },
                    children: vec![square!(1.0)]
                }
        )
    }

    #[test]
    fn resize_from_size_children() {
        let resize = resize!([2.0, 2.0, 6.0], cube!(10.0););
        assert!(
            resize
                == Scad {
                    op: ScadOp::Resize {
                        newsize: Pt3::new(2.0, 2.0, 6.0),
                        auto: false,
                        auto_is_vec: false,
                        autovec: (false, false, false),
                        convexity: 1,
                    },
                    children: vec![cube!(10.0)],
                }
        )
    }

    #[test]
    fn resize_from_size_auto_children() {
        let resize = resize!([2.0, 2.0, 6.0], true, cube!(10.0););
        assert!(
            resize
                == Scad {
                    op: ScadOp::Resize {
                        newsize: Pt3::new(2.0, 2.0, 6.0),
                        auto: true,
                        auto_is_vec: false,
                        autovec: (false, false, false),
                        convexity: 1,
                    },
                    children: vec![cube!(10.0)],
                }
        )
    }

    #[test]
    fn resize_from_size_autovec_children() {
        let resize = resize!([2.0, 2.0, 6.0], [true, false, true], cube!(10.0););
        assert!(
            resize
                == Scad {
                    op: ScadOp::Resize {
                        newsize: Pt3::new(2.0, 2.0, 6.0),
                        auto: false,
                        auto_is_vec: true,
                        autovec: (true, false, true),
                        convexity: 1,
                    },
                    children: vec![cube!(10.0)],
                }
        )
    }

    #[test]
    fn resize_from_size_auto_convexity_children() {
        let resize = resize!([2.0, 2.0, 6.0], true, 10, cube!(10.0););
        assert!(
            resize
                == Scad {
                    op: ScadOp::Resize {
                        newsize: Pt3::new(2.0, 2.0, 6.0),
                        auto: true,
                        auto_is_vec: false,
                        autovec: (false, false, false),
                        convexity: 10,
                    },
                    children: vec![cube!(10.0)],
                }
        )
    }

    #[test]
    fn resize_from_size_autovec_convexity_children() {
        let resize = resize!([2.0, 2.0, 6.0], [true, false, true], 10, cube!(10.0););
        assert!(
            resize
                == Scad {
                    op: ScadOp::Resize {
                        newsize: Pt3::new(2.0, 2.0, 6.0),
                        auto: false,
                        auto_is_vec: true,
                        autovec: (true, false, true),
                        convexity: 10,
                    },
                    children: vec![cube!(10.0)],
                }
        )
    }

    #[test]
    fn resize_from_nsize_children() {
        let resize = resize!(newsize=[2.0, 2.0, 6.0], cube!(10.0););
        assert!(
            resize
                == Scad {
                    op: ScadOp::Resize {
                        newsize: Pt3::new(2.0, 2.0, 6.0),
                        auto: false,
                        auto_is_vec: false,
                        autovec: (false, false, false),
                        convexity: 1,
                    },
                    children: vec![cube!(10.0)],
                }
        )
    }

    #[test]
    fn resize_from_nsize_auto_children() {
        let resize = resize!(newsize=[2.0, 2.0, 6.0], auto=true, cube!(10.0););
        assert!(
            resize
                == Scad {
                    op: ScadOp::Resize {
                        newsize: Pt3::new(2.0, 2.0, 6.0),
                        auto: true,
                        auto_is_vec: false,
                        autovec: (false, false, false),
                        convexity: 1,
                    },
                    children: vec![cube!(10.0)],
                }
        )
    }

    #[test]
    fn resize_from_nsize_autovec_children() {
        let resize = resize!(newsize=[2.0, 2.0, 6.0], auto=[true, false, true], cube!(10.0););
        assert!(
            resize
                == Scad {
                    op: ScadOp::Resize {
                        newsize: Pt3::new(2.0, 2.0, 6.0),
                        auto: false,
                        auto_is_vec: true,
                        autovec: (true, false, true),
                        convexity: 1,
                    },
                    children: vec![cube!(10.0)],
                }
        )
    }

    #[test]
    fn resize_from_nsize_auto_convexity_children() {
        let resize = resize!(newsize=[2.0, 2.0, 6.0], auto=true, convexity=10, cube!(10.0););
        assert!(
            resize
                == Scad {
                    op: ScadOp::Resize {
                        newsize: Pt3::new(2.0, 2.0, 6.0),
                        auto: true,
                        auto_is_vec: false,
                        autovec: (false, false, false),
                        convexity: 10,
                    },
                    children: vec![cube!(10.0)],
                }
        )
    }

    #[test]
    fn resize_from_nsize_autovec_convexity_children() {
        let resize =
            resize!(newsize=[2.0, 2.0, 6.0], auto=[true, false, true], convexity=10, cube!(10.0););
        assert!(
            resize
                == Scad {
                    op: ScadOp::Resize {
                        newsize: Pt3::new(2.0, 2.0, 6.0),
                        auto: false,
                        auto_is_vec: true,
                        autovec: (true, false, true),
                        convexity: 10,
                    },
                    children: vec![cube!(10.0)],
                }
        )
    }

    #[test]
    fn mirror_from_vec_children() {
        let mirror = mirror!([1.0, 1.0, 1.0], cube!(20.0););
        assert!(
            mirror
                == Scad {
                    op: ScadOp::Mirror {
                        v: Pt3::new(1.0, 1.0, 1.0)
                    },
                    children: vec![cube!(20.0)],
                }
        )
    }

    #[test]
    fn mirror_from_nvec_children() {
        let mirror = mirror!(v=[1.0, 1.0, 1.0], cube!(20.0););
        assert!(
            mirror
                == Scad {
                    op: ScadOp::Mirror {
                        v: Pt3::new(1.0, 1.0, 1.0)
                    },
                    children: vec![cube!(20.0)],
                }
        )
    }

    #[test]
    fn color_from_pt4_children() {
        let color = color!([0.18, 0.18, 0.18, 1.0], cube!(20.0););
        assert!(
            color
                == Scad {
                    op: ScadOp::Color {
                        rgba: Some(Pt4::new(0.18, 0.18, 0.18, 1.0)),
                        color: None,
                        hex: None,
                        alpha: None,
                    },
                    children: vec![cube!(20.0)],
                }
        )
    }

    #[test]
    fn color_from_hex_children() {
        let color = color!("#12345678", cube!(20.0););
        assert!(
            color
                == Scad {
                    op: ScadOp::Color {
                        rgba: None,
                        color: None,
                        hex: Some("#12345678".to_string()),
                        alpha: None,
                    },
                    children: vec![cube!(20.0)],
                }
        )
    }

    #[test]
    fn color_from_color_children() {
        let color = color!(c=ScadColor::BlanchedAlmond, cube!(20.0););
        assert!(
            color
                == Scad {
                    op: ScadOp::Color {
                        rgba: None,
                        color: Some(ScadColor::BlanchedAlmond),
                        hex: None,
                        alpha: None,
                    },
                    children: vec![cube!(20.0)],
                }
        )
    }

    #[test]
    fn color_from_color_alpha_children() {
        let color = color!(c=ScadColor::BlanchedAlmond, alpha=0.75, cube!(20.0););
        assert!(
            color
                == Scad {
                    op: ScadOp::Color {
                        rgba: None,
                        color: Some(ScadColor::BlanchedAlmond),
                        hex: None,
                        alpha: Some(0.75),
                    },
                    children: vec![cube!(20.0)],
                }
        )
    }

    #[test]
    fn offset_from_r_children() {
        let offset = offset!(0.75, square!(20.0););
        assert!(
            offset
                == Scad {
                    op: ScadOp::Offset {
                        r: Some(0.75),
                        delta: None,
                        chamfer: false,
                    },
                    children: vec![square!(20.0)],
                }
        )
    }

    #[test]
    fn offset_from_delta_chamfer_children() {
        let offset = offset!(delta=0.75, chamfer=true, square!(20.0););
        assert!(
            offset
                == Scad {
                    op: ScadOp::Offset {
                        r: None,
                        delta: Some(0.75),
                        chamfer: true,
                    },
                    children: vec![square!(20.0)],
                }
        )
    }

    #[test]
    fn hull_from_children() {
        let hull = hull!(square!(20.0););
        assert!(
            hull == Scad {
                op: ScadOp::Hull,
                children: vec![square!(20.0)],
            }
        )
    }

    #[test]
    fn minkowski_from_children() {
        let minkowski = minkowski!(square!(20.0););
        assert!(
            minkowski
                == Scad {
                    op: ScadOp::Minkowski { convexity: 1 },
                    children: vec![square!(20.0)],
                }
        )
    }

    #[test]
    fn minkowski_from_convexity_children() {
        let minkowski = minkowski!(12, square!(20.0););
        assert!(
            minkowski
                == Scad {
                    op: ScadOp::Minkowski { convexity: 12 },
                    children: vec![square!(20.0)],
                }
        )
    }
}
