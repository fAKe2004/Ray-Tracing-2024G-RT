const HEIGHT_PARTITION: usize = 10; // multithreading parameters
const WIDTH_PARTITION: usize = 10;
const THREAD_LIMIT: usize = 16;

use super::EPS;

use super::vec3::{*};
use super::ray::{*};
use super::hittable::{*};
use super::color::{*};
use super::color::ColorType;
use super::utility::{*};
use super::interval::{*};
use super::INFINITY;

use std::sync::{Arc, Mutex, Condvar};
use crossbeam::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use image::{ImageBuffer, RgbImage}; 
use indicatif::{ProgressBar, ProgressStyle};

pub struct Camera {
  pub aspect_ratio: f64, // Ratio of image width over height
  pub image_width: usize,
  pub sample_per_pixel: usize, // Count of random samples for each pixel
  pub max_ray_depth: usize, // Maximum number of ray bounces into scene
  pub vfov: f64, // Vertical view angle (field of view) in degree
  pub lookfrom: Point3,
  pub lookat: Point3,
  pub vup: Vec3,
  pub defocus_angle: f64, // Variation angle of rays through each pixel
  pub focus_dist: f64,  // Distance from camera lookfrom point to plane of perfect focus

  image_height: usize,
  pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
  center: Point3, 
  pixel00_loc: Point3, // Location of pixel 0, 0
  pixel_delta_u: Vec3, // Offset to pixel to the right
  pixel_delta_v: Vec3, // Offset to pixel below
  u: Vec3,
  v: Vec3,
  w: Vec3,
  defocus_disk_u: Vec3, // Defocus disk horizontal radius
  defocus_disk_v: Vec3, // Defocus disk vertical radius
}

impl Camera {
  pub fn new(
    aspect_ratio: f64, 
    image_width: usize, 
    sample_per_pixel: usize, 
    max_ray_depth: usize, 
    vfov: f64, 
    lookfrom: Point3, 
    lookat: Point3, 
    vup: Vec3,
    defocus_angle: f64,
    focus_dist: f64
  ) -> Self {
    let mut cam = Camera {
      aspect_ratio,
      image_width,
      sample_per_pixel,
      max_ray_depth, 
      vfov,
      lookfrom,
      lookat,
      vup,
      defocus_angle,
      focus_dist,
      image_height: 0,
      pixel_samples_scale: 0.0,
      center: Point3::zero(),
      pixel00_loc: Point3::zero(),
      pixel_delta_u: Vec3::zero(),
      pixel_delta_v: Vec3::zero(),
      u: Vec3::zero(),
      v: Vec3::zero(),
      w: Vec3::zero(),
      defocus_disk_u: Vec3::zero(),
      defocus_disk_v: Vec3::zero(),
    };
    cam.initialize();
    cam
  }


  // private function 
  fn initialize(&mut self) {
     // Camera
    self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
    self.pixel_samples_scale = 1.0 / self.sample_per_pixel as f64;
    
    self.center = self.lookfrom;

    // Determine viewport dimensions.
    let theta = degrees_to_radians(self.vfov);
    let h = (theta / 2.0 as f64).tan();
    let viewport_height = 2.0 * h * self.focus_dist;
    let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

    // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
    self.w = (self.lookfrom - self.lookat).normalize();
    self.u = Vec3::cross(&self.vup, &self.w).normalize();
    self.v = Vec3::cross(&self.w, &self.u);

    
    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = viewport_width * self.u;
    let viewport_v = viewport_height * -self.v; 

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    self.pixel_delta_u = viewport_u / self.image_width as f64;
    self.pixel_delta_v = viewport_v / self.image_height as f64;
    
    // Calculate the location of the upper left pixel.
    let viewport_upper_left = self.center
      - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
    self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    /*
      layout:
      00 -> u
      |
      V
      v
      looking into w
    */

    // Calculate the camera defocus disk basis vectors.
    let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();

    self.defocus_disk_u = self.u * defocus_radius;
    self.defocus_disk_v = self.v * defocus_radius;
  }

  // Return a ray pointing from camera to pixel (i, j) where exact coordinates is randomly sampled.
  fn get_ray(&self, i: usize, j: usize) -> Ray {
    let offset = Self::sample_square();
    let pixel_sample_coord = self.pixel00_loc + 
      (i as f64 + offset.x) * self.pixel_delta_u + 
      (j as f64 + offset.y) * self.pixel_delta_v;

    let ray_origin = if self.defocus_angle <= 0.0 {
        self.center 
      } else {
        self.defocus_disk_sample()
      };
    let ray_direction = pixel_sample_coord - ray_origin;
    Ray::new(ray_origin, ray_direction) 
  }
 

  // Return a random ([-0.5, 0.5], [-0.5, 0.5], 0) Vec3
  fn sample_square() -> Vec3 {
    static deviation: f64 = 0.5;
    Vec3::new(rand_range(-deviation, deviation), rand_range(-deviation, deviation), 0.0)
  }

  fn defocus_disk_sample(&self) -> Point3 {
    let p = Vec3::rand_in_unit_disk();
    self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
  }

