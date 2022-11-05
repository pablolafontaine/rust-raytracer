mod camera;
mod colour;
mod hittable;
mod ray;
mod vec3;

use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList, Sphere};
use image::{ImageBuffer, RgbImage};
use ray::Ray;
use rayon::prelude::*;
use std::time::Instant;
use vec3::Vec3;

fn ray_colour(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    let mut rec = HitRecord::default();
    if depth <= 0 {
        return Vec3::default();
    }
    if world.hit(&r, 0.001, f32::INFINITY, &mut rec) {
        let target = rec.p + Vec3::random_in_hemisphere(rec.normal);
        return ray_colour(
            &Ray {
                origin: rec.p,
                direction: target - rec.p,
            },
            world,
            depth - 1,
        ) * 0.5;
    }
    let unit_direction: Vec3 = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.e[1] + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let now = Instant::now();
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1920;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: i32 = 50;
    const MAX_DEPTH: i32 = 50;
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // World

    let mut l: Vec<Box<dyn Hittable>> = Vec::new();
    l.push(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    l.push(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));
    let world: HittableList = HittableList { list: l };

    // Camera

    let cam: Camera = Camera::camera();

    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(i, j, pixel)| {
            let mut pixel_colour = Vec3::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + fastrand::f32()) / (IMAGE_WIDTH - 1) as f32;
                let v = ((IMAGE_HEIGHT as f32) - j as f32 + fastrand::f32())
                    / (IMAGE_HEIGHT - 1) as f32;
                let r = &cam.get_ray(u, v);
                pixel_colour = pixel_colour + ray_colour(r, &world, MAX_DEPTH);
            }
            *pixel = colour::get_colour(pixel_colour, SAMPLES_PER_PIXEL);
        });

    img.save("image.png").unwrap();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
