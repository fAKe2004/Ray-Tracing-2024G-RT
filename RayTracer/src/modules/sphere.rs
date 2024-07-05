use crate::Material;

use super::vec3::{*};
use super::ray::{*};
use super::hittable::{*};
use super::interval::{*};
use super::material::{*};

use std::sync::Arc;

pub struct Sphere {
  pub center: Point3,
  pub radius: f64,
  pub mat: Material,
}

impl Sphere {
  pub fn new(center: Point3, radius: f64, mat: Material) -> Self {
    Sphere {
      center,
      radius,
      mat,
    }
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
    let oc = self.center - ray.orig;
    let a = ray.dir.norm_squared();
    // let b = -2.0 * ray.dir.dot(&oc);
    let h = ray.dir.dot(&oc);
    let c = oc.norm_squared() - self.radius * self.radius;
    let discriminant = h * h - a * c;

    let mut root = 0.0;
    let result = if discriminant < 0.0 {
        false
      } else {
        let sqrtd = discriminant.sqrt();
        root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
          root = (h + sqrtd) / a;
          ray_t.surrounds(root)
        } else {
          true
        }
    };

    if result == false {
      return false;
    }
    
    *rec = HitRecord::new_from_ray_and_outward_normal(ray, ray.at(root) - self.center, self.mat.clone(), root);

    true
  }
  fn to_object(self) -> Object {
    Arc::new(self)
  }
}

