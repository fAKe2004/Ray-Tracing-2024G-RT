#![allow(warnings)]

mod modules;
use modules::*;

// standard library
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::env;
use std::io;
use color::ColorType;


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
        println!("Info: No output file specified, using default file path: \"{}{}\"", path, file_name);
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

fn build_camera_1() -> Camera { // bouncing_spheres
    let aspect_ratio = 16.0 / 9.0;
    // let image_width = 400 as usize;
    let image_width = 1200 as usize;
    let sample_per_pixel = 500 as usize;
    // let sample_per_pixel = 100 as usize;
    // let sample_per_pixel = 10 as usize;
    let max_ray_depth = 50 as usize;
    let vfov = 20.0;
    
    let lookfrom = Point3::new(13.0, 2.0, 3.0);   // Point camera is looking from
    let lookat = Point3::new(0.0, 0.0, 0.0); // Point camera is looking at
    let vup = Vec3::new(0.0, 1.0, 0.0);     // Camera-relative "up" direction

    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, 
        image_width, 
        sample_per_pixel, 
        max_ray_depth, 
        vfov, 
        lookfrom, 
        lookat, 
        vup, 
        defocus_angle, 
        focus_dist
    );
    cam
}


fn build_camera_2() -> Camera { // checkered_spheres
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400 as usize;
    let sample_per_pixel = 100 as usize;
    let max_ray_depth = 50 as usize;
    let vfov = 20.0;
    
    let lookfrom = Point3::new(13.0, 2.0, 3.0);   // Point camera is looking from
    let lookat = Point3::new(0.0, 0.0, 0.0); // Point camera is looking at
    let vup = Vec3::new(0.0, 1.0, 0.0);     // Camera-relative "up" direction

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, 
        image_width, 
        sample_per_pixel, 
        max_ray_depth, 
        vfov, 
        lookfrom, 
        lookat, 
        vup, 
        defocus_angle, 
        focus_dist
    );
    cam
}

fn build_camera_3() -> Camera { // earth
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400 as usize;
    let sample_per_pixel = 100 as usize;
    let max_ray_depth = 50 as usize;
    let vfov = 20.0;
    
    let lookfrom = Point3::new(0.0, 0.0, 12.0);   // Point camera is looking from
    let lookat = Point3::new(0.0, 0.0, 0.0); // Point camera is looking at
    let vup = Vec3::new(0.0, 1.0, 0.0);     // Camera-relative "up" direction

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, 
        image_width, 
        sample_per_pixel, 
        max_ray_depth, 
        vfov, 
        lookfrom, 
        lookat, 
        vup, 
        defocus_angle, 
        focus_dist
    );
    cam
}

fn build_camera_4() -> Camera { // perlin_spheres
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400 as usize;
    let sample_per_pixel = 100 as usize;
    let max_ray_depth = 50 as usize;
    let vfov = 20.0;
    
    let lookfrom = Point3::new(13.0, 2.0, 3.0);   // Point camera is looking from
    let lookat = Point3::new(0.0, 0.0, 0.0); // Point camera is looking at
    let vup = Vec3::new(0.0, 1.0, 0.0);     // Camera-relative "up" direction

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, 
        image_width, 
        sample_per_pixel, 
        max_ray_depth, 
        vfov, 
        lookfrom, 
        lookat, 
        vup, 
        defocus_angle, 
        focus_dist
    );
    cam
}


fn build_world_1() -> HittableList {
    let mut world = HittableList::default();

    let checker = CheckerTexture::new_by_color(0.32, ColorType::new(0.2, 0.3, 0.1), ColorType::new(0.9, 0.9, 0.9)).to_texture();
    
    let material_ground = Lambertian::new(checker).to_material();
    world.add(Sphere::new_static(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            material_ground,
        ).to_object()
    );
    
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_01();
            let center = Point3::new(a as f64 + 0.9 * rand_01(), 0.2, b as f64 + 0.9 * rand_01());


            if (center - Point3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let mut sphere_material : Material = DefaultMaterial::new().to_material();

                if choose_mat < 0.8 {
                    let albedo = ColorType::rand_01().elemul(&ColorType::rand_01());
                    let center_after_move = center + Vec3::new(0.0, rand_range(0.0, 0.5), 0.0);

                    sphere_material = Lambertian::new_by_color(albedo).to_material();

                    world.add(
                        Sphere::new_moving(center, center_after_move, 0.2, sphere_material).to_object()
                    );
                } else if choose_mat < 0.95 {
                    let albedo = ColorType::rand_range(0.5, 1.0);
                    let fuzz = rand_range(0.0, 0.5);
                    sphere_material = Metal::new(albedo, fuzz).to_material();
                    world.add(
                        Sphere::new_static(center, 0.2, sphere_material).to_object()
                    );
                } else {
                    sphere_material = Dielectric::new(1.5).to_material();
                    world.add(
                        Sphere::new_static(center, 0.2, sphere_material).to_object()
                    );
                }
            }
        }
    }


    let material_1: Material = Dielectric::new(1.5).to_material();
    world.add(
        Sphere::new_static(Point3::new(0.0, 1.0, 0.0), 1.0, material_1).to_object()
    );


    let material_2: Material = Lambertian::new_by_color(ColorType::new(0.4, 0.2, 0.1)).to_material();
    world.add(
        Sphere::new_static(Point3::new(-4.0, 1.0, 0.0), 1.0, material_2).to_object()
    );

    let material_3: Material = Metal::new(ColorType::new(0.7, 0.6, 0.5), 0.0).to_material();
    world.add(
        Sphere::new_static(Point3::new(4.0, 1.0, 0.0), 1.0, material_3).to_object()
    );

    world.to_bvh()
}


fn build_world_2() -> HittableList {
    let mut world = HittableList::default();

    let checker = CheckerTexture::new_by_color(0.32, ColorType::new(0.2, 0.3, 0.1), ColorType::new(0.9, 0.9, 0.9)).to_texture();
    
    let material_ground = Lambertian::new(checker).to_material();
    world.add(Sphere::new_static(
            Point3::new(0.0, -10.0, 0.0),
            10.0,
            material_ground.clone(),
        ).to_object()
    );
    world.add(Sphere::new_static(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        material_ground.clone(),
        ).to_object()
    );

    world.to_bvh()
}

fn build_world_3() -> HittableList {
    let mut world = HittableList::default();
    let erath_texture = ImageTexture::new("input/earthmap.jpg").to_texture();
    let erath_surface = Lambertian::new(erath_texture).to_material();
    let global = Sphere::new_static(Point3::zero(), 2.0, erath_surface);
    world.add(global.to_object());

    world.to_bvh()
}

fn build_world_4() -> HittableList {
    let mut world = HittableList::default();
    let pertext = NoiseTexture::new().to_texture();
    let material = Lambertian::new(pertext).to_material();
    world.add(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material.clone()
    ).to_object());
    world.add(Sphere::new_static(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        material.clone()
    ).to_object());

    world.to_bvh()
}
// main part

fn main() {

    let parameters = init_prompt();

    let TYPE = 4;
    let cam = match TYPE {
        1 => build_camera_1(),
        2 => build_camera_2(),
        3 => build_camera_3(),
        4 => build_camera_4(),
        _ => panic!("Not matched"),
    };
    let world = match TYPE {
        1 => build_world_1(),
        2 => build_world_2(),
        3 => build_world_3(),
        4 => build_world_4(),
        _ => panic!("Not matched"),
    };

    let img = cam.render(&(world.to_object()));

    tail_process(img, parameters, "fAKe");
}


