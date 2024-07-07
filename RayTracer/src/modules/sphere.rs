use crate::{Material, PI};

use crate::vec3::{*};
use crate::ray::{*};
use crate::hittable::{*};
use crate::interval::{*};
use crate::material::{*};
use crate::aabb::{*};

use std::sync::Arc;

pub struct Sphere {
  pub center: Point3,
  pub radius: f64,
  pub mat: Material,
  is_moving: bool,
  center_vec: Vec3,
  bbox: Aabb,
}

impl Sphere {
  pub fn new(center: Point3, radius: f64, mat: Material, center_vec: Vec3) -> Self {
    let rvec = Vec3::new(radius, radius, radius);
    let is_moving = !center_vec.near_zero();
    let mut bbox = Aabb::new_by_point(center - rvec, center + rvec);
    if is_moving {
      let center_after_move = center + center_vec;
      bbox = Aabb::new_by_aabb(bbox, 
        Aabb::new_by_point(center_after_move - rvec, center_after_move + rvec));
    }
    Sphere {
      center,
      radius,
      mat,
      is_moving,
      center_vec,
      bbox,
    }
  }

  pub fn new_static(center: Point3, radius: f64, mat: Material) -> Self {
    Self::new(center, radius, mat, Vec3::zero())
  }
  pub fn new_moving(center: Point3, center_after_move: Point3, radius: f64, mat: Material) -> Self {
    Self::new(center, radius, mat, center_after_move - center)
  }

  pub fn sphere_center(&self, time: f64) -> Point3 {
    if self.is_moving {
      self.center + self.center_vec * time
    } else {
      self.center
    }
  }

  pub fn get_spherer_uv(p: Point3 /* on unit sphere */) -> (f64, f64) {
    let theta = (-p.y).acos();
    let phi = (-p.z).atan2(p.x) + PI;

    let u = phi / (2.0 * PI);
    let v = theta / PI;

    (u, v)
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
    let center = self.sphere_center(ray.tm);
    let oc = center - ray.orig;
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
    
    let outward_normal = (ray.at(root) - center).normalize();
    let (u, v) = Self::get_spherer_uv(outward_normal);
    *rec = HitRecord::new_from_ray_and_outward_normal(ray, outward_normal, self.mat.clone(), root, u, v);

    true
  }
  fn to_object(self) -> Object {
    Arc::new(self)
  }
  fn bounding_box(&self) -> Aabb {
    self.bbox
  }
}

