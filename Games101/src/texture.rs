#![allow(warnings)]
use nalgebra::{Vector3};

use opencv::core::{MatTraitConst, VecN};
use opencv::imgcodecs::{imread, IMREAD_COLOR};

pub struct Texture {
    pub img_data: opencv::core::Mat,
    pub width: usize,
    pub height: usize,
}


pub const bilinear_coloring: bool = true;

impl Texture {
    pub fn new(name: &str) -> Self {
        let img_data = imread(name, IMREAD_COLOR).expect("Image reading error!");
        let width = img_data.cols() as usize;
        let height = img_data.rows() as usize;
        Texture {
            img_data,
            width,
            height,
        }
    }

    pub fn get_color(&self, mut u: f64, mut v: f64) -> Vector3<f64> {
        match bilinear_coloring {
            false => self.get_color_basic(u, v),
            true => self.get_color_bilinear(u, v),
        }
    }
    pub fn get_color_basic(&self, mut u: f64, mut v: f64) -> Vector3<f64> {

        const eps: f64 = 1e-6;
        if u < 0.0 { u = 0.0; }
        if u > 1.0 { u = 1.0; }
        if v < 0.0 { v = 0.0; }
        if v > 1.0 { v = 1.0; }

        let mut u_img = u * self.width as f64;
        let mut v_img = (1.0 - v) * self.height as f64;

        let color: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32, u_img as i32).unwrap();

        Vector3::new(color[2] as f64, color[1] as f64, color[0] as f64) // note that color [2..0]
    }

    pub fn get_color_bilinear(&self, mut u: f64, mut v: f64) -> Vector3<f64> {
            // 在此实现双线性插值函数, 并替换掉get_color

        const eps: f64 = 1e-6;
        if u < 0.0 { u = 0.0; }
        if u > 1.0 { u = 1.0; }
        if v < 0.0 { v = 0.0; }
        if v > 1.0 { v = 1.0; }

        let mut u_img = u * (self.width - 1) as f64;
        let mut v_img = (1.0 - v) * (self.height - 1) as f64;

        let u1 = u_img as i32;
        let u2 = u1 + 1;
        let v1 = v_img as i32;
        let v2 = v1 + 1;
        let color11: Vector3<f64> = convert_U8VecN_to_Vector3f64(self.img_data.at_2d(v1, u1).unwrap());
        let color12: Vector3<f64> = convert_U8VecN_to_Vector3f64(self.img_data.at_2d(v1, u2).unwrap());
        let color21: Vector3<f64> = convert_U8VecN_to_Vector3f64(self.img_data.at_2d(v2, u1).unwrap());
        let color22: Vector3<f64> = convert_U8VecN_to_Vector3f64(self.img_data.at_2d(v2, u2).unwrap());

        let color1: Vector3<f64> = (v2 as f64 - v_img) * color11 + (v_img - v1 as f64) * color21;
        let color2: Vector3<f64> = (v2 as f64 - v_img) * color12 + (v_img - v1 as f64) * color22;
        let color : Vector3<f64> = (u2 as f64 - u_img) * color1 + (u_img - u1 as f64) * color2;

        Vector3::new(color[2] as f64, color[1] as f64, color[0] as f64)
    }
}


fn convert_U8VecN_to_Vector3f64(vec : &VecN<u8, 3>) -> Vector3<f64> {
    Vector3::new(vec[0] as f64, vec[1] as f64, vec[2] as f64)
}