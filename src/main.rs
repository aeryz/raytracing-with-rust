extern crate image;

use image::png::PNGEncoder;
use image::ColorType;
use rust_tracer::camera::Camera;
use rust_tracer::light::Light;
use rust_tracer::ray::Ray;
use rust_tracer::sphere::Sphere;
use rust_tracer::vector::Vec3;
use std::f32;
use std::fs::File;

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;

    Ok(())
}

fn nearest_intersection_index(intersections: &Vec<f32>) -> Option<usize> {
    let mut last = f32::MAX;
    let mut ret_index: Option<usize> = None;
    for (index, int) in intersections.iter().enumerate() {
        if !int.is_sign_negative() && f32::lt(&int, &last) {
            last = *int;
            ret_index = Some(index);
        }
    }
    ret_index
}

fn any_sphere_before_light(spheres: &Vec<Sphere>, ray: &Ray, ray_magnitude: f32) -> bool {
    for sp in spheres {
        match sp.intersects(ray) {
            f if !f.is_sign_negative() && f < ray_magnitude => return true,
            _ => continue,
        }
    }
    false
}

fn get_color_at(
    spheres: &Vec<Sphere>,
    intersection_sphere: &Sphere,
    intersection_pos: &Vec3,
    light: &Light,
) -> u8 {
    let normal = intersection_sphere.normal_at(intersection_pos);
    let ray_origin = intersection_pos + &normal.scalar_div(1000.0);
    let ray_direction = &light.pos - &ray_origin;
    let ray_magnitude = ray_direction.magnitude();
    let ray = Ray::from(ray_origin, ray_direction.clone());

    if any_sphere_before_light(spheres, &ray, ray_magnitude) {
        return 0;
    }

    match ray.direction.dot(&normal) {
        f if f.is_sign_positive() => (f * 255.0) as u8,
        _ => 0,
    }
}

fn main() {
    let camera = Camera::new(
        Vec3::from(3.0, 1.5, -4.0),
        Vec3::from(0.0, 1.0, 0.0),
        Vec3::new(),
    );

    let width = 800;
    let height = 600;

    let mut spheres: Vec<Sphere> = Vec::new();
    spheres.push(Sphere::from(Vec3::from(1.25, -0.25, 0.0), 0.5));
    spheres.push(Sphere::from(Vec3::from(0.0, 0.0, 0.0), 1.0));
    spheres.push(Sphere::from(Vec3::from(0.0, -100.0, 0.0), 99.0));
    spheres.push(Sphere::from(Vec3::from(0.0, 0.0, 0.0), 20.0));

    let mut x_offset: f32;
    let mut y_offset: f32;
    let mut colors: Vec<u8> = Vec::new();
    colors.resize(width * height, 0);
    let light = Light::from(Vec3::from(5.0, 5.0, 5.0));

    for x in 0..width as i32 {
        for y in 0..height as i32 {
            let width = width as f32;
            let height = height as f32;
            if width > height {
                x_offset = ((x as f32) - width / 2.0) / height + 0.5;
                y_offset = (y as f32) / height;
            } else if height > width {
                x_offset = (x as f32) / width;
                y_offset = ((y as f32) - width / 2.0) / width + 0.5;
            } else {
                x_offset = (x as f32) / width;
                y_offset = (y as f32) / height;
            }
            let cam_ray = camera.ray_with_offset(x_offset, y_offset);

            let mut intersections: Vec<f32> = Vec::new();
            for sp in &spheres {
                intersections.push(sp.intersects(&cam_ray));
            }

            let color: u8 = match nearest_intersection_index(&intersections) {
                None => 0,
                Some(index) => {
                    let intersection_pos =
                        &cam_ray.origin + &cam_ray.direction.scalar_mul(intersections[index]);
                    get_color_at(&spheres, &spheres[index], &intersection_pos, &light)
                }
            };

            colors[(y * width as i32 + x) as usize] = color;
        }
    }

    write_image("out.png", &colors, (width as usize, height as usize))
        .expect("Write to file error.");
}
