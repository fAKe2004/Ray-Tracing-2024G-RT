use crate::{EPS, GAMMA_COEFFICIENT};

use crate::vec3::{*};
use crate::ray::{*};
use crate::hittable::{*};
use crate::interval::{*};

use image::{RgbImage};

// color type
pub type ColorType = Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.powf(1.0 / GAMMA_COEFFICIENT)
    } else {
        0.0
    }
}

pub fn linear_to_gamma_ColorType(pixel_color: ColorType) -> ColorType {
    ColorType::new(
        linear_to_gamma(pixel_color.x),
        linear_to_gamma(pixel_color.y),
        linear_to_gamma(pixel_color.z),
    )
}

// !!! 保证 texture 的材质不失真，要先把 gamma 转回 linear
// 效果见 Image5
pub fn gamma_to_linear(gamma_component: f64) -> f64{
    if gamma_component > 0.0 {
        gamma_component.powf(GAMMA_COEFFICIENT)
    } else {
        0.0
    }
}

pub fn gamma_to_linear_ColorType(pixel_color: ColorType) -> ColorType {
    ColorType::new(
        gamma_to_linear(pixel_color.x),
        gamma_to_linear(pixel_color.y),
        gamma_to_linear(pixel_color.z),
    )
}


/// the multi-sample write_color() function 
/// no gamma correction applied
pub fn write_color_256(pixel_color: [u8; 3], img: &mut RgbImage, i: usize, j: usize) {
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    *pixel = image::Rgb(pixel_color);
    // Write the translated [0,255] value of each color component.
}

pub fn convert_ColorType_to_u8Array(pixel_color: ColorType) -> [u8; 3] {
    let intensity: Interval = Interval::new(0.0, 1.0 - EPS);
    let (x, y, z) = (intensity.clamp(pixel_color.x), intensity.clamp(pixel_color.y), intensity.clamp(pixel_color.z));
    [(x * 256.0) as u8, (y * 256.0) as u8, (z * 256.0) as u8]
}

// write color in ColorType (range [0, 1)) with gamma correction
pub fn write_color_01(pixel_color: ColorType, img: &mut RgbImage, i: usize, j: usize) {
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());

    let pixel_color = linear_to_gamma_ColorType(pixel_color);

    *pixel = image::Rgb(convert_ColorType_to_u8Array(pixel_color));
    // Write the translated [0,255] value of each color component.
}