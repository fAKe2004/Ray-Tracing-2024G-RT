use super::vec3::*;

pub struct Ray {
  pub orig: Point3,
  pub dir: Vec3,
}

impl Ray {
  pub fn new(orig: Point3, dir: Vec3) -> Self {
    Ray {
      orig,
      dir,
    }
  }
  pub fn at(&self, t: f64) -> Point3 {
    self.orig + self.dir * t
  }
}
