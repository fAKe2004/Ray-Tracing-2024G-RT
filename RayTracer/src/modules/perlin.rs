use nalgebra::ComplexField;

use crate::utility::{*};
use crate::vec3::{self, *};

const PERLIN_POINT_COUNT: usize = 256;
pub struct Perlin {
  randvec: Vec<Vec3>,
  perm_x: Vec<i32>,
  perm_y: Vec<i32>,
  perm_z: Vec<i32>,
}

impl Perlin {
  pub fn new() -> Self {
    let mut randvec = vec![Vec3::zero(); PERLIN_POINT_COUNT];
    for i in 0..PERLIN_POINT_COUNT {
      randvec[i] = Vec3::rand_unit();
    }
    Perlin {
      randvec,
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
    for i in (1..=n-1).rev() { // n-1..0 是没用的... 要 (1..=n-1).rev()
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

    let mut c = vec![vec![vec![Vec3::zero(); 2]; 2]; 2];

    for di in 0..2 as i32 {
      for dj in 0..2 as i32 {
        for dk in 0..2 as i32 {
          c[di as usize][dj as usize][dk as usize] = self.randvec[
            (self.perm_x[((i + di) & 255) as usize] ^
            self.perm_y[((j + dj) & 255) as usize] ^
            self.perm_z[((k + dk) & 255) as usize]) as usize
          ];
        }
      }
    }
    Self::perlin_interp(c, u, v, w)
  }

  fn perlin_interp(c: Vec<Vec<Vec<Vec3>>>, u: f64, v: f64, w: f64) -> f64 {
    let (uu, vv, ww) = (
      u * u * (3.0 - 2.0 * u),
      v * v * (3.0 - 2.0 * v),
      w * w * (3.0 - 2.0 * w)
    );
    let mut accum = 0.0;
    for i in 0..2 as i32 {
      for j in 0..2 as i32 {
        for k in 0..2 as i32 {
          let weight_vec = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
          accum += 
            (i as f64 * uu + (1 - i) as f64 * (1.0 - uu)) *
            (j as f64 * vv + (1 - j) as f64 * (1.0 - vv)) *
            (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
            * c[i as usize][j as usize][k as usize].dot(&weight_vec);
        }
      }
    }
    accum
  }
}