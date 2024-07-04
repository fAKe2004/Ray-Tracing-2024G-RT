
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;
pub const EPS: f64 = 1e-4;



pub use std::rc::Rc;

pub mod utility;
pub mod color;
pub mod ray;
pub mod vec3;
pub mod hittable;
pub mod sphere;
pub mod interval;
pub mod camera;
pub mod material;

pub use utility::{*};
pub use color::{*};
pub use ray::{*};
pub use hittable::{*};
pub use vec3::{*};
pub use sphere::{*};
pub use interval::{*};
pub use camera::{*};
pub use material::{*};