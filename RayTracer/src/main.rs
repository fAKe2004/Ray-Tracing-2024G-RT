#![allow(warnings)]


mod modules;
use modules::*;

use color::ColorType;

// standard library
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::{ProgressBar, ProgressStyle};
use nalgebra::center;
use nalgebra::Point;
use std::fs::File;
use std::env;
use std::io;


// anxilliary part


// return (path, file_name, default_file_name, quality)
fn init_prompt() -> (String, String, String, u8, bool) {
    let path: String = "output/".into();
    let default_file_name: String = "test.jpg".into();

    let mut file_name: String = default_file_name.clone();

    let quality = 60 as u8;

    let mut release_flag = false;



    let args: Vec<String> = env::args().collect();

    println!("[Ray Tracer]");
    if args.len() < 2 {
        println!("Info: No output file specified, using default file path: \"{}{}\"", path, file_name);
    } else {
        file_name = args[1].clone();
        println!("Info: Output file specified as \"{}{}\"", path, file_name);
        if args.len() > 2 && args[2].clone() == "--release" {
            release_flag = true
        }
    }
    (path, file_name, default_file_name, quality, release_flag)
}

fn tail_process(img: RgbImage, parameters: (String, String, String, u8, bool), author: &str) {
    let (path, mut file_name, default_file_name, quality, release_flag) = parameters;
    
    println!("Ouput image as \"{}\"\n Author: {}\n Is release? {}", path.clone() + &file_name, author, release_flag);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);


    let confirmation_flag = get_output_confirmation(&mut file_name, &default_file_name, release_flag);

    if confirmation_flag == false {
        println!("Canceled");
        return ();
    }

    let mut output_file: File = File::create(path.clone() + &file_name).unwrap();
    while true {
        match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
            Ok(_) => { 
                println!("Render finished with success.");
                break; 
            }
            Err(_) => {
                println!("Outputting image fails. \n Please enter another file name.");
                let mut input: String = String::default();
                let _ = io::stdin().read_line(&mut input);
                output_file = File::create(path.clone() + &input).unwrap();
            }
        }
    }
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

    let background = ColorType::new(0.70, 0.80, 1.00);

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
        focus_dist,
        background
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

    let background = ColorType::new(0.70, 0.80, 1.00);

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
        focus_dist,
        background
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

    let background = ColorType::new(0.70, 0.80, 1.00);

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
        focus_dist,
        background
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

    let background = ColorType::new(0.70, 0.80, 1.00);

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
        focus_dist,
        background
    );
    cam
}

fn build_camera_5() -> Camera { // quads
    let aspect_ratio = 1.0;
    let image_width = 400 as usize;
    let sample_per_pixel = 100 as usize;
    let max_ray_depth = 50 as usize;
    let vfov = 80.0;
    
    let lookfrom = Point3::new(0.0, 0.0, 9.0);   // Point camera is looking from
    let lookat = Point3::new(0.0, 0.0, 0.0); // Point camera is looking at
    let vup = Vec3::new(0.0, 1.0, 0.0);     // Camera-relative "up" direction

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let background = ColorType::new(0.70, 0.80, 1.00);

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
        focus_dist,
        background
    );
    cam
}

fn build_camera_6() -> Camera { // simple_light
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400 as usize;
    let sample_per_pixel = 100 as usize;
    let max_ray_depth = 50 as usize;
    let vfov = 20.0;
    
    let lookfrom = Point3::new(26.0, 3.0,6.0);   // Point camera is looking from
    let lookat = Point3::new(0.0, 2.0, 0.0); // Point camera is looking at
    let vup = Vec3::new(0.0, 1.0, 0.0);     // Camera-relative "up" direction

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let background = ColorType::new(0.0, 0.0, 0.0);

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
        focus_dist,
        background
    );
    cam
}

fn build_camera_7() -> Camera { // cornell_box
    let aspect_ratio = 1.0;
    let image_width = 600 as usize;
    let sample_per_pixel = 200 as usize;
    let max_ray_depth = 50 as usize;
    let vfov = 40.0;
    
    let lookfrom = Point3::new(278.0, 278.0,-800.0);   // Point camera is looking from
    let lookat = Point3::new(278.0, 278.0, 0.0); // Point camera is looking at
    let vup = Vec3::new(0.0, 1.0, 0.0);     // Camera-relative "up" direction

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let background = ColorType::new(0.0, 0.0, 0.0);

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
        focus_dist,
        background
    );
    cam
}

