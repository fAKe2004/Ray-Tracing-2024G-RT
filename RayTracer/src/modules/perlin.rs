use nalgebra::ComplexField;

use crate::utility::{*};
use crate::vec3::{*};

const PERLIN_POINT_COUNT: usize = 256;
pub struct Perlin {
  randfloat: Vec<f64>,
  perm_x: Vec<i32>,
  perm_y: Vec<i32>,
  perm_z: Vec<i32>,
}

impl Perlin {
  pub fn new() -> Self {
    let mut randfloat = vec![0.0 as f64; PERLIN_POINT_COUNT];
    for i in 0..PERLIN_POINT_COUNT {
      randfloat[i] = rand_01();
    }
    Perlin {
      randfloat,
      perm_x: Self::generate_perm(),
      perm_y: Self::generate_perm(),
      perm_z: Self::generate_perm(),
    }
  }

  pub fn generate_perm() -> Vec<i32> {
    let mut p: Vec<i32> = vec![0; PERLIN_POINT_COUNT];
    for i in 0..PERLIN_POINT_COUNT {
      p[i] = i as i32;
    }
    Self::random_permutation(p, PERLIN_POINT_COUNT) 
  }

  pub fn random_permutation(mut p: Vec<i32>, n: usize) -> Vec<i32> {
    for i in n - 1..0 {
      let target = rand_range_int(0, i as i32) as usize;
      let swp = p[i];
      p[i] = p[target];
      p[target] = swp;
    }
    p
  }

  pub fn noise(&self, p: Point3) -> f64 {
    let (u, v, w) = (
      p.x - p.x.floor(),
      p.y - p.y.floor(),
      p.z - p.z.floor()
    );

    let (i, j, k) = (
      p.x.floor() as i32,
      p.y.floor() as i32,
      p.z.floor() as i32
    );

    let mut c = vec![vec![vec![0.0 as f64; 2]; 2]; 2];

    for di in 0..2 as i32 {
      for dj in 0..2 as i32 {
        for dk in 0..2 as i32 {
          c[di as usize][dj as usize][dk as usize] = self.randfloat[
            (self.perm_x[((i + di) & 255) as usize] ^
            self.perm_y[((j + dj) & 255) as usize] ^
            self.perm_z[((k + dk) & 255) as usize]) as usize
          ];
        }
      }
    }
    Self::trilinear_interp(c, u, v, w)
  }

  fn trilinear_interp(c: Vec<Vec<Vec<f64>>>, u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    for i in 0..2 as i32 {
      for j in 0..2 as i32 {
        for k in 0..2 as i32 {
          accum += 
            (i as f64 * u + (1 - i) as f64 * (1.0 - u)) *
            (j as f64 * v + (1 - j) as f64 * (1.0 - v)) *
            (k as f64 * w + (1 - k) as f64 * (1.0 - w))
            * c[i as usize][j as usize][k as usize];
        }
      }
    }
    accum
  }
}