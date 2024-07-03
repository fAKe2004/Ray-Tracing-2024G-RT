pub use super::vec3::{*};
pub use super::ray::{*};
pub use super::hittable::{*};

use image::RgbImage;

// color type
pub type ColorType = Vec3;


/// the multi-sample write_color() function
pub fn write_color_256(pixel_color: [u8; 3], img: &mut RgbImage, i: usize, j: usize) {
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    *pixel = image::Rgb(pixel_color);
    // Write the translated [0,255] value of each color component.
}

pub fn convert_ColorType_to_u8Array(pixel_color: ColorType) -> [u8; 3] {
    [(pixel_color.x * 255.999) as u8, (pixel_color.y * 255.999) as u8, (pixel_color.z * 255.999) as u8]
}

pub fn write_color_01(pixel_color: ColorType, img: &mut RgbImage, i: usize, j: usize) {
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());

    *pixel = image::Rgb(convert_ColorType_to_u8Array(pixel_color));
    // Write the translated [0,255] value of each color component.
}