fn build_camera_8() -> Camera { // cornell_smoke
    let aspect_ratio = 1.0;
    let image_width = 600 as usize;
    let sample_per_pixel = 200 as usize;
    let max_ray_depth = 50 as usize;
    let vfov = 40.0;
    
    let lookfrom = Point3::new(278.0, 278.0,-800.0);   // Point camera is looking from
    let lookat = Point3::new(278.0, 278.0, 0.0); // Point camera is looking at
    let vup = Vec3::new(0.0, 1.0, 0.0);     // Camera-relative "up" direction

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let background = ColorType::new(0.0, 0.0, 0.0);

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
        focus_dist,
        background
    );
    cam
}

fn build_camera_9(image_width: usize, sample_per_pixel: usize, max_ray_depth: usize) -> Camera { // cornell_smoke
    let aspect_ratio = 1.0;
    // let image_width = 1200 as usize;
    // let sample_per_pixel = 1000 as usize;
    // let max_ray_depth = 50 as usize;
    let vfov = 40.0;
    
    let lookfrom = Point3::new(478.0, 278.0,-600.0);   // Point camera is looking from
    let lookat = Point3::new(278.0, 278.0, 0.0); // Point camera is looking at
    let vup = Vec3::new(0.0, 1.0, 0.0);     // Camera-relative "up" direction

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let background = ColorType::new(0.0, 0.0, 0.0);

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
        focus_dist,
        background
    );
    cam
}

fn build_world_1() -> Object {
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

fn build_world_2() -> Object {
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

fn build_world_3() -> Object {
    let mut world = HittableList::default();
    let erath_texture = ImageTexture::new("input/earthmap.jpg").to_texture();
    let erath_surface = Lambertian::new(erath_texture).to_material();
    let global = Sphere::new_static(Point3::zero(), 2.0, erath_surface);
    world.add(global.to_object());

    world.to_bvh()
}

fn build_world_4() -> Object {
    let mut world = HittableList::default();
    let pertext = NoiseTexture::new(4.0).to_texture();
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

fn build_world_5() -> Object {
    let mut world = HittableList::default();

    let left_red = Lambertian::new_by_color(ColorType::new(1.0, 0.2, 0.2)).to_material();
    let back_green = Lambertian::new_by_color(ColorType::new(0.2, 1.0, 0.2)).to_material();
    let right_blue = Lambertian::new_by_color(ColorType::new(0.2, 0.2, 1.0)).to_material();
    let upper_orange = Lambertian::new_by_color(ColorType::new(1.0, 0.5, 0.0)).to_material();
    let lower_teal = Lambertian::new_by_color(ColorType::new(0.2, 0.8, 0.8)).to_material();

    world.add(
        Quad::new(Point3::new(-3.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), left_red).to_object()
    );

    world.add(
        Quad::new(Point3::new(-2.0, -2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), back_green).to_object()
    );


    world.add(
        Quad::new(Point3::new(3.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), right_blue).to_object()
    );

    world.add(
        Quad::new(Point3::new(-2.0, 3.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), upper_orange).to_object()
    );

    
    world.add(
        Quad::new(Point3::new(-2.0, -3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), lower_teal).to_object()
    );

    world.to_bvh()
}

fn build_world_6() -> Object {
    let mut world = HittableList::default();
    let pertext = NoiseTexture::new(4.0).to_texture();
    let mat = Lambertian::new(pertext).to_material();
    world.add(
        Sphere::new_static(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            mat.clone()
        ).to_object()
    );
    world.add(
        Sphere::new_static(
            Point3::new(0.0, 2.0, 0.0),
            2.0,
            mat.clone()
        ).to_object()
    );

    let difflight = DiffuseLight::new_by_color(ColorType::new(4.0, 4.0, 4.0)).to_material();

    world.add(
        Quad::new(
            Point3::new(3.0, 1.0, -2.0),
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
            difflight.clone()
        ).to_object()
    );

    
    world.add(
        Sphere::new_static(
            Point3::new(0.0, 7.0, 0.0),
            2.0,
            difflight.clone()
        ).to_object()
    );

    world.to_bvh()
}

fn build_world_7() -> Object {
    let mut world = HittableList::default();
    let red = Lambertian::new_by_color(ColorType::new(0.65, 0.05, 0.05)).to_material();
    let white = Lambertian::new_by_color(ColorType::new(0.73, 0.73, 0.73)).to_material();
    let green = Lambertian::new_by_color(ColorType::new(0.12, 0.45, 0.12)).to_material();
    let light = DiffuseLight::new_by_color(ColorType::new(15.0, 15.0, 15.0)).to_material();

    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green
    ).to_object());

    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red
    ).to_object());


    world.add(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light
    ).to_object());

    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone()
    ).to_object());

    world.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone()
    ).to_object());

    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0,555.0, 0.0),
        white.clone()
    ).to_object());

    let box1 = build_box(
        Point3::new(0.0, 0.0, 0.0), 
        Point3::new(165.0, 330.0, 165.0), 
        white.clone()
    ).to_object();
    let box1 = RotateY::new(box1, 15.0).to_object();
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)).to_object();
    world.add(box1);

    let box2 = build_box(
        Point3::new(0.0, 0.0, 0.0), 
        Point3::new(165.0, 165.0, 165.0), 
        white.clone()
    ).to_object();
    let box2 = RotateY::new(box2, -18.0).to_object();
    let box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)).to_object();
    world.add(box2);

    world.to_bvh()
}

