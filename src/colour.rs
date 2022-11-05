use crate::vec3::Vec3;
use image::Rgb;

pub fn get_colour(colour: Vec3, samples_per_pixel: i32) -> Rgb<u8> {
    let scale = 1.0 / samples_per_pixel as f32;
    let r = colour.e[0] * scale;
    let g = colour.e[1] * scale;
    let b = colour.e[2] * scale;

    let ir: u8 = (256.0 * f32::clamp(r, 0.0, 0.999)) as u8;
    let ig: u8 = (256.0 * f32::clamp(g, 0.0, 0.999)) as u8;
    let ib: u8 = (256.0 * f32::clamp(b, 0.0, 0.999)) as u8;

    Rgb([ir, ig, ib])
}
