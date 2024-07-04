use super::vec3::{*};
use super::ray::{*};
use super::interval::{*};
use super::hittable::{*};
use super::color::{*};
use std::rc::Rc;

pub trait Scatter {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attunation: &mut ColorType, scattered: &mut Ray) -> bool;
}

pub type Material = Rc<dyn Scatter>;

pub struct DefaultMaterial {
}
impl DefaultMaterial {
  pub fn new() -> Self {
    DefaultMaterial {
    }
  }
}

impl Scatter for DefaultMaterial {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attunation: &mut ColorType, scattered: &mut Ray) -> bool {
    false
  }
}

pub struct Lambertian{
  albedo: ColorType,
}

impl Lambertian {
  pub fn new(albedo: ColorType) -> Self {
    Lambertian {
      albedo,
    }
  }
}

impl Scatter for Lambertian {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attunation: &mut ColorType, scattered: &mut Ray) -> bool {
    let mut scatter_dircton = rec.normal + Vec3::rand_unit();

    if scatter_dircton.near_zero() { // to handle zero vector error
      scatter_dircton = rec.normal
    }

    *scattered = Ray::new(rec.p, scatter_dircton);
    *attunation = self.albedo;
    true
  }
}

pub struct Metal {
  albedo: ColorType,
}

impl Metal {
  pub fn new(albedo: ColorType) -> Self {
    Metal {
      albedo,
    }
  }
}

impl Scatter for Metal {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attunation: &mut ColorType, scattered: &mut Ray) -> bool {
    let reflected = Vec3::reflect(ray_in.dir, rec.normal);
    *scattered = Ray::new(rec.p, reflected);
    *attunation = self.albedo;
    true
  }
}