  fn ray_color(&self, ray: &Ray, depth: usize, world: &Object) -> ColorType {
    if depth >= self.max_ray_depth { // ray tracing depth exceeds limit // note that my depth is incremental, which is different from the textbook
      return ColorType::zero();
    }

    let mut rec = HitRecord::default();
    if world.hit(ray, Interval::new(EPS /* fix shadow acne */, INFINITY), &mut rec) {
      let mut scattered = Ray::default();
      let mut attenuation = ColorType::zero();
      // let dir = Vec3::rand_on_hemisphere(rec.normal) + rec.normal; // Lambertian Reflection
      // 0.5 * self.ray_color(&Ray::new(rec.p, dir), depth + 1, world)
      if rec.mat.scatter(ray, &rec, &mut attenuation, &mut scattered) {
        attenuation.elemul(&self.ray_color(&scattered, depth + 1, world))
      } else {
        ColorType::zero()
      }
    } else {
        let unit_direction = ray.dir.normalize();
        let a = 0.5*(unit_direction.y + 1.0);
        ColorType::new(1.0, 1.0, 1.0) * (1.0 - a) + ColorType::new(0.5, 0.7, 1.0) * a
    }
  }






  // Multithread mechanism -> Partition into fine granularity (with WIDTH_PARTITION * HEIGHT_PARTITION sub-tasks), and only let THREAD_LIMIT threads run at the same time.
  pub fn render(&self, world: &Object) -> RgbImage { 
    let mut img: RgbImage = ImageBuffer::new(self.image_width as u32, self.image_height as u32);

    println!("[Render progress]:");
    let bar = get_ProgressBar(self.image_height, self.image_width);
    let bar_wrapper = Arc::new(&bar);

    let camera = Arc::new(self.clone());
    let world = Arc::new(world);
    let img_mtx = Arc::new(Mutex::new(&mut img));
    
    thread::scope(move |thd|{
      let thread_count = Arc::new(AtomicUsize::new(0));
      let thread_number_controller = Arc::new(Condvar::new());
      
      let chunk_height = (self.image_height + HEIGHT_PARTITION - 1) / HEIGHT_PARTITION;
      let chunk_width = (self.image_width + WIDTH_PARTITION - 1) / WIDTH_PARTITION;
      for j in 0..HEIGHT_PARTITION {
        for i in 0..WIDTH_PARTITION {
          let lock_for_condv = Mutex::new(false);
          while !(thread_count.load(Ordering::SeqCst) < THREAD_LIMIT) { // outstanding thread number control
            thread_number_controller.wait(lock_for_condv.lock().unwrap()).unwrap();
          }

          
          let bar = Arc::clone(&bar_wrapper);

          let camera = Arc::clone(&camera);
          let world = Arc::clone(&world);
          let img_mtx = Arc::clone(&img_mtx);
          
          let thread_count = Arc::clone(&thread_count);
          let thread_number_controller = Arc::clone(&thread_number_controller);

          thread_count.fetch_add(1, Ordering::SeqCst);
          bar.set_message(format!("|{} threads outstanding|", thread_count.load(Ordering::SeqCst))); // move out of thread, so that its sequential with thread number control code

          let _ = thd.spawn(move |_| {
            camera.render_sub(&world, &img_mtx, &bar, 
              i * chunk_width, (i + 1) * chunk_width, 
              j * chunk_height, (j + 1) * chunk_height);
            // println!("subtask ({}, {}) done", i, j);
            thread_count.fetch_sub(1, Ordering::SeqCst);
            bar.set_message(format!("|{} threads outstanding|", thread_count.load(Ordering::SeqCst)));
            thread_number_controller.notify_one();
          });

        }
      }
    }).unwrap();

    bar.finish();
    img
  }
  
  pub fn render_sub(&self, world: &Object, img_mtx: &Mutex<&mut RgbImage>, bar: &ProgressBar, x_min: usize, x_max: usize, y_min: usize, y_max: usize) {
    let x_min = x_min.max(0);
    let y_min = y_min.max(0);
    let x_max = x_max.min(self.image_width);
    let y_max = y_max.min(self.image_height);

    let mut buff: Vec<Vec<ColorType>> = vec![vec![ColorType::zero(); y_max - y_min]; x_max - x_min];
    for j in y_min..y_max {
        for i in x_min..x_max {
          let mut pixel_color = ColorType::zero();
          for _ in 0..self.sample_per_pixel {
            let ray = self.get_ray(i, j);
            pixel_color += self.ray_color(&ray, 0 as usize, &world);
          }
          pixel_color *= self.pixel_samples_scale;

          buff[i - x_min][j - y_min] = pixel_color;
          // bar.inc(1); // fact: bar.inc 相当慢，脱了速度
        }
        bar.inc((x_max - x_min) as u64);
      }
      let mut img = img_mtx.lock().unwrap();
      for j in y_min..y_max {
        for i in x_min..x_max {
          write_color_01(buff[i - x_min][j - y_min], &mut img, i, j);
        }
      }
  }


}

impl Clone for Camera {
  fn clone(&self) -> Self {
    Camera {
      ..*self
    }
  }
}