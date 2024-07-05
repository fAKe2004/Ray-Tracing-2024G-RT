
// hittable and hittable list
use super::vec3::{*};
use super::ray::{*};
use super::interval::{*};
use super::material::{*};
use std::sync::Arc;


pub struct HitRecord {
  pub p: Point3,
  pub normal: Vec3, // expected to be unit vector
  pub mat: Material,
  pub t: f64,
  pub front_surface: bool
}


// Hittable Trait
pub trait Hittable: Send + Sync{
  fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
  fn to_object(self) -> Object;
}

// use Arc::new, instead of Object::new btw
pub type Object = Arc<dyn Hittable + Send + Sync>; // Shared Ptr




impl HitRecord {
  pub fn new(p: Point3, normal: Vec3, mat: Material, t: f64, front_surface: bool) -> Self {
    HitRecord {
      p,
      normal: if normal.near_zero() { normal } else { normal.normalize() },
      mat,
      t,
      front_surface,
    }
  }

  pub fn new_from_ray_and_outward_normal(ray: &Ray, outward_normal: Vec3, mat: Material, t: f64) -> Self{
    let front_surface = ray.dir.dot(&outward_normal) < 0.0;
    let outward_normal = outward_normal.normalize();
    
    HitRecord {
      p: ray.at(t),
      normal: if front_surface {outward_normal} else {-outward_normal},
      mat,
      t,
      front_surface,
    }
  }

  pub fn default() -> Self {
    Self::new(Point3::zero(), Vec3::zero(), Arc::new(DefaultMaterial::new()) , 0.0, false)
  }
}

impl Clone for HitRecord {
  fn clone(&self) -> Self {
    HitRecord {
      p: self.p,
      normal: self.normal,
      mat: self.mat.clone(),
      t: self.t,
      front_surface: self.front_surface,
    }
  }
}
// impl Copy for HitRecord {
// }


pub struct HittableList {
  pub objects: Vec<Object>
}

impl HittableList {
  pub fn new(objects :Vec<Object>) -> Self {
    HittableList {
      objects,
    }
  }

  pub fn default() -> Self {
    HittableList {
      objects: Vec::default(),
    }    
  }
  pub fn clear(&mut self) {
    self.objects = Vec::default();
  }

  pub fn add(&mut self, object : Object) {
    self.objects.push(object);
  }

  // convert into Object(aka. Arc<dyn Hittable>)
  // pub fn to_object(&self) -> Object {
  //   Arc::new(*self)
  // }
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
}