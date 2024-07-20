use crate::PI;

use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::env;
use std::io;
use rand::Rng;
use nalgebra::Vector3;
use opencv::core::VecN;

pub fn degrees_to_radians(degrees: f64) -> f64 {
  degrees * PI / 180.0
}

pub fn rand_01() -> f64{
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

// [min, max)
pub fn rand_range(min: f64, max: f64) -> f64{
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn rand_range_int(min: i32, max: i32) -> i32 {
    rand_range(min.into(), max as f64 + 1.0).floor() as i32
}

pub fn is_ci() -> bool {
  option_env!("CI").unwrap_or_default() == "true"
}

pub fn get_ProgressBar(height: usize, width: usize) -> ProgressBar {
  let bar: ProgressBar = if is_ci() {
      ProgressBar::hidden()
  } else {
      ProgressBar::new((height * width) as u64)
  };

  bar.set_style(ProgressStyle::default_bar()
  .template("{spinner:.green} Elapsed {elapsed_precise} [{wide_bar:.cyan/blue}] {pos}/{len} ({eta}) {msg}")
  .progress_chars("●▸▹⋅"));

  bar
}

pub fn get_output_confirmation(file_name: &mut String, default_file_name: &String, is_release: bool) -> bool {
  if is_release {
    println!("Release version; confirmation procedure skipped.");
    return true
  
  }
    
  if file_name == default_file_name {
    println!("File name is default; confirmation procedure skipped.");
    return true
  }

  let mut confirmation_input: String = String::default();
  println!("Please confirm to write to {}. \n[\'y\' to confirm; 'n' to revert to default file path; otherwise, cancel]", file_name.clone());
  let input = io::stdin().read_line(&mut confirmation_input);

  match input {
      Ok(_) => {
          if confirmation_input.chars().nth(0) == Some('y') {
              true
          } else {
              if confirmation_input.chars().nth(0) == Some('n') {
                  println!("File name reverted to \"{}\"", default_file_name.clone());
                  *file_name = default_file_name.clone();
                  true
              } else {
                  false
              }
          }
      },
      Err(_) => {
          false
      }
  }
}


pub fn convert_U8VecN_to_Vector3f64(vec : &VecN<u8, 3>) -> Vector3<f64> {
    Vector3::new(vec[0] as f64, vec[1] as f64, vec[2] as f64)
}