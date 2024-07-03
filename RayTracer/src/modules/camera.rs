use super::vec3::{*};
use super::ray::{*};
use super::hittable::{*};
use super::color::{*};
use super::utility::{*};
use super::interval::{*};
use super::INFINITY;

use std::rc::Rc;
use image::{ImageBuffer, RgbImage}; 
use indicatif::{ProgressBar, ProgressStyle};

pub struct Camera {
  pub aspect_ratio: f64,
  pub image_width: usize,
  image_height: usize,
  center: Point3,
  pixel00_loc: Point3,
  pixel_delta_u: Vec3,
  pixel_delta_v: Vec3,
}

impl Camera {
  pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
    let mut cam = Camera {
      aspect_ratio,
      image_width,
      image_height: 0 as usize,
      center: Point3::zero(),
      pixel00_loc: Point3::zero(),
      pixel_delta_u: Vec3::zero(),
      pixel_delta_v: Vec3::zero(),
    };
    cam.initialize();
    cam
  }

  
  pub fn render(&self, world: &Object) -> RgbImage {
    let mut img: RgbImage = ImageBuffer::new(self.image_width as u32, self.image_height as u32);

    let quality = 60;
    let bar = get_ProgressBar(self.image_height, self.image_width);

    for j in 0..self.image_height {
      for i in 0..self.image_width {

          let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
          let ray_direction = pixel_center - self.center;
          // println!("at [{}, {}] is [{} {} {}]", i, j, ray_direction.x, ray_direction.y, ray_direction.z);
          let ray = Ray::new(self.center, ray_direction);
          let pixel_color = self.ray_color(&ray, &world);

          write_color_01(pixel_color, &mut img, i as usize, j as usize);

          bar.inc(1);
      }
    }
    bar.finish();
    img
  }
  

  // private function 
  fn initialize(&mut self) {
     // Camera
    self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
    
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

    self.center = Point3::new(0.0, 0.0, 0.0);    
    
    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    self.pixel_delta_u = viewport_u / self.image_width as f64;
    self.pixel_delta_v = viewport_v / self.image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = self.center
      - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    /*
      layout:
      00 -> u(x)
      |
      V
      v(-y)
      looking into -z
    */

  }

  fn ray_color(&self, ray: &Ray, world: &Object) -> ColorType {
    let mut rec = HitRecord::default();
    if world.hit(ray, Interval::new(0.0, INFINITY), &mut rec) {
        return 0.5 * (rec.normal + ColorType::ones());
    } else {
        let unit_direction = ray.dir.normalize();
        let a = 0.5*(unit_direction.y + 1.0);
        ColorType::new(1.0, 1.0, 1.0) * (1.0 - a) + ColorType::new(0.5, 0.7, 1.0) * a
    }
  }

}