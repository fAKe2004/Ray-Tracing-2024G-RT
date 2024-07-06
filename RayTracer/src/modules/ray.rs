use super::vec3::*;

pub struct Ray {
  pub orig: Point3,
  pub dir: Vec3,
  pub tm: f64,
}

impl Ray {
  pub fn new(orig: Point3, dir: Vec3, tm: f64) -> Self {
    Ray {
      orig,
      dir,
      tm,
    }
  }
  pub fn default() -> Self {
    Ray::new(Point3::zero(), Vec3::zero(), 0.0)
  }

  
  pub fn new_static(orig: Point3, dir: Vec3) -> Self {
    Ray {
      orig,
      dir,
      tm: 0.0,
    }
  }
  pub fn at(&self, t: f64) -> Point3 {
    self.orig + self.dir * t
  }
}
