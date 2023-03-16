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

use crate::{dcos, dsin, dtan, Pt3, Pt4};

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Mt4 {
  pub x: Pt4,
  pub y: Pt4,
  pub z: Pt4,
  pub w: Pt4,
}

impl std::fmt::Display for Mt4 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "| {} {} {} {} |", self.x.x, self.y.x, self.z.x, self.w.x)?;
    writeln!(f, "| {} {} {} {} |", self.x.y, self.y.y, self.z.y, self.w.y)?;
    writeln!(f, "| {} {} {} {} |", self.x.z, self.y.z, self.z.z, self.w.z)?;
    writeln!(f, "| {} {} {} {} |", self.x.w, self.y.w, self.z.w, self.w.w)
  }
}

impl Mt4 {
  pub fn new(x: Pt4, y: Pt4, z: Pt4, w: Pt4) -> Self {
    Self { x, y, z, w }
  }

  pub fn transposed(&self) -> Self {
    Mt4::new(
      Pt4::new(self.x.x, self.y.x, self.z.x, self.w.x),
      Pt4::new(self.x.y, self.y.y, self.z.y, self.w.y),
      Pt4::new(self.x.z, self.y.z, self.z.z, self.w.z),
      Pt4::new(self.x.w, self.y.w, self.z.w, self.w.w),
    )
  }

  pub fn identity() -> Self {
    Mt4::new(
      Pt4::new(1.0, 0.0, 0.0, 0.0),
      Pt4::new(0.0, 1.0, 0.0, 0.0),
      Pt4::new(0.0, 0.0, 1.0, 0.0),
      Pt4::new(0.0, 0.0, 0.0, 1.0),
    )
  }

  pub fn scale_matrix(x: f64, y: f64, z: f64) -> Self {
    let mut result = Mt4::identity();
    result.x.x = x;
    result.y.y = y;
    result.z.z = z;
    result
  }

  pub fn translate_matrix(x: f64, y: f64, z: f64) -> Self {
    let mut result = Mt4::identity();
    result.w.x = x;
    result.w.y = y;
    result.w.z = z;
    result
  }

  pub fn rot_x_matrix(degrees: f64) -> Self {
    let c = dcos(degrees);
    let s = dsin(degrees);
    Mt4::new(
      Pt4::new(1.0, 0.0, 0.0, 0.0),
      Pt4::new(0.0, c, -s, 0.0),
      Pt4::new(0.0, s, c, 0.0),
      Pt4::new(0.0, 0.0, 0.0, 1.0),
    )
    .transposed()
  }

  pub fn rot_y_matrix(degrees: f64) -> Self {
    let c = dcos(degrees);
    let s = dsin(degrees);
    Mt4::new(
      Pt4::new(c, 0.0, s, 0.0),
      Pt4::new(0.0, 1.0, 0.0, 0.0),
      Pt4::new(-s, 0.0, c, 0.0),
      Pt4::new(0.0, 0.0, 0.0, 1.0),
    )
    .transposed()
  }

  pub fn rot_z_matrix(degrees: f64) -> Self {
    let c = dcos(degrees);
    let s = dsin(degrees);
    Mt4::new(
      Pt4::new(c, -s, 0.0, 0.0),
      Pt4::new(s, c, 0.0, 0.0),
      Pt4::new(0.0, 0.0, 1.0, 0.0),
      Pt4::new(0.0, 0.0, 0.0, 1.0),
    )
    .transposed()
  }

  pub fn rot_vec(x: f64, y: f64, z: f64, degrees: f64) -> Self {
    let c = dcos(degrees);
    let s = dsin(degrees);
    Mt4::new(
      Pt4::new(
        c + x * x * (1.0 - c),
        x * y * (1.0 - c) - z * s,
        x * z * (1.0 - c) + y * s,
        0.0,
      ),
      Pt4::new(
        y * x * (1.0 - c) + z * s,
        c + y * y * (1.0 - c),
        y * z * (1.0 - c) - x * s,
        0.0,
      ),
      Pt4::new(
        z * x * (1.0 - c) - z * s,
        z * y * (1.0 - c) + x * s,
        c + z * z * (1.0 - c),
        0.0,
      ),
      Pt4::new(0.0, 0.0, 0.0, 1.0),
    )
    .transposed()
  }

  pub fn perspective_matrix(fovy: f64, aspect: f64, near: f64, far: f64) -> Self {
    let tan_half_fovy = dtan(fovy / 2.0);
    Mt4::new(
      Pt4::new(1.0 / (aspect * tan_half_fovy), 0.0, 0.0, 0.0),
      Pt4::new(0.0, 1.0 / tan_half_fovy, 0.0, 0.0),
      Pt4::new(
        0.0,
        0.0,
        -(far + near) / (far - near),
        -(2.0 * far * near) / (far - near),
      ),
      Pt4::new(0.0, 0.0, -1.0, 1.0),
    )
    .transposed()
  }

