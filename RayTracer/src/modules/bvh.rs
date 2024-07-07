use std::sync::Arc;
use std::cmp::Ordering;

use crate::aabb::{*};
use crate::utility::{*};
use crate::hittable::{*};
use crate::vec3::{*};
use crate::interval::{*};
use crate::ray::{*};


pub struct BvhNode {
  left: Object,
  right: Object,
  bbox: Aabb,
}

impl BvhNode {
  pub fn new(list: &mut HittableList) -> Self {
    let end = list.objects.len();
    Self::new_range(&mut list.objects, 0, end)
  }

  pub fn new_range(objects: &mut Vec<Object>, start: usize, end: usize) -> Self {
    let mut bbox = Aabb::default();
    for i in start..end {
      bbox = Aabb::new_by_aabb(bbox, objects[i].bounding_box());
    }

    let axis = bbox.longest_axis();

    let object_span = end - start;
    let left : Object;
    let right : Object;

    match object_span {
      1 => {
        left = objects[start].clone();
        right = objects[start].clone();  
      },
      2 => {
        left = objects[start].clone();
        right = objects[start + 1].clone();
      },
      _ => {
        objects[start..end].sort_by(|a, b| Self::box_compare(a, b, axis));

        let mid = (start + end) / 2;
        left = BvhNode::new_range(objects, start, mid).to_object();
        right = BvhNode::new_range(objects, mid, end).to_object();
      }
    };
    Self {
      left,
      right,
      bbox,
    }

  }
    
  fn box_compare(a: &Object, b: &Object, axis: usize) -> Ordering {
    let a_axis_interval = a.bounding_box().axis_interval(axis);
    let b_axis_interval = b.bounding_box().axis_interval(axis);
    if a_axis_interval.min < b_axis_interval.min {
      Ordering::Less
    } else if a_axis_interval.min == b_axis_interval.min {
      Ordering::Equal
    } else {
      Ordering::Greater
    }
  }
}

impl Hittable for BvhNode {
  fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
    if !self.bbox.hit(ray, ray_t) {
      return false;
    }
    let hit_left = self.left.hit(ray, ray_t, rec);
    let hit_right = self.right.hit(ray, Interval::new(ray_t.min, if hit_left {rec.t} else {ray_t.max}), rec);
    hit_left || hit_right
  }

  fn to_object(self) -> Object {
    Arc::new(self)
  }

  fn bounding_box(&self) -> Aabb {
    self.bbox
  }
}

