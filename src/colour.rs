use crate::vec3::Vec3;

pub fn write_colour(colour: Vec3, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f32;
    let r = colour.e[0] * scale;
    let g = colour.e[1] * scale;
    let b = colour.e[2] * scale;

    let ir: i32 = (256.0 * f32::clamp(r, 0.0, 0.999)) as i32;
    let ig: i32 = (256.0 * f32::clamp(g, 0.0, 0.999)) as i32;
    let ib: i32 = (256.0 * f32::clamp(b, 0.0, 0.999)) as i32;

    println!("{ir} {ig} {ib}");
}