  pub fn inverse(&self) -> Option<Self> {
    let mut out = Mt4::identity();

    out[0] =
      self[5] * self[10] * self[15] - self[5] * self[11] * self[14] - self[9] * self[6] * self[15]
        + self[9] * self[7] * self[14]
        + self[13] * self[6] * self[11]
        - self[13] * self[7] * self[10];

    out[4] =
      -self[4] * self[10] * self[15] + self[4] * self[11] * self[14] + self[8] * self[6] * self[15]
        - self[8] * self[7] * self[14]
        - self[12] * self[6] * self[11]
        + self[12] * self[7] * self[10];

    out[8] =
      self[4] * self[9] * self[15] - self[4] * self[11] * self[13] - self[8] * self[5] * self[15]
        + self[8] * self[7] * self[13]
        + self[12] * self[5] * self[11]
        - self[12] * self[7] * self[9];

    out[12] =
      -self[4] * self[9] * self[14] + self[4] * self[10] * self[13] + self[8] * self[5] * self[14]
        - self[8] * self[6] * self[13]
        - self[12] * self[5] * self[10]
        + self[12] * self[6] * self[9];

    out[1] =
      -self[1] * self[10] * self[15] + self[1] * self[11] * self[14] + self[9] * self[2] * self[15]
        - self[9] * self[3] * self[14]
        - self[13] * self[2] * self[11]
        + self[13] * self[3] * self[10];

    out[5] =
      self[0] * self[10] * self[15] - self[0] * self[11] * self[14] - self[8] * self[2] * self[15]
        + self[8] * self[3] * self[14]
        + self[12] * self[2] * self[11]
        - self[12] * self[3] * self[10];

    out[9] =
      -self[0] * self[9] * self[15] + self[0] * self[11] * self[13] + self[8] * self[1] * self[15]
        - self[8] * self[3] * self[13]
        - self[12] * self[1] * self[11]
        + self[12] * self[3] * self[9];

    out[13] =
      self[0] * self[9] * self[14] - self[0] * self[10] * self[13] - self[8] * self[1] * self[14]
        + self[8] * self[2] * self[13]
        + self[12] * self[1] * self[10]
        - self[12] * self[2] * self[9];

    out[2] =
      self[1] * self[6] * self[15] - self[1] * self[7] * self[14] - self[5] * self[2] * self[15]
        + self[5] * self[3] * self[14]
        + self[13] * self[2] * self[7]
        - self[13] * self[3] * self[6];

    out[6] =
      -self[0] * self[6] * self[15] + self[0] * self[7] * self[14] + self[4] * self[2] * self[15]
        - self[4] * self[3] * self[14]
        - self[12] * self[2] * self[7]
        + self[12] * self[3] * self[6];

    out[10] =
      self[0] * self[5] * self[15] - self[0] * self[7] * self[13] - self[4] * self[1] * self[15]
        + self[4] * self[3] * self[13]
        + self[12] * self[1] * self[7]
        - self[12] * self[3] * self[5];

    out[14] =
      -self[0] * self[5] * self[14] + self[0] * self[6] * self[13] + self[4] * self[1] * self[14]
        - self[4] * self[2] * self[13]
        - self[12] * self[1] * self[6]
        + self[12] * self[2] * self[5];

    out[3] =
      -self[1] * self[6] * self[11] + self[1] * self[7] * self[10] + self[5] * self[2] * self[11]
        - self[5] * self[3] * self[10]
        - self[9] * self[2] * self[7]
        + self[9] * self[3] * self[6];

    out[7] =
      self[0] * self[6] * self[11] - self[0] * self[7] * self[10] - self[4] * self[2] * self[11]
        + self[4] * self[3] * self[10]
        + self[8] * self[2] * self[7]
        - self[8] * self[3] * self[6];

    out[11] =
      -self[0] * self[5] * self[11] + self[0] * self[7] * self[9] + self[4] * self[1] * self[11]
        - self[4] * self[3] * self[9]
        - self[8] * self[1] * self[7]
        + self[8] * self[3] * self[5];

    out[15] =
      self[0] * self[5] * self[10] - self[0] * self[6] * self[9] - self[4] * self[1] * self[10]
        + self[4] * self[2] * self[9]
        + self[8] * self[1] * self[6]
        - self[8] * self[2] * self[5];

    let mut det = self[0] * out[0] + self[1] * out[4] + self[2] * out[8] + self[3] * out[12];

    if det == 0.0 {
      None
    } else {
      det = 1.0 / det;
      for i in 0..16 {
        out[i] = out[i] * det;
      }
      Some(out)
    }
  }

  pub fn look_at_matrix_rh(eye: Pt3, center: Pt3, up: Pt3) -> Self {
    let mut f = center - eye;
    f = f.normalized();
    let mut s = f.cross(up);
    s = s.normalized();
    let u = s.cross(f);
    Mt4::new(
      Pt4::new(s.x, s.y, s.z, -s.dot(eye)),
      Pt4::new(u.x, u.y, u.z, -u.dot(eye)),
      Pt4::new(-f.x, -f.y, -f.z, f.dot(eye)),
      Pt4::new(0.0, 0.0, 0.0, 1.0),
    )
  }

