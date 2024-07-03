
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
  degrees * PI / 180.0
}

pub use std::rc::Rc;

pub mod color;
pub mod ray;
pub mod vec3;
pub mod hittable;
pub mod sphere;

pub use color::{*};
pub use ray::{*};
pub use hittable::{*};
pub use vec3::{*};
pub use sphere::{*};
