extern crate image;

use rust_tracer::vector::Vec3;
use rust_tracer::ray::Ray;
use rust_tracer::light::Light;
use rust_tracer::sphere::Sphere;
use std::f32;
use image::png::PNGEncoder;
use std::fs::File;
use image::ColorType;

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error>
{
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32,
                   ColorType::Gray(8))?;

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

fn get_color_at(spheres: &Vec<Sphere>, intersection_sphere: &Sphere, intersection_pos: &Vec3, light: &Light) -> u8 {
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
        _ => 0
    }
}

fn main() {
    let cam_pos = Vec3::from(3.0, 1.5, -4.0);
    let look_at = Vec3::new();

    let cam_dir = (&look_at - &cam_pos).normalized();
    let cam_right = Vec3::from(0.0, 1.0, 0.0).cross(&cam_dir).normalized();
    let cam_up = cam_right.negative().cross(&cam_dir);

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
    colors.resize(800 * 900, 0);
    let light = Light::from(Vec3::from(5.0, 5.0, 5.0));

    for x in 0..width {
        for y in 0..height {
            let width = width as f32;
            let height = height as f32;
            if width > height {
                x_offset = (x as f32 + 0.5) / height - (width - height) / (height * 2.0);
                y_offset = (y as f32 + 0.5) / height;
            } else if height > width {
                x_offset = (x as f32 + 0.5) / width;
                y_offset = (y as f32 + 0.5) / width - (height - width) / (width * 2.0);
            } else {
                x_offset = (x as f32 + 0.5) / width;
                y_offset = (y as f32 + 0.5) / height;
            }

            let cam_ray = Ray::from(
                cam_pos.clone(),
                &cam_dir + &(&cam_right.scalar_mul(x_offset - 0.5) + &cam_up.scalar_mul(y_offset - 0.5)));

            let mut intersections: Vec<f32> = Vec::new();
            for sp in &spheres {
                intersections.push(sp.intersects(&cam_ray));
            }

            let color: u8 = match nearest_intersection_index(&intersections) {
                None => 0,
                Some(index) => {
                    let intersection_pos = &cam_ray.origin + &cam_ray.direction.scalar_mul(intersections[index]);
                    get_color_at(&spheres, &spheres[index], &intersection_pos, &light)
                },
            };

            colors[(y * width as i32 + x) as usize] = color;
        }
    }

    write_image("out.png", &colors, (width as usize, height as usize))
        .expect("Write to file error.");

}
