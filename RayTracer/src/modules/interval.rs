use crate::INFINITY;
use std::ops::{Add};
pub struct Interval {
  pub min: f64,
  pub max: f64,
}

impl Interval {
  pub fn default() -> Self {
    Interval {
      min: INFINITY,
      max: -INFINITY,
    }
  }

  // construct [min, max] without adjusting order.
  pub fn new(min: f64, max: f64) -> Self {
    Interval {
      min,
      max,
    }
  }

  // automatically adjust order to min < max 
  pub fn new_adaptive(min: f64, max: f64) -> Self {
    Interval {
      min: min.min(max),
      max: max.max(min),
    }
  }

  pub fn new_overlap(i1: Interval, i2: Interval) -> Self {
    i1.overlap(i2)
  }

  pub fn new_union(i1: Interval, i2: Interval) -> Self {
    i1.union(i2)
  }

  pub fn size(&self) -> f64 {
    self.max - self.min
  }

  pub fn empty(&self) -> bool {
    self.size() <= 0.0
  }

  // in [min, max] (closed)
  pub fn contains(&self, x : f64) -> bool {
    self.min <= x && x <= self.max
  }

  // in (min, max) (open)
  pub fn surrounds(&self, x : f64) -> bool {
    self.min < x && x < self.max
  }

  pub fn clamp(&self, x : f64) -> f64 {
    if x < self.min {
      self.min
    } else if x > self.max {
      self.max
    } else {
      x
    }
  }

  pub fn expand(&self, delta: f64) -> Self {
    let padding = delta / 2.0;
    Interval::new(self.min - padding, self.max + padding)
  }

  pub fn overlap(&self, other: Interval) -> Self {
    Interval::new(self.min.max(other.min), self.max.min(other.max))
  }

  pub fn union(&self, other: Interval) -> Self {
    Interval::new(self.min.min(other.min), self.max.max(other.max))
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

impl Add<f64> for Interval {
  type Output = Interval;
  fn add(self, offset: f64) -> Interval{
    Interval::new(self.min + offset, self.max + offset)
  }
}


impl Add<Interval> for f64 {
  type Output = Interval;
  fn add(self, interval: Interval) -> Interval{
    interval + self
  }
}