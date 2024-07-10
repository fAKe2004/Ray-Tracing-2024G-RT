use crate::utility::{*};
use crate::vec3::{*};
use crate::color::{*};
use crate::ray::{*};
use crate::interval::{*};
use crate::hittable::{*};
use crate::material::{*};
use crate::texture::{*};
use crate::{EPS, INFINITY, E};

use std::sync::{Arc};

pub struct ConstantMedium {
  boundary: Object,
  neg_inv_density: f64,
  phase_function: Material,
}

impl ConstantMedium {
  pub fn new(boundary: Object, density: f64, tex: Texture) -> Self {
    Self {
      boundary,
      neg_inv_density: -1.0 / density,
      phase_function: Isotropic::new(tex).to_material(),
    }
  }

  pub fn new_by_color(boundary: Object, density: f64, albedo: ColorType) -> Self {
    Self::new(boundary, density, SolidColor::new(albedo).to_texture())
  }
}

impl Hittable for ConstantMedium {
  fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
    let mut rec1 = HitRecord::default();
    let mut rec2 = HitRecord::default();


    if !self.boundary.hit(ray, Interval::new(-INFINITY, INFINITY), &mut rec1) {
      return false;
    }

    if !self.boundary.hit(ray, Interval::new(rec1.t + EPS, INFINITY), &mut rec2) {
      return false;
    }

    rec1.t = ray_t.clamp(rec1.t);
    rec2.t = ray_t.clamp(rec2.t);

    if rec1.t >= rec2.t {
      return false;
    }

    rec1.t = rec1.t.max(0.0);

    let ray_length = ray.dir.norm();
    let dis_inside_boundary = (rec2.t - rec1.t) * ray_length;
    let hit_distance = self.neg_inv_density * rand_range(EPS, 1.0 - EPS).log(E);

    if hit_distance > dis_inside_boundary {
      return false;
    }

    rec.t = rec1.t + hit_distance / ray_length;
    rec.p = ray.at(rec.t);


    rec.mat = self.phase_function.clone();
    // rec.normal and front_face is aribitrary
    true
  }

  fn to_object(self) -> Object {
      Arc::new(self)
  }

  fn bounding_box(&self) -> crate::Aabb {
      self.boundary.bounding_box()
  }
}