mod color;
mod vec3;
mod ray;

// standard library
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::env;
use std::io;

// my library
use color::{ColorType, convert_ColorType_to_u8Array, write_color_256, write_color_01};
use vec3::{Vec3, Point3};
use ray::Ray;

// anxilliary part
const AUTHOR: &str = "fAKe";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn get_ProgressBar(height: u32, width: u32) -> ProgressBar {
    let bar: ProgressBar = if is_ci() {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    bar.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} Elapsed [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
    .progress_chars("●▸▹⋅"));

    bar
}

fn get_output_confirmation(file_name: &mut String, default_file_name: &String) -> bool {

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

// main part

fn ray_color(ray: &Ray) -> ColorType {
    let unit_direction = ray.dir.normalize();
    let a = 0.5*(unit_direction.y + 1.0);
    ColorType::new(1.0, 1.0, 1.0) * (1.0 - a) + ColorType::new(0.5, 0.7, 1.0) * a
}


fn main() {
    let path: String = "output/".into();
    let default_file_name: String = "test.jpg".into();
    let mut file_name: String = default_file_name.clone();
    let aspect_ratio = 16.0 / 9.0;
    
    let width = 800;
    let height = (width as f64 / aspect_ratio) as u32;
    
    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (width as f64 / height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);    
    
    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / width as f64;
    let pixel_delta_v = viewport_v / height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
      - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
    /*
      layout:
      00 -> u
      |
      V
      v
    */


    let quality = 60;
    let bar = get_ProgressBar(height, width);

    let args: Vec<String> = env::args().collect();

    println!("[Ray Tracer]");
    if args.len() < 2 {
        println!("Info: No output file specified, using default file path: \"{}{}\"``", path, file_name);
    } else {
        file_name = args[1].clone();
        println!("Info: Output file specified as \"{}{}\"", path, file_name);
    }

// end of unrelated pre process

    let mut img: RgbImage = ImageBuffer::new(width, height);

    for j in 0..height {
        for i in 0..width {

            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            // println!("at [{}, {}] is [{} {} {}]", i, j, ray_direction.x, ray_direction.y, ray_direction.z);
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r);

            write_color_01(pixel_color, &mut img, i as usize, j as usize);

            bar.inc(1);
        }
    }

    bar.finish();

    println!("Ouput image as \"{}\"\n Author: {}", path.clone() + &file_name, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);


    let confirmation_flag = get_output_confirmation(&mut file_name, &default_file_name);
    
    if confirmation_flag == false {
        println!("Canceled");
        return ();
    }

    let mut output_file: File = File::create(path.clone() + &file_name).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
    println!("Render finished with success.");
}