  // I don't understand this yet but this one maps the z axis to the axis
  // from eye to center, it's used to make the line segments in EuclideanSpace
  pub fn look_at_matrix_lh(eye: Pt3, center: Pt3, up: Pt3) -> Self {
    let mut f = center - eye;
    f = f.normalized();
    let mut s = up.cross(f);
    // parallel check
    if s == Pt3::new(0.0, 0.0, 0.0) {
      // anti-parallel check
      if up.dot(f) < 0.0 {
        return Mt4::rot_x_matrix(180.0);
      }
      return Mt4::identity();
    }
    s = s.normalized();
    let u = f.cross(s);
    Mt4::new(
      Pt4::new(s.x, s.y, s.z, -s.dot(eye)),
      Pt4::new(u.x, u.y, u.z, -u.dot(eye)),
      Pt4::new(f.x, f.y, f.z, -f.dot(eye)),
      Pt4::new(0.0, 0.0, 0.0, 1.0),
    )
  }

  pub fn rotation_from_direction(direction: Pt3, up: Pt3) -> Self {
    let x_axis: Pt3 = up.cross(direction).normalized();
    let z_axis: Pt3 = direction.cross(x_axis).normalized();
    let mut result = Mt4::identity();
    result.x.x = x_axis.x;
    result.x.y = direction.x;
    result.x.z = z_axis.x;

    result.y.x = x_axis.y;
    result.y.y = direction.y;
    result.y.z = z_axis.y;

    result.z.x = x_axis.z;
    result.z.y = direction.z;
    result.z.z = z_axis.z;

    result
  }
}

impl std::ops::Mul<Pt4> for Mt4 {
  type Output = Pt4;

  fn mul(self, rhs: Pt4) -> Self::Output {
    let t = self.transposed();
    Pt4::new(t.x.dot(rhs), t.y.dot(rhs), t.z.dot(rhs), t.w.dot(rhs))
  }
}

impl std::ops::Mul<Pt3> for Mt4 {
  type Output = Pt3;

  fn mul(self, rhs: Pt3) -> Self::Output {
    let t = self.transposed();
    Pt3::new(
      t.x.as_pt3().dot(rhs),
      t.y.as_pt3().dot(rhs),
      t.z.as_pt3().dot(rhs),
    )
  }
}

impl std::ops::Mul<Mt4> for Mt4 {
  type Output = Mt4;

  fn mul(self, rhs: Mt4) -> Self::Output {
    let t = self.transposed();
    Mt4::new(
      Pt4::new(
        t.x.dot(rhs.x),
        t.y.dot(rhs.x),
        t.z.dot(rhs.x),
        t.w.dot(rhs.x),
      ),
      Pt4::new(
        t.x.dot(rhs.y),
        t.y.dot(rhs.y),
        t.z.dot(rhs.y),
        t.w.dot(rhs.y),
      ),
      Pt4::new(
        t.x.dot(rhs.z),
        t.y.dot(rhs.z),
        t.z.dot(rhs.z),
        t.w.dot(rhs.z),
      ),
      Pt4::new(
        t.x.dot(rhs.w),
        t.y.dot(rhs.w),
        t.z.dot(rhs.w),
        t.w.dot(rhs.w),
      ),
    )
  }
}

impl std::ops::Index<usize> for Mt4 {
  type Output = f64;

  fn index(&self, index: usize) -> &Self::Output {
    match index {
      0 => &self.x.x,
      1 => &self.x.y,
      2 => &self.x.z,
      3 => &self.x.w,
      4 => &self.y.x,
      5 => &self.y.y,
      6 => &self.y.z,
      7 => &self.y.w,
      8 => &self.z.x,
      9 => &self.z.y,
      10 => &self.z.z,
      11 => &self.z.w,
      12 => &self.w.x,
      13 => &self.w.y,
      14 => &self.w.z,
      15 => &self.w.w,
      _ => panic!("Index out of bounds"),
    }
  }
}

impl std::ops::IndexMut<usize> for Mt4 {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    match index {
      0 => &mut self.x.x,
      1 => &mut self.x.y,
      2 => &mut self.x.z,
      3 => &mut self.x.w,
      4 => &mut self.y.x,
      5 => &mut self.y.y,
      6 => &mut self.y.z,
      7 => &mut self.y.w,
      8 => &mut self.z.x,
      9 => &mut self.z.y,
      10 => &mut self.z.z,
      11 => &mut self.z.w,
      12 => &mut self.w.x,
      13 => &mut self.w.y,
      14 => &mut self.w.z,
      15 => &mut self.w.w,
      _ => panic!("Index out of bounds"),
    }
  }
}
