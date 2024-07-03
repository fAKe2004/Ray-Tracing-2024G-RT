use crate::INFINITY;

pub struct Interval {
  pub min: f64,
  pub max: f64,
}

impl Interval {
  pub fn default() -> Self {
    Interval {
      min: -INFINITY,
      max: INFINITY,
    }
  }
  pub fn new(min: f64, max: f64) -> Self {
    Interval {
      min,
      max,
    }
  }

  pub fn size(&self) -> f64 {
    self.max - self.min
  }

  // in [min, max] (closed)
  pub fn contains(&self, x : f64) -> bool {
    self.min <= x && x <= self.max
  }

  // in (min, max) (open)
  pub fn surrends(&self, x : f64) -> bool {
    self.min < x && x < self.max
  }
}

impl Clone for Interval {
  fn clone(&self) -> Self {
    Interval {
      min: self.min,
      max: self.max,
    }
  }
}

impl Copy for Interval {
}