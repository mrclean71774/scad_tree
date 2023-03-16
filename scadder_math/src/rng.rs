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

//! Random number generation via Mersenne Twister algorithm.
//! A port of https://github.com/ESultanik/mtwister

const STATE_VECTOR_LENGTH: usize = 624;
const STATE_VECTOR_M: usize = 397; // changes to STATE_VECTOR_LENGTH also require changes to this

const UPPER_MASK: u32 = 0x80000000;
const LOWER_MASK: u32 = 0x7fffffff;
const TEMPERING_MASK_B: u32 = 0x9d2c5680;
const TEMPERING_MASK_C: u32 = 0xefc60000;

#[derive(Clone)]
pub struct MersenneTwister {
  buffer: Vec<u32>,
  index: usize,
}

impl MersenneTwister {
  fn next(&mut self) -> u32 {
    let mut y: u32;
    let mag: [u32; 2] = [0x0, 0x9908b0df]; // mag[x] = x * 0x9908b0df for x = 0,1
    if self.index >= STATE_VECTOR_LENGTH {
      let mut kk: usize = 0;
      while kk < STATE_VECTOR_LENGTH - STATE_VECTOR_M {
        y = (self.buffer[kk] & UPPER_MASK) | (self.buffer[kk + 1] & LOWER_MASK);
        self.buffer[kk] = self.buffer[kk + STATE_VECTOR_M] ^ (y >> 1) ^ mag[(y & 0x1) as usize];
        kk += 1;
      }
      while kk < STATE_VECTOR_LENGTH - 1 {
        y = (self.buffer[kk] & UPPER_MASK) | (self.buffer[kk + 1] & LOWER_MASK);
        self.buffer[kk] = self.buffer
          [(kk as i64 + (STATE_VECTOR_M as i64 - STATE_VECTOR_LENGTH as i64)) as usize]
          ^ (y >> 1)
          ^ mag[(y & 0x1) as usize];
        kk += 1;
      }
      y = (self.buffer[STATE_VECTOR_LENGTH - 1] & UPPER_MASK) | (self.buffer[0] & LOWER_MASK);
      self.buffer[STATE_VECTOR_LENGTH - 1] =
        self.buffer[STATE_VECTOR_M - 1] ^ (y >> 1) ^ mag[(y & 0x1) as usize];
      self.index = 0;
    }
    y = self.buffer[self.index];
    self.index += 1;
    y ^= y >> 11;
    y ^= (y << 7) & TEMPERING_MASK_B;
    y ^= (y << 15) & TEMPERING_MASK_C;
    y ^= y >> 18;
    y
  }

  pub fn raw(&mut self) -> u32 {
    self.next()
  }

  pub fn f32_0_1(&mut self) -> f32 {
    let mut u = self.next();
    if u == u32::MAX {
      u -= 1
    };
    u as f32 / 0xffffffffu32 as f32
  }

  pub fn i32_minmax(&mut self, min: i32, max: i32) -> i32 {
    min + ((max - min) as f32 * self.f32_0_1()) as i32
  }

  pub fn f32_minmax(&mut self, min: f32, max: f32) -> f32 {
    min + (max - min) * self.f32_0_1() as f32
  }

  pub fn f64_minmax(&mut self, min: f64, max: f64) -> f64 {
    min + (max - min) * self.f32_0_1() as f64
  }

  pub fn new() -> Self {
    let t = std::time::SystemTime::now();
    let ptr = &t as *const std::time::SystemTime as *const usize;
    Self::with_seed(unsafe { (*ptr & 0xffffffff) as u32 })
  }

  pub fn with_seed(seed: u32) -> Self {
    let mut result = MersenneTwister {
      buffer: Vec::with_capacity(STATE_VECTOR_LENGTH),
      index: 1,
    };
    unsafe {
      result.buffer.set_len(STATE_VECTOR_LENGTH);
    }
    result.buffer[0] = seed & 0xffffffff;
    while result.index < STATE_VECTOR_LENGTH {
      result.buffer[result.index] =
        ((6069 * result.buffer[result.index - 1] as usize) & 0xffffffff) as u32;
      result.index += 1;
    }
    result
  }
}
