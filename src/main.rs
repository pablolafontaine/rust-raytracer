mod colour;
mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

fn hit_sphere(center: Vec3, radius: f32, r: Ray) -> f32 {
    let oc: Vec3 = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = Vec3::dot(oc, r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_colour(r: Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n: Vec3 = Vec3::unit_vector(r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0));
        return Vec3::new(n.e[0] + 1.0, n.e[1] + 1.0, n.e[2] + 1.0) * 0.5;
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

    // Camera

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f32) / (IMAGE_WIDTH - 1) as f32;
            let v = (j as f32) / (IMAGE_HEIGHT - 1) as f32;
            let r: Ray = Ray {
                origin: origin,
                direction: (lower_left_corner + horizontal * u + vertical * v - origin),
            };
            let pixel_colour = ray_colour(r);
            colour::write_colour(pixel_colour);
        }
    }
}