fn build_world_8() -> Object {
    let mut world = HittableList::default();
    let red = Lambertian::new_by_color(ColorType::new(0.65, 0.05, 0.05)).to_material();
    let white = Lambertian::new_by_color(ColorType::new(0.73, 0.73, 0.73)).to_material();
    let green = Lambertian::new_by_color(ColorType::new(0.12, 0.45, 0.12)).to_material();
    let light = DiffuseLight::new_by_color(ColorType::new(7.0, 7.0, 7.0)).to_material();

    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone()
    ).to_object());

    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone()
    ).to_object());


    world.add(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light.clone()
    ).to_object());

    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone()
    ).to_object());

    world.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone()
    ).to_object());

    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0,555.0, 0.0),
        white.clone()
    ).to_object());

    let box1 = build_box(
        Point3::new(0.0, 0.0, 0.0), 
        Point3::new(165.0, 330.0, 165.0), 
        white.clone()
    ).to_object();
    let box1 = RotateY::new(box1, 15.0).to_object();
    let box1 = Translate::new(box1, Vec3::new(265.0, 1.0, 295.0)).to_object();
    let box1 = ConstantMedium::new_by_color(box1, 0.01, ColorType::zero()).to_object();
    world.add(box1);

    let box2 = build_box(
        Point3::new(0.0, 0.0, 0.0), 
        Point3::new(165.0, 165.0, 165.0), 
        white.clone()
    ).to_object();
    let box2 = RotateY::new(box2, -18.0).to_object();
    let box2 = Translate::new(box2, Vec3::new(130.0, 1.0, 65.0)).to_object();
    let box2 = ConstantMedium::new_by_color(box2, 0.01, ColorType::ones()).to_object();
    world.add(box2);

    world.to_bvh()
}

fn build_world_9() -> Object {
    let mut world = HittableList::default();
    let ground = Lambertian::new_by_color(ColorType::new(0.48, 0.83, 0.53)).to_material();

    let mut boxes1 = HittableList::default();
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let p0 = Point3::new(
                -1000.0 + i as f64 * w,
                0.0,
                -1000.0 + j as f64 * w
            );
            let p1 = Point3::new(
                p0.x + w,
                rand_range(1.0, 101.0),
                p0.z + w
            );

            boxes1.add(build_box(p0, p1, ground.clone()).to_object());
        }
    }
    world.add(boxes1.to_bvh());

    let light = DiffuseLight::new_by_color(ColorType::new(7.0, 7.0, 7.0)).to_material();
    world.add(
        Quad::new(
            Point3::new(123.0, 554.0, 147.0),
            Vec3::new(300.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 265.0),
            light
        ).to_object()
    ); // light

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Lambertian::new_by_color(ColorType::new(0.7, 0.3, 0.1)).to_material();
    world.add(Sphere::new_moving(
        center1, center2,
        50.0, 
        sphere_material
    ).to_object());

    world.add(Sphere::new_static(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5).to_material()
    ).to_object());

    world.add(Sphere::new_static(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(ColorType::new(0.8, 0.8, 0.9), 1.0).to_material()
    ).to_object());

    let boundary = Sphere::new_static(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Dielectric::new(1.5).to_material()
    ).to_object();
    world.add(boundary.clone());

    world.add(ConstantMedium::new_by_color(
        boundary.clone(),
        0.2, 
        ColorType::new(0.2, 0.4, 0.9)
    ).to_object());

    let boundary = Sphere::new_static(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Dielectric::new(1.5).to_material()
    ).to_object();
    world.add(ConstantMedium::new_by_color(
        boundary.clone(),
        0.0001, 
        ColorType::new(1.0, 1.0, 1.0)
    ).to_object());

    let emat = Lambertian::new(ImageTexture::new("input/earthmap.jpg").to_texture()).to_material();
    world.add(Sphere::new_static(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat
    ).to_object());

    let pertext = NoiseTexture::new(0.2).to_texture();
    world.add(Sphere::new_static(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new(pertext).to_material()
    ).to_object());

    let mut boxes2 = HittableList::default();
    let white = Lambertian::new_by_color(ColorType::new(0.73, 0.73, 0.73)).to_material();
    let ns = 1000;
    for j in 0..ns {
        boxes2.add(
            Sphere::new_static(
                Point3::rand_range(0.0, 165.0),
                10.0,
                white.clone()
            ).to_object()
        );
    }

    world.add(Translate::new(
        RotateY::new(
            boxes2.to_bvh(),
            15.0
        ).to_object(),
        Vec3::new(-100.0, 270.0, 395.0)
    ).to_object());

    world.to_bvh()
}
// main part

