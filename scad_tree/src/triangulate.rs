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

//! A Rust implementation of the ear clipping algorithm described, and coded in C++, at
//! https://abitwise.blogspot.com/2013/09/triangulating-concave-and-convex.html

use crate::{approx_eq, Indices, Pt2, Pt2s, Pt3, Pt3s};

/// Test if winding order is counter clockwise.
///
/// pts: The points of the polygon.
///
/// return: True if ccw else false.
pub fn is_ccw(pts: &Vec<(u64, Pt2)>) -> bool {
    (pts[1].1.x - pts[0].1.x) * (pts[2].1.y - pts[0].1.y)
        - (pts[2].1.x - pts[0].1.x) * (pts[1].1.y - pts[0].1.y)
        > 0.0
}

/// Check if a point is within a triangle.
///
/// p: The point to check.
///
/// a: The first vertex of the triangle in ccw order.
///
/// b: The second vertex of the triangle.
///
/// c: The third vertex of the triangle.
///
/// return: True if the point is within the triangle else false.   
pub fn in_triangle(p: &(u64, Pt2), a: &(u64, Pt2), b: &(u64, Pt2), c: &(u64, Pt2)) -> bool {
    let mut denom = (b.1.y - c.1.y) * (a.1.x - c.1.x) + (c.1.x - b.1.x) * (a.1.y - c.1.y);
    if approx_eq(denom, 0.0, 1.0e-5) {
        return true;
    }
    denom = 1.0 / denom;

    let alpha = denom * ((b.1.y - c.1.y) * (p.1.x - c.1.x) + (c.1.x - b.1.x) * (p.1.y - c.1.y));
    if alpha < 0.0 {
        return false;
    }

    let beta = denom * ((c.1.y - a.1.y) * (p.1.x - c.1.x) + (a.1.x - c.1.x) * (p.1.y - c.1.y));
    if beta < 0.0 {
        return false;
    }

    let gamma = 1.0 - alpha - beta;
    if gamma < 0.0 {
        return false;
    }
    true
}

/// Triangulate a 3D polygon
///
/// vertices: The vertices of the polygon.
///
/// normal: The normal of the polygon.
///
/// return: An array of indices into the given vertex array.
pub fn triangulate3d(vertices: &Pt3s, normal: Pt3) -> Indices {
    assert!(vertices.len() > 3);
    const PX: u8 = 1;
    const NX: u8 = 2;
    const PY: u8 = 3;
    const NY: u8 = 4;
    const PZ: u8 = 5;
    const NZ: u8 = 6;
    let mut nml_type = 0;
    if normal.x.abs() >= normal.y.abs() && normal.x.abs() >= normal.z.abs() {
        if normal.x >= 0.0 {
            nml_type = PX;
        } else {
            nml_type = NX;
        }
    } else if normal.y.abs() >= normal.x.abs() && normal.y.abs() >= normal.z.abs() {
        if normal.y >= 0.0 {
            nml_type = PY;
        } else {
            nml_type = NY;
        }
    } else if normal.z.abs() >= normal.x.abs() && normal.z.abs() >= normal.y.abs() {
        if normal.z >= 0.0 {
            nml_type = PZ;
        } else {
            nml_type = NZ;
        }
    }
    let mut polygon = Vec::with_capacity(vertices.len());
    match nml_type {
        PX => {
            // x = y, y = z
            for (i, v) in vertices.iter().enumerate() {
                polygon.push((i as u64, Pt2::new(v.y, v.z)));
            }
        }
        NX => {
            // x = -y, y = z
            for (i, v) in vertices.iter().enumerate() {
                polygon.push((i as u64, Pt2::new(-v.y, v.z)));
            }
        }
        PY => {
            // x = -x, y = z
            for (i, v) in vertices.iter().enumerate() {
                polygon.push((i as u64, Pt2::new(-v.x, v.z)));
            }
        }
        NY => {
            // x = x, y = z
            for (i, v) in vertices.iter().enumerate() {
                polygon.push((i as u64, Pt2::new(v.x, v.z)));
            }
        }
        PZ => {
            // x = x, y = y
            for (i, v) in vertices.iter().enumerate() {
                polygon.push((i as u64, Pt2::new(v.x, v.y)));
            }
        }
        NZ => {
            // x = -x, y =  y
            for (i, v) in vertices.iter().enumerate() {
                polygon.push((i as u64, Pt2::new(-v.x, v.y)));
            }
        }
        _ => {}
    }

    triangulate(polygon)
}

/// Triangulate a 2D polygon
///
/// vertices: The vertices of the polygon.
///
/// return: An array of indices into the given vertex array.
pub fn triangulate2d(vertices: &Pt2s) -> Indices {
    assert!(vertices.len() > 3);
    let mut polygon = Vec::with_capacity(vertices.len());
    for (i, v) in vertices.iter().enumerate() {
        polygon.push((i as u64, *v));
    }

    triangulate(polygon)
}

// triangulates clockwise
fn triangulate(mut polygon: Vec<(u64, Pt2)>) -> Indices {
    let mut triangles = Indices::from_indices(Vec::with_capacity((polygon.len() - 2) * 3));

    let mut left = polygon[0].1;
    let mut index = 0usize;

    for i in 0..polygon.len() {
        if polygon[i].1.x < left.x
            || (approx_eq(polygon[i].1.x, left.x, 1.0e-5) && polygon[i].1.y < left.y)
        {
            index = i;
            left = polygon[i].1;
        }
    }

    let tri = vec![
        polygon[if index == 0 {
            polygon.len() - 1
        } else {
            index - 1
        }],
        polygon[index],
        polygon[if index == polygon.len() - 1 {
            0
        } else {
            index + 1
        }],
    ];
    assert!(!is_ccw(&tri));

    while polygon.len() >= 3 {
        let mut eartip = -1i16;
        let mut index = -1i16;

        for i in &polygon {
            index += 1;
            if eartip >= 0 {
                break;
            }

            let p: u16 = if index == 0 {
                (polygon.len() - 1) as u16
            } else {
                (index - 1) as u16
            };
            let n: u16 = if index as usize == polygon.len() - 1 {
                0
            } else {
                (index + 1) as u16
            };

            let tri = vec![polygon[p as usize], *i, polygon[n as usize]];
            if is_ccw(&tri) {
                continue;
            }

            let mut ear = true;

            for j in ((index + 1) as usize)..polygon.len() {
                let v = &polygon[j];
                if std::ptr::eq(v, &polygon[p as usize])
                    || std::ptr::eq(v, &polygon[n as usize])
                    || std::ptr::eq(v, &polygon[index as usize])
                {
                    continue;
                }
                if in_triangle(v, &polygon[p as usize], i, &polygon[n as usize]) {
                    ear = false;
                    break;
                }
            }

            if ear {
                eartip = index;
            }
        } // for i in &polygon
        if eartip < 0 {
            break;
        }
        let p = if eartip == 0 {
            polygon.len() - 1
        } else {
            eartip as usize - 1
        };
        let n = if eartip == (polygon.len() - 1) as i16 {
            0
        } else {
            eartip as usize + 1
        };
        triangles.push(polygon[p].0);
        triangles.push(polygon[eartip as usize].0);
        triangles.push(polygon[n].0);

        polygon.remove(eartip as usize);
    } // while polygon.len()

    triangles
}
