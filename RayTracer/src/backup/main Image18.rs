mod modules;
use modules::*;

// standard library
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::env;
use std::io;


// anxilliary part


// return (path, file_name, default_file_name, quality)
fn init_prompt() -> (String, String, String, u8) {
    let path: String = "output/".into();
    let default_file_name: String = "test.jpg".into();

    let mut file_name: String = default_file_name.clone();

    let quality = 60 as u8;



    let args: Vec<String> = env::args().collect();

    println!("[Ray Tracer]");
    if args.len() < 2 {
        println!("Info: No output file specified, using default file path: \"{}{}\"``", path, file_name);
    } else {
        file_name = args[1].clone();
        println!("Info: Output file specified as \"{}{}\"", path, file_name);
    }
    (path, file_name, default_file_name, quality)
}

fn tail_process(img: RgbImage, parameters: (String, String, String, u8), author: &str) {
    let (path, mut file_name, default_file_name, quality) = parameters;
    
    println!("Ouput image as \"{}\"\n Author: {}", path.clone() + &file_name, author);
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

// main part

fn main() {

    let parameters = init_prompt();

    
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800 as usize;
    let sample_per_pixel = 100 as usize;
    // let sample_per_pixel = 10 as usize;
    let max_ray_depth = 50 as usize;
    let cam: Camera = Camera::new(aspect_ratio, image_width, sample_per_pixel, max_ray_depth);

    let material_ground: Material = Rc::new(Lambertian::new(ColorType::new(0.8, 0.8, 0.0)));
    let material_center: Material = Rc::new(Lambertian::new(ColorType::new(0.1, 0.2, 0.5)));
    let material_left: Material = Rc::new(Dielectric::new(1.5)); // air to water
    let material_bubble: Material = Rc::new(Dielectric::new(1.00 / 1.50));
    let material_right: Material = Rc::new(Metal::new(ColorType::new(0.8, 0.6, 0.2), 1.0));


    let mut world = HittableList::default();
    world.add(Rc::new(
        Sphere::new(    
            Point3::new(0.0, -100.5, -1.0), 100.0, material_ground
        )
    )
    );
    world.add(Rc::new(
            Sphere::new(
                Point3::new(0.0, 0.0, -1.2), 0.5, material_center
            )
        )
    );
    world.add(Rc::new(
            Sphere::new(
                Point3::new(-1.0, 0.0, -1.0), 0.5, material_left
            )
        )
    );
    world.add(Rc::new(
            Sphere::new(
                Point3::new(-1.0, 0.0, -1.0),   0.4, material_bubble)
        )
    );

    world.add(Rc::new(
            Sphere::new(
                Point3::new(1.0, 0.0, -1.0), 0.5, material_right
            )
        )
    );

    let img = cam.render(&(world.to_object()));

    tail_process(img, parameters, "fAKe");
}


