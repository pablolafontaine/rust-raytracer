mod colour;
mod hittable;
mod ray;
mod vec3;

use hittable::{HitRecord, Hittable, HittableList, Sphere};
use ray::Ray;
use vec3::Vec3;

fn ray_colour(r: Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(&r, 0.0, f32::INFINITY, &mut rec) {
        return (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction: Vec3 = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.e[1] + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as i32;

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

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    let ori: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        ori - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f32) / (IMAGE_WIDTH - 1) as f32;
            let v = (j as f32) / (IMAGE_HEIGHT - 1) as f32;
            let r: Ray = Ray {
                origin: ori,
                direction: (lower_left_corner + horizontal * u + vertical * v - ori),
            };
            let pixel_colour = ray_colour(r, &world);
            colour::write_colour(pixel_colour);
        }
    }
}