fn build_final_camera(image_width: usize, sample_per_pixel: usize, max_ray_depth: usize) -> Camera { // cornell_smoke
    let aspect_ratio = 16.0 / 9.0;
    let vfov = 40.0;
    
    let lookfrom = Point3::new(600.0, 120.0,600.0);   // Point camera is looking from
    let lookat = Point3::new(0.0, 0.0, 0.0); // Point camera is looking at
    let vup = Vec3::new(0.0, 1.0, 0.0);     // Camera-relative "up" direction

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let background = ColorType::new(0.0, 0.0, 0.0);

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
        focus_dist,
        background
    );
    cam
}

fn build_final_world() -> Object {
    let mut world = HittableList::default();
    
    let SeaTexture = ImageTexture::new("input/sea.jpg").to_texture();
    let MarsTexture = ImageTexture::new("input/Mars.jpg").to_texture();
    let JupiterTexture = ImageTexture::new("input/Jupiter.jpg").to_texture();
    let UranusTexture = ImageTexture::new("input/Uranus.jpg").to_texture();
    let VenusTexture = ImageTexture::new("input/Venus.jpg").to_texture();
    let SaturnTexture = ImageTexture::new("input/Saturn.jpg").to_texture();
    let SunTexture = ImageTexture::new("input/Sun.jpg").to_texture();
    let EarthDayTexture = ImageTexture::new("input/EarthDay.jpg").to_texture();

    let Dielectric05 = Dielectric::new(0.5).to_material();
    let Dielectric15 = Dielectric::new(1.5).to_material();
    
    let OrbitStationMaterial = Dielectric::new(1.5).to_material();
    let OrbitStationInnerMaterial = LambertianWithLight::new(JupiterTexture.clone(), ColorType::new(10.0, 10.0, 10.0)).to_material();

    world.add( // main sphere
        Sphere::new_static(
            Point3::new(0.0, -100.0, 0.0),
            70.0, 
            LambertianWithLight::new(SeaTexture.clone(), ColorType::new(0.6, 0.6, 0.6)).to_material()
        ).to_object()
    );
    world.add( // main sphere
        Sphere::new_static(
            Point3::new(0.0, -100.0, 0.0),
            100.0, 
            Dielectric15.clone()
        ).to_object()
    );


    world.add( // OrbitStation
        Sphere::new_static(
            Point3::new(0.0, 50.0, 0.0),
            20.0, 
            OrbitStationMaterial
        ).to_object()
    );

    world.add(
        Sphere::new_static(
            Point3::new(0.0, 50.0, 0.0),
            8.0, 
            OrbitStationInnerMaterial
        ).to_object()
    );

    world.add(
        Circle::new(
            Point3::new(0.0, 50.0, 0.0),
            Vec3::new(50.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 50.0),
            Lambertian::new(VenusTexture.clone()).to_material()
        ).to_object()
    );



    world.add( // light
        Sphere::new_static(
            Point3::new(0.0, 500.0, 0.0),
            50.0,
            DiffuseLight::new_by_color(ColorType::new(100.0, 100.0, 100.0)).to_material()
        ).to_object()
    );

    let TriangleMaterial = Dielectric05.clone();
    
    world.add( // orbit station extension
        Triangle::new(
            Point3::new(0.0, 50.0, 0.0),
            Point3::new(10.0, -60.0, -10.0),
            Point3::new(10.0, -60.0, 10.0),
            TriangleMaterial.clone()
        ).to_object()
    );
    world.add(
        Triangle::new(
            Point3::new(0.0, 50.0, 0.0),
            Point3::new(10.0, -60.0, 10.0),
            Point3::new(-10.0, -60.0, 10.0),
            TriangleMaterial.clone()
        ).to_object()
    );
    world.add(
        Triangle::new(
            Point3::new(0.0, 50.0, 0.0),
            Point3::new(-10.0, -60.0, 10.0),
            Point3::new(-10.0, -60.0, -10.0),
            TriangleMaterial.clone()
        ).to_object()
    );
    world.add(
        Triangle::new(
            Point3::new(0.0, 50.0, 0.0),
            Point3::new(10.0, -60.0, -10.0),
            Point3::new(-10.0, -60.0, -10.0),
            TriangleMaterial.clone()
        ).to_object()
    );


    let SaturnCenter = Point3::new(-300.0, -30.0,200.0);
    world.add( // main sphere 1
        Sphere::new_static(
            SaturnCenter,
            50.0,
            Lambertian::new(SaturnTexture.clone()).to_material()
        ).to_object()
    );
    world.add(
        Ring::new(
            SaturnCenter,
            Vec3::new(80.0, -20.0, 0.0),
            Vec3::new(0.0, 0.0, 90.0),
            Lambertian::new(JupiterTexture.clone()).to_material(),
            0.75
        ).to_object()
    );

    world.add( // main sphere 2
        Sphere::new_static(
            Point3::new(100.0, -50.0, 450.0),
            25.0,
            LambertianWithLight::new(UranusTexture.clone(), ColorType::new(0.1, 0.1, 0.1)).to_material()
        ).to_object()
    );

    world.add( // main sphere 3
        Sphere::new_static(
            Point3::new(200.0, -50.0, 300.0),
            40.0,
            LambertianWithLight::new(MarsTexture.clone(), ColorType::new(0.02, 0.02, 0.02)).to_material()
        ).to_object()
    );

    world.add( // main sphere 4
        Sphere::new_static(
            Point3::new(100.0, -25.0, -300.0),
            55.0,
            Lambertian::new(EarthDayTexture.clone()).to_material()
        ).to_object()
    );

    world.add( // main sphere 5
        Sphere::new_static(
            Point3::new(400.0, -35.0, 0.0),
            50.0,
            Lambertian::new(JupiterTexture.clone()).to_material()
        ).to_object()
    );




    let AmbientLight = ColorType::new(0.2, 0.2, 0.2);
    let RandomSphereMaterials: Vec<Material> = vec![
        Metal::new(ColorType::new(0.1, 0.1, 0.8), 0.2).to_material(),
        Metal::new(ColorType::new(0.8, 0.8, 0.8), 0.2).to_material(),
        Metal::new(ColorType::new(0.8, 0.8, 0.8), 0.2).to_material(),
        Dielectric::new(1.5).to_material(),
        Dielectric::new(1.5).to_material(),
        Dielectric::new(1.5).to_material(),
        Dielectric::new(0.2).to_material(),
        Dielectric::new(0.2).to_material(),
        Dielectric::new(0.2).to_material(),
        LambertianWithLight::new(UranusTexture.clone(), AmbientLight).to_material(),
        LambertianWithLight::new(JupiterTexture.clone(), AmbientLight).to_material(),
        LambertianWithLight::new(MarsTexture.clone(), AmbientLight).to_material(),
        LambertianWithLight::new(JupiterTexture.clone(), AmbientLight).to_material(),
        LambertianWithLight::new(SaturnTexture.clone(), AmbientLight).to_material(),
        LambertianWithLight::new(NoiseTexture::new(1.0).to_texture(), AmbientLight).to_material(),
        LambertianWithLight::new(NoiseTexture::new(1.0).to_texture(), AmbientLight).to_material()
    ];

    let center_interval = Interval::new(-100.0, 100.0);
    for i in 0..80 {
        let x = rand_range(-1000.0, -100.0);
        let y = rand_range(50.0, 500.0);
        let z = rand_range(-1000.0, -100.0);
        let r = rand_range(5.0, 15.0);
        let dynamic = rand_01() < 0.8;
        if center_interval.contains(x) && center_interval.contains(y) {
            continue;
        }
        let center = Point3::new(x, y, z);

        let mat = RandomSphereMaterials[rand_range_int(0, RandomSphereMaterials.len() as i32 - 1) as usize].clone();
        if dynamic {
            let velo = Vec3::rand_01() * r * r;
            world.add(
                Sphere::new_moving(
                    center,
                    center + velo,
                    r / 1.5, 
                    mat
                ).to_object()
            )
        } else {
            world.add(
                Sphere::new_static(
                    center,
                    r,
                    mat
                ).to_object()
            );
        }
        
    }


    let RandomPillarMaterials: Vec<Material> = vec![
        Dielectric::new(1.2).to_material(),
        Dielectric::new(0.5).to_material(),
        Dielectric::new(1.5).to_material()
    ];
    for i in 0..10 {
        let x = rand_range(-1000.0, -200.0);
        let y = rand_range(-40.0, 80.0);
        let z = rand_range(-1000.0, -200.0);
        let d = rand_range(30.0, 50.0);

        if center_interval.contains(x) && center_interval.contains(y) {
            continue;
        }
        let mat = RandomPillarMaterials[rand_range_int(0, RandomPillarMaterials.len() as i32 - 1) as usize].clone();

        let pillar = build_box(Point3::new(x, -10000.0, z), Point3::new(x + d, y, z + d), mat).to_object();
        world.add(
            pillar
        );
    }

    world.add( // mirror
        Quad::new(
            Point3::new(-2000.0, -100.0, -2000.0),
            Vec3::new(4000.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 4000.0),
            Metal::new(ColorType::new(1.0, 1.0, 1.0), 0.05).to_material()
        ).to_object()
    );
    // let halo = build_box(Point3::new(-2000.0, -100.0, -2000.0), Point3::new(2000.0, -90.0, 2000.0), DefaultMaterial::new().to_material()).to_bvh();
            
    // world.add( // halo
    //     ConstantMedium::new(halo, 0.04, SolidColor::new(ColorType::new(0.2, 0.4, 0.6)).to_texture()).to_object()
    // );

    let background_material = LambertianWithLight::new(ImageTexture::new("input/hubble_skymap.jpg").to_texture(), ColorType::new(0.1, 0.1, 0.1)).to_material();

    world.add(
        // Quad::new(
        //     Point3::new(-5000.0 - 7500.0, -4000.0, 5000.0 - 7500.0),
        //     Vec3::new(10000.0, 0.0, -10000.0),
        //     Vec3::new(0.0, 10000.0, 0.0),
        //     background_material
        // ).to_object()
        Sphere::new_static(
            Point3::new(0.0, 0.0, 0.0), 
            3000.0, 
            background_material
        ).to_object()
    );

    world.to_bvh()
}







