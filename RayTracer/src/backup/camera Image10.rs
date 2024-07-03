use super::EPS;

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
  pub aspect_ratio: f64, // Ratio of image width over height
  pub image_width: usize,
  pub sample_per_pixel: usize, // Count of random samples for each pixel
  pub max_ray_depth: usize, // Maximum number of ray bounces into scene
  image_height: usize,
  pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
  center: Point3, 
  pixel00_loc: Point3, // Location of pixel 0, 0
  pixel_delta_u: Vec3, // Offset to pixel to the right
  pixel_delta_v: Vec3, // Offset to pixel below
}

impl Camera {
  pub fn new(aspect_ratio: f64, image_width: usize, sample_per_pixel: usize, max_ray_depth: usize) -> Self {
    let mut cam = Camera {
      aspect_ratio,
      image_width,
      sample_per_pixel,
      max_ray_depth, 
      image_height: 0 as usize,
      pixel_samples_scale: 0 as f64,
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

    println!("[Render progress]:");
    let bar = get_ProgressBar(self.image_height, self.image_width);

    for j in 0..self.image_height {
      for i in 0..self.image_width {
        let mut pixel_color = ColorType::zero();
        for _ in 0..self.sample_per_pixel {
          let ray = self.get_ray(i, j);
          pixel_color += self.ray_color(&ray, 0 as usize, &world);
        }
        pixel_color *= self.pixel_samples_scale;
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
    self.pixel_samples_scale = 1.0 / self.sample_per_pixel as f64;
    
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

  // Return a ray pointing from camera to pixel (i, j) where exact coordinates is randomly sampled.
  fn get_ray(&self, i: usize, j: usize) -> Ray {
    let offset = Self::sample_square();
    let pixel_sample_coord = self.pixel00_loc + 
      (i as f64 + offset.x) * self.pixel_delta_u + 
      (j as f64 + offset.y) * self.pixel_delta_v;
    Ray::new(self.center, pixel_sample_coord- self.center) 
  }
 
  // Return a random ([-0.5, 0.5], [-0.5, 0.5], 0) Vec3
  fn sample_square() -> Vec3 {
    static deviation: f64 = 0.5;
    Vec3::new(rand_range(-deviation, deviation), rand_range(-deviation, deviation), 0.0)
  }

  fn ray_color(&self, ray: &Ray, depth: usize, world: &Object) -> ColorType {

    if depth >= self.max_ray_depth { // ray tracing depth exceeds limit // note that my depth is incremental, which is different from the textbook
      return ColorType::zero();
    }

    let mut rec = HitRecord::default();
    if world.hit(ray, Interval::new(EPS /* fix shadow acne */, INFINITY), &mut rec) {
        let dir = Vec3::rand_on_hemisphere(rec.normal) + rec.normal; // Lambertian Reflection
        0.5 * self.ray_color(&Ray::new(rec.p, dir), depth + 1, world)
    } else {
        let unit_direction = ray.dir.normalize();
        let a = 0.5*(unit_direction.y + 1.0);
        ColorType::new(1.0, 1.0, 1.0) * (1.0 - a) + ColorType::new(0.5, 0.7, 1.0) * a
    }
  }

}