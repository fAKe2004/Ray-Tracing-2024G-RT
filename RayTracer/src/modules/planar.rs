use crate::utility::{*};
use crate::vec3::{*};
use crate::material::{*};
use crate::texture::{*};
use crate::aabb::{*};
use crate::hittable::{*};
use crate::interval::{*};
use crate::EPS;

use std::sync::{Arc};

pub trait Planar {
  fn is_interier(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool;
}


pub struct Quad {
  Q: Point3,
  u: Vec3,
  v: Vec3,
  w: Vec3,
  mat: Material,
  bbox: Aabb,
  normal: Vec3,
  D: f64, // constant for plane equation
}

impl Quad {
  pub fn new(Q: Point3, u: Vec3, v: Vec3, mat: Material) -> Self {
    let bbox_diagonal1 = Aabb::new_by_point(Q, Q + u + v);
    let bbox_diagonal2 = Aabb::new_by_point(Q + u, Q + v);
    let bbox = Aabb::new_by_aabb(bbox_diagonal1, bbox_diagonal2);
    let n = u.cross(&v);
    let normal = n.normalize();
    let D  = normal.dot(&Q);
    let w = n / n.norm_squared(); 
    Self {
      Q,
      u,
      v,
      w,
      mat,
      bbox,
      normal,
      D,
    }
  }
}

impl Clone for Quad {
  fn clone(&self) -> Self {
    Self {
      mat: self.mat.clone(),
      ..*self
    }
  }
}

impl Planar for Quad {
  fn is_interier(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool {
    let unit_interval = Interval::new(0.0, 1.0);

    if !unit_interval.contains(a) || !unit_interval.contains(b) {
      false
    } else {
      rec.u = a;
      rec.v = b;
      true
    }
  }
}

impl Hittable for Quad {
  fn hit(&self, ray: &crate::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
    let den = self.normal.dot(&ray.dir);
    if den.abs() < EPS {
      return false;
    }

    let t = (self.D - self.normal.dot(&ray.orig))/ den;
    if !ray_t.contains(t) {
      return false;
    }

    let intersection = ray.at(t);
    let planar_hitpt_vector = intersection - self.Q;
    let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
    let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

    
    if !self.is_interier(alpha, beta, rec) {
      return false;
    }
    
    *rec = HitRecord::new_from_ray_and_outward_normal(
      ray,
      self.normal,
      self.mat.clone(),
      t,
      rec.u, rec.v
    );

    true
  }
  fn to_object(self) -> Object {
    Arc::new(self)
  }
  fn bounding_box(&self) -> Aabb {
    self.bbox
  }
}

pub fn build_box(a: Point3, b: Point3, mat: Material) -> HittableList {
  let mut sides = HittableList::default();
  let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
  let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

  let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
  let dy = Vec3::new(0.0, max.y - min.y, 0.0);
  let dz = Vec3::new(0.0, 0.0, max.z - min.z);

  sides.add(Quad::new(
    Point3::new(min.x, min.y, max.z),
    dx, dy, 
    mat.clone()
    ).to_object()
  ); // front

  sides.add(Quad::new(
    Point3::new(max.x, min.y, max.z),
    -dz, dy, 
    mat.clone()
    ).to_object()
  ); // right

  sides.add(Quad::new(
    Point3::new(max.x, min.y, min.z),
    -dx, dy, 
    mat.clone()
    ).to_object()
  ); // back

  sides.add(Quad::new(
    Point3::new(min.x, min.y, min.z),
    dz, dy, 
    mat.clone()
    ).to_object()
  ); // left

  sides.add(Quad::new(
    Point3::new(min.x, max.y, max.z),
    dx, -dz, 
    mat.clone()
    ).to_object()
  ); // top

  sides.add(Quad::new(
    Point3::new(min.x, min.y, max.z),
    dx, dz, 
    mat.clone()
    ).to_object()
  ); // bottom
  sides
}