use crate::utility::{*};
use crate::vec3::{*};
use crate::color::{*};
use std::sync::Arc;

pub trait TextureTrait {
  fn value(&self, u: f64, v: f64, p: Point3) -> ColorType;
  fn to_texture(self) -> Texture;
}

pub type Texture = Arc<dyn TextureTrait + Send + Sync>;

pub struct SolidColor {
  albedo: ColorType,
}

impl SolidColor {
  pub fn new(albedo : ColorType) -> Self {
    SolidColor {
      albedo,
    }
  }
}

impl TextureTrait for SolidColor {
  fn value(&self, u: f64, v: f64, p: Point3) -> ColorType {
    self.albedo
  }
  fn to_texture(self) -> Texture {
    Arc::new(self)
  }
}


pub struct CheckerTexture {
  inv_scale: f64,
  even: Texture,
  odd: Texture,
}

impl CheckerTexture {
  pub fn new(scale: f64, even: Texture, odd: Texture) -> Self {
    Self {
      inv_scale: 1.0 / scale,
      even,
      odd,
    }
  }
  pub fn new_by_color(scale: f64, even: ColorType, odd: ColorType) -> Self {
    Self::new(scale,
       SolidColor::new(even).to_texture(), SolidColor::new(odd).to_texture())
  }
}

impl TextureTrait for CheckerTexture {
  fn value(&self, u: f64, v: f64, p: Point3) -> ColorType {
    let x_int = (self.inv_scale * p.x).floor() as i32;
    let y_int = (self.inv_scale * p.y).floor() as i32;
    let z_int = (self.inv_scale * p.z).floor() as i32;

    let is_even = (x_int + y_int + z_int) % 2 == 0;

    if is_even { self.even.value(u, v, p) } else {self.odd.value(u, v, p) }
  }
  fn to_texture(self) -> Texture {
      Arc::new(self)
  }
}