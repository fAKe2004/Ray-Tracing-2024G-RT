
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
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attunation: &mut ColorType, scattered: &mut Ray) -> bool;
  fn to_material(self) ->
 Material;
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
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attunation: &mut ColorType, scattered: &mut Ray) -> bool {
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
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attunation: &mut ColorType, scattered: &mut Ray) -> bool {
    let mut scatter_dircton = rec.normal + Vec3::rand_unit();

    if scatter_dircton.near_zero() { // to handle zero vector error
      scatter_dircton = rec.normal
    }

    *scattered = Ray::new(rec.p, scatter_dircton, ray_in.tm);
    *attunation = self.tex.value(rec.u, rec.v, rec.p);
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
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attunation: &mut ColorType, scattered: &mut Ray) -> bool {
    let mut reflected = Vec3::reflect(ray_in.dir, rec.normal);
    reflected = reflected.normalize() + (self.fuzz * Vec3::rand_unit());
    *scattered = Ray::new(rec.p, reflected, ray_in.tm);
    *attunation = self.albedo;
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
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attunation: &mut ColorType, scattered: &mut Ray) -> bool {
    *attunation = ColorType::ones();
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


pub struct NoiseTexture {
  noise: Perlin,
  scale: f64,
}

impl NoiseTexture {
  pub fn new(scale: f64) -> Self{
    NoiseTexture {
      noise: Perlin::new(),
      scale,
    }
  }
}

impl TextureTrait for NoiseTexture {
  fn value(&self, u: f64, v: f64, p: Point3) -> ColorType {
    ColorType::ones() * self.noise.noise(self.scale * p)
  }

  fn to_texture(self) -> Texture {
    Arc::new(self)
  }
}