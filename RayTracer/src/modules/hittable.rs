
use crate::{degrees_to_radians, INFINITY};
// hittable and hittable list
use crate::vec3::{*};
use crate::ray::{*};
use crate::interval::{*};
use crate::material::{*};
use crate::aabb::{*};
use crate::bvh::{*};
use std::sync::Arc;


pub struct HitRecord {
  pub p: Point3,
  pub normal: Vec3, // expected to be unit vector
  pub mat: Material,
  pub t: f64,
  pub u: f64, // texture coord
  pub v: f64,
  pub front_surface: bool
}


// Hittable Trait
pub trait Hittable: Send + Sync{
  fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
  
  fn to_object(self) -> Object;

  fn bounding_box(&self) -> Aabb;

}

// use Arc::new, instead of Object::new btw
pub type Object = Arc<dyn Hittable + Send + Sync>; // Shared Ptr




impl HitRecord {
  pub fn new(p: Point3, normal: Vec3, mat: Material, t: f64, front_surface: bool, u: f64, v: f64) -> Self {
    HitRecord {
      p,
      normal: if normal.near_zero() { normal } else { normal.normalize() },
      mat,
      t,
      u,
      v,
      front_surface,
    }
  }

  pub fn new_from_ray_and_outward_normal(ray: &Ray, outward_normal: Vec3, mat: Material, t: f64, u: f64, v: f64) -> Self{
    let front_surface = ray.dir.dot(&outward_normal) < 0.0;
    let outward_normal = outward_normal.normalize();
    
    HitRecord {
      p: ray.at(t),
      normal: if front_surface {outward_normal} else {-outward_normal},
      mat,
      t,
      u,
      v,
      front_surface,
    }
  }

  pub fn default() -> Self {
    Self::new(Point3::zero(), Vec3::zero(), Arc::new(DefaultMaterial::new()) , 0.0, false, 0.0, 0.0)
  }
}

impl Clone for HitRecord {
  fn clone(&self) -> Self {
    HitRecord {
      mat: self.mat.clone(),
      ..*self
    }
  }
}
// impl Copy for HitRecord {
// }



// HittableList
pub struct HittableList {
  pub objects: Vec<Object>,
  bbox: Aabb,
}

impl HittableList {
  pub fn new(objects :Vec<Object>) -> Self {
    let mut bbox = Aabb::default();
    for iter in &objects {
      bbox = Aabb::new_by_aabb(bbox, iter.bounding_box());
    }
    HittableList {
      objects,
      bbox,
    }
  }

  pub fn default() -> Self {
    HittableList {
      objects: Vec::default(),
      bbox: Aabb::default(),
    }    
  }
  pub fn clear(&mut self) {
    self.objects = Vec::default();
  }

  pub fn add(&mut self, object : Object) {
    self.bbox = Aabb::new_by_aabb(self.bbox, object.bounding_box());
    self.objects.push(object);
  }

  pub fn to_bvh(&mut self) -> Object {
    BvhNode::new(self).to_object()
  }
}

impl Hittable for HittableList {
  fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
    let mut tmp_rec = HitRecord::default();
    let mut hit_anything = false;
    let mut closest_root = ray_t.max;

    for object in &self.objects {
      if object.hit(ray, Interval::new(ray_t.min, closest_root), &mut tmp_rec) {
        hit_anything = true;
        closest_root = tmp_rec.t;
        *rec = tmp_rec.clone();
      }
    }

    hit_anything
  }
  fn to_object(self) -> Object {
      Arc::new(self)
  }
  fn bounding_box(&self) -> Aabb {
      self.bbox
  }
}


// Instances
pub struct Translate {
  object: Object,
  offset: Vec3,
  bbox: Aabb,
}

impl Translate {
  pub fn new(object: Object, offset: Vec3) -> Self {
    let bbox = object.bounding_box() + offset;
    Self {
      object,
      offset,
      bbox,
    }
  }
}

impl Hittable for Translate {
  fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
    let offset_ray = Ray::new(ray.orig - self.offset, ray.dir, ray.tm);

    if self.object.hit(&offset_ray, ray_t, rec) {
      rec.p += self.offset;
      true
    } else {
      false
    }
  }
  fn bounding_box(&self) -> Aabb {
    self.bbox
  }
  fn to_object(self) -> Object {
    Arc::new(self)
  }
}

pub struct RotateY {
  object: Object,
  sin_theta: f64,
  cos_theta: f64,
  bbox: Aabb,
}

impl RotateY {
  pub fn new(object: Object, angle: f64) -> Self {
    let radians = degrees_to_radians(angle);
    let sin_theta = radians.sin();
    let cos_theta = radians.cos();
    let bbox = object.bounding_box();

    let mut rotate_y = Self {
      object,
      sin_theta,
      cos_theta,
      bbox: Aabb::default(),
    };

    let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
    let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

    for i in 0..2 {
      for j in 0..2 {
        for k in 0..2 {
          let x = if i == 1 { bbox.x.max } else { bbox.x.min};
          let y = if j == 1 { bbox.y.max } else { bbox.y.min};
          let z = if k == 1 { bbox.z.max } else { bbox.z.min};
          
          let tester = rotate_y.rotate_pos(Vec3::new(x, y, z));
          for idx in 0..3 {
            min[idx] = min[idx].min(tester[idx]);
            max[idx] = min[idx].max(tester[idx]);
          }
        }
      }
    }
    let bbox = Aabb::new_by_point(min, max);
    rotate_y.bbox = bbox;
    rotate_y
  }

  fn rotate_neg(&self, v: Vec3) -> Vec3 {
    Vec3::new(
      self.cos_theta * v.x - self.sin_theta * v.z,
      v.y,
      self.sin_theta * v.x + self.cos_theta * v.z
    )
  }

  fn rotate_pos(&self, v: Vec3) -> Vec3 {
    Vec3::new(
      self.cos_theta * v.x + self.sin_theta * v.z,
      v.y,
      -self.sin_theta * v.x + self.cos_theta * v.z
    )
  }
}

impl Hittable for RotateY {
  fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
    let orig = self.rotate_neg(ray.orig);
    let dir = self.rotate_neg(ray.dir);

    let rotated_ray = Ray::new(orig, dir, ray.tm);

    if !self.object.hit(&rotated_ray, ray_t, rec) {
      return false;
    }

    rec.p = self.rotate_pos(rec.p);
    rec.normal = self.rotate_pos(rec.normal);
    true
  }

  fn to_object(self) -> Object {
    Arc::new(self)
  }

  fn bounding_box(&self) -> Aabb {
    self.bbox
  }
}