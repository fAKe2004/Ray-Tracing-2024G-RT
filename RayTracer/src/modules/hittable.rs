
// hittable and hittable list
use super::vec3::{*};
use super::ray::{*};
use super::interval::{*};
use std::rc::Rc;


pub struct HitRecord {
  pub p: Point3,
  pub normal: Vec3,
  pub t: f64,
  pub front_surface: bool
}


// Hittable Trait
pub trait Hittable {
  fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}





impl HitRecord {
  pub fn new(p: Point3, normal: Vec3, t: f64, front_surface: bool) -> Self {
    HitRecord {
      p,
      normal,
      t,
      front_surface,
    }
  }

  pub fn new_from_ray_and_outward_normal(ray: &Ray, outward_normal: Vec3, t: f64) -> Self{
    let front_surface = ray.dir.dot(&outward_normal) < 0.0;
    let outward_normal = outward_normal.normalize();
    
    HitRecord {
      p: ray.at(t),
      normal: if front_surface {outward_normal} else {-outward_normal},
      t,
      front_surface,
    }
  }

  pub fn default() -> Self {
    Self::new(Point3::zero(), Vec3::zero(), 0.0, false)
  }
}

impl Clone for HitRecord {
  fn clone(&self) -> Self {
    HitRecord {
      p: self.p,
      normal: self.normal,
      t: self.t,
      front_surface: self.front_surface,
    }
  }
}
impl Copy for HitRecord {
}

// use Rc::new, instead of Object::new btw
pub type Object = Rc<dyn Hittable>; // Shared Ptr

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

  // convert into Object(aka. Rc<dyn Hittable>)
  pub fn to_object(self) -> Rc<dyn Hittable> {
    let rc: Rc<dyn Hittable> = Rc::new(self);
    rc
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
        *rec = tmp_rec;
      }
    }

    hit_anything
  }
}
