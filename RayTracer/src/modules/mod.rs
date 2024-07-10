
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;
pub const EPS: f64 = 1e-4;
pub const E: f64 = std::f64::consts::E;
pub const GAMMA_COEFFICIENT: f64 = 2.0;



pub use std::sync::Arc;

pub mod utility;
pub mod color;
pub mod ray;
pub mod vec3;
pub mod hittable;
pub mod sphere;
pub mod interval;
// pub mod camera;
pub mod camera_multithreading;
pub mod material;
pub mod aabb;
pub mod bvh;
pub mod texture;
pub mod perlin;
pub mod planar;
pub mod constant_medium;

pub use utility::{*};
pub use color::{*};
pub use ray::{*};
pub use hittable::{*};
pub use vec3::{*};
pub use sphere::{*};
pub use interval::{*};
// pub use camera::{*};
pub use camera_multithreading::{*};
pub use material::{*};
pub use aabb::{*};
pub use bvh::{*};
pub use texture::{*};
pub use perlin::{*};
pub use planar::{*};
pub use constant_medium::{*};
