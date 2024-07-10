
use indicatif::ProgressIterator;

use crate::vec3::{*};
use crate::ray::{*};
use crate::interval::{*};
use crate::hittable::{*};
use crate::color::{*};
use crate::utility::{*};
use crate::texture::{*};
use crate::perlin::{*};

use std::sync::Arc;

pub trait MaterialTrait {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut ColorType, scattered: &mut Ray) -> bool {
    false
  }
  fn emitted(&self, u: f64, v: f64, p: Point3) -> ColorType {
    ColorType::zero()
  }
  fn to_material(self) -> Material;
}

pub type Material = Arc<dyn MaterialTrait + Sync + Send>;




pub struct DefaultMaterial {
}
impl DefaultMaterial {
  pub fn new() -> Self {
    DefaultMaterial {
    }
  }
}

impl MaterialTrait for DefaultMaterial {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut ColorType, scattered: &mut Ray) -> bool {
    false
  }
  fn to_material(self) ->
   Material {
      Arc::new(self)
  }
}

// Lambertian
pub struct Lambertian{
  tex: Texture,
}

impl Lambertian {
  pub fn new(tex: Texture) -> Self {
    Lambertian {
      tex,
    }
  }

  pub fn new_by_color(albedo: ColorType) -> Self {
    Lambertian {
      tex: SolidColor::new(albedo).to_texture(),
    }
  }
}

impl MaterialTrait for Lambertian {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut ColorType, scattered: &mut Ray) -> bool {
    let mut scatter_dircton = rec.normal + Vec3::rand_unit();

    if scatter_dircton.near_zero() { // to handle zero vector error
      scatter_dircton = rec.normal
    }

    *scattered = Ray::new(rec.p, scatter_dircton, ray_in.tm);
    *attenuation = self.tex.value(rec.u, rec.v, rec.p);
    true
  }
  fn to_material(self) ->
  Material {
     Arc::new(self)
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

impl MaterialTrait for Metal {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut ColorType, scattered: &mut Ray) -> bool {
    let mut reflected = Vec3::reflect(ray_in.dir, rec.normal);
    reflected = reflected.normalize() + (self.fuzz * Vec3::rand_unit());
    *scattered = Ray::new(rec.p, reflected, ray_in.tm);
    *attenuation = self.albedo;
    Vec3::dot(&scattered.dir, &rec.normal) > 0.0
  }
  fn to_material(self) ->
  Material {
     Arc::new(self)
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
  pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0*r0;
    return r0 + (1.0 as f64 - r0) * (1.0 as f64 - cosine).powf(5.0);
  }
}

impl MaterialTrait for Dielectric {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut ColorType, scattered: &mut Ray) -> bool {
    *attenuation = ColorType::ones();
    let ratio = if rec.front_surface { 1.0 / self.refraction_index } else { self.refraction_index };
    
    let unit_direction =  ray_in.dir.normalize();
    let cos_theta = -unit_direction.dot(&rec.normal).min(1.0);
    let sin_theta = (1.0 as f64 - cos_theta * cos_theta).sqrt();

    let cannot_refract = ratio * sin_theta > 1.0;
    
    let scattered_direction = 
      if cannot_refract || Self::reflectance(cos_theta, self.refraction_index) > rand_01() {
        Vec3::reflect(ray_in.dir.normalize(), rec.normal)
      } else {
        Vec3::refract(ray_in.dir.normalize(), rec.normal, ratio)
      };

    *scattered = Ray::new(rec.p, scattered_direction, ray_in.tm);
    true
  }
  fn to_material(self) ->
  Material {
     Arc::new(self)
  }
}

pub struct DiffuseLight  {
  tex: Texture,
}

impl DiffuseLight {
  pub fn new(tex: Texture) -> Self {
    Self {
      tex,
    }
  }
  pub fn new_by_color(emit: ColorType) -> Self {
    Self::new(SolidColor::new(emit).to_texture())
  }
}

impl MaterialTrait for DiffuseLight {
  fn emitted(&self, u: f64, v: f64, p: Point3) -> ColorType {
      self.tex.value(u, v, p)
  }
  fn to_material(self) -> Material {
      Arc::new(self)
  }
}


pub struct Isotropic  {
  tex: Texture,
}

impl Isotropic {
  pub fn new(tex: Texture) -> Self {
    Self {
      tex,
    }
  }
  pub fn new_by_color(emit: ColorType) -> Self {
    Self::new(SolidColor::new(emit).to_texture())
  }
}

impl MaterialTrait for Isotropic {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut ColorType, scattered: &mut Ray) -> bool {
    *scattered = Ray::new(rec.p, Vec3::rand_unit(), ray_in.tm);
    *attenuation = self.tex.value(rec.u, rec.v, rec.p);
    true
  }
  fn to_material(self) -> Material {
      Arc::new(self)
  }
}
