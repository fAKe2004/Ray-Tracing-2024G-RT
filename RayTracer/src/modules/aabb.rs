use std::cmp::{min, max};

use crate::utility::{*};
use crate::vec3::{*};
use crate::ray::{*};

use crate::Interval;

pub struct Aabb { 
  x: Interval, // assumed to have min < max
  y: Interval,
  z: Interval,
}

impl Aabb {
  pub fn new(x: Interval, y: Interval, z: Interval) -> Self{
    Aabb {
      x,
      y,
      z,
    }
  }

  pub fn new_by_point(a: Point3, b: Point3) -> Self{
    Aabb {
      x: Interval::new_adaptive(a.x, b.x),
      y: Interval::new_adaptive(a.y, b.y),
      z: Interval::new_adaptive(a.z, b.z),
    }
  }

  pub fn new_by_aabb(a: Aabb, b: Aabb) -> Self{
    Aabb {
      x: Interval::new_union(a.x, b.x),
      y: Interval::new_union(a.y, b.y),
      z: Interval::new_union(a.z, b.z),
    }
  }


  pub fn default() -> Self{
    Aabb {
      x: Interval::default(),
      y: Interval::default(),
      z: Interval::default(),
    }
  }

  pub fn axis_interval(&self, which: usize) -> Interval {
    match which {
      0 => self.x,
      1 => self.y,
      2 => self.z,
      _ => panic!("invalid indexing"),
    }
  }
  
  pub fn hit(&self, ray: &Ray, ray_t: Interval) -> bool {
    let mut ray_t = ray_t;
    for axis in 0..3 as usize {
      let ax = self.axis_interval(axis);
      let adinv = 1.0 / ray.dir[axis];

      let t0 = (ax.min - ray.orig[axis]) * adinv;
      let t1 = (ax.max - ray.orig[axis]) * adinv;

      let axis_t = Interval::new_adaptive(t0, t1);
      ray_t = ray_t.overlap(axis_t);
      if ray_t.empty() {
        return false
      }
    }
    true
  }

  pub fn longest_axis(&self) -> usize {
    let x_len = self.x.size();
    let y_len = self.y.size();
    let z_len = self.z.size();
    let longest_len: f64 = x_len.max(y_len).max(z_len);
    if longest_len == x_len {
      0
    } else if longest_len == y_len {
      1 
    } else {
      2
    }
  }
}

impl Clone for Aabb {
  fn clone(&self) -> Self {
    Self {
      ..*self
    }
  }
}

impl Copy for Aabb {

}