fn main() {

    let parameters = init_prompt();

    let mut TYPE = if parameters.4 {0} else {-1};
    let cam = match TYPE {
        -1 => build_final_camera(400, 2000, 10),
        0 => build_final_camera(1200, 5000, 40),
        1 => build_camera_1(), // bouncing_spheres
        2 => build_camera_2(), // checkered_spheres
        3 => build_camera_3(), // earth
        4 => build_camera_4(), // perlin_spheres
        5 => build_camera_5(), // quads
        6 => build_camera_6(), // simple_light
        7 => build_camera_7(), // cornell_box
        8 => build_camera_8(), // cornell_smoke
        9 => build_camera_9(800, 10000, 40), // final scene
        10 => build_camera_9(400, 2000, 10), // final scene test
        _ => panic!("Not matched"),
    };
    let world = match TYPE {
        -1 => build_final_world(),
        0 => build_final_world(), 
        1 => build_world_1(),
        2 => build_world_2(),
        3 => build_world_3(),
        4 => build_world_4(),
        5 => build_world_5(),
        6 => build_world_6(),
        7 => build_world_7(),
        8 => build_world_8(),
        9 => build_world_9(),
        10 => build_world_9(), // final scene test
        _ => panic!("Not matched"),
    };

    let img = cam.render(&world);

    tail_process(img, parameters, "fAKe");
}


