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

// Lambertian
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


// Metal
pub struct Metal {
  albedo: ColorType,
  fuzz: f64,
}

impl Metal {
  pub fn new(albedo: ColorType, fuzz: f64) -> Self {
    Metal {
      albedo,
      fuzz: (1.0 as f64).min(fuzz),
    }
  }
}

impl Scatter for Metal {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attunation: &mut ColorType, scattered: &mut Ray) -> bool {
    let mut reflected = Vec3::reflect(ray_in.dir, rec.normal);
    reflected = reflected.normalize() + (self.fuzz * Vec3::rand_unit());
    *scattered = Ray::new(rec.p, reflected);
    *attunation = self.albedo;
    Vec3::dot(&scattered.dir, &rec.normal) > 0.0
  }
}



// Dielectric
pub struct Dielectric {
  refraction_index: f64,
}

impl Dielectric {
  pub fn new(refraction_index: f64) -> Self {
    Dielectric {
      refraction_index,
    }
  }
}

impl Scatter for Dielectric {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attunation: &mut ColorType, scattered: &mut Ray) -> bool {
    *attunation = ColorType::ones();
    let ratio = if rec.front_surface { 1.0 / self.refraction_index } else { self.refraction_index };
    let refracted = Vec3::refract(ray_in.dir.normalize(), rec.normal, ratio);

    *scattered = Ray::new(rec.p, refracted);
    true
  }
}