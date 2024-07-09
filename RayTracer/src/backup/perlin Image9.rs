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
    let (i, j, k) = (
      (4.0 * p.x) as i32 & 255,
      (4.0 * p.y) as i32 & 255,
      (4.0 * p.z) as i32 & 255, // 不能提前转 usize，不然会负数会炸
    );

    self.randfloat[(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
  }
}