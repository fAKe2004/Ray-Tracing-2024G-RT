use crate::utility::{*};
use crate::vec3::{*};
use crate::color::{*};
use crate::perlin::{*};
use crate::EPS;
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




// ImageTexture (borrowed from Games101)

use nalgebra::Vector3;
use opencv::core::{MatTraitConst, VecN};
use opencv::imgcodecs::{imread, IMREAD_COLOR};

pub struct ImageTexture {
    pub img_data: opencv::core::Mat,
    pub width: usize,
    pub height: usize,
}


pub const bilinear_coloring: bool = true;

impl ImageTexture {
    pub fn new(path: &str) -> Self {
        let img_data = imread(path, IMREAD_COLOR).expect("ImageTexture: Image reading error!");
        let width = img_data.cols() as usize;
        let height = img_data.rows() as usize;
        ImageTexture {
            img_data,
            width,
            height,
        }
    }

    pub fn get_color_bilinear(&self, mut u: f64, mut v: f64) -> ColorType {
        if u < 0.0 + EPS { u = 0.0; }
        if u > 1.0 - EPS { u = 1.0; }
        if v < 0.0 + EPS { v = 0.0; }
        if v > 1.0 - EPS { v = 1.0; }

        // let mut u_img = u * (self.width - 1) as f64;
        let mut u_img = u * (self.width - 1) as f64;
        let mut v_img = (1.0 - v) * (self.height - 1) as f64;

        let u1 = u_img as i32;
        let u2 = (u1 + 1).min((self.width - 1) as i32);
        let v1 = v_img as i32;
        let v2 = (v1 + 1).min((self.height - 1) as i32);
        let color11: Vector3<f64> = convert_U8VecN_to_Vector3f64(self.img_data.at_2d(v1, u1).unwrap());
        let color12: Vector3<f64> = convert_U8VecN_to_Vector3f64(self.img_data.at_2d(v1, u2).unwrap());
        let color21: Vector3<f64> = convert_U8VecN_to_Vector3f64(self.img_data.at_2d(v2, u1).unwrap());
        let color22: Vector3<f64> = convert_U8VecN_to_Vector3f64(self.img_data.at_2d(v2, u2).unwrap());

        let color1: Vector3<f64> = (v2 as f64 - v_img) * color11 + (v_img - v1 as f64) * color21;
        let color2: Vector3<f64> = (v2 as f64 - v_img) * color12 + (v_img - v1 as f64) * color22;
        let color : Vector3<f64> = (u2 as f64 - u_img) * color1 + (u_img - u1 as f64) * color2;

        let color_scale = 1.0 / 255.0;
        let gamma_color = ColorType::new(
          color_scale * color[2] as f64, 
          color_scale * color[1] as f64, 
          color_scale * color[0] as f64);

        // correction from image gamma space to linear space
        gamma_to_linear_ColorType(gamma_color)
    }
}

unsafe impl Send for ImageTexture {}
unsafe impl Sync for ImageTexture {}

impl TextureTrait for ImageTexture {
  fn value(&self, u: f64, v: f64, p: Point3) -> ColorType {
    self.get_color_bilinear(u, v)
  }
  fn to_texture(self) -> Texture {
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
  // fn value(&self, u: f64, v: f64, p: Point3) -> ColorType {
  //   ColorType::ones() * (1.0 + self.noise.noise(self.scale * p)) / 2.0
  // }
  // fn value(&self, u: f64, v: f64, p: Point3) -> ColorType {
  //   ColorType::ones() * self.noise.turb(p, 7)
  // }

  fn value(&self, u: f64, v: f64, p: Point3) -> ColorType {
    ColorType::ones() / 2.0 * (1.0 + (
      self.scale * p.z + 10.0 * self.noise.turb(p, 7))
      .sin()
    )
  }

  fn to_texture(self) -> Texture {
    Arc::new(self)
  }
}