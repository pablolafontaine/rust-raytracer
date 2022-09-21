use crate::vec3::Vec3;

pub fn write_colour(colour: Vec3) {
    let ir: i32 = (255.999 * colour.e[0]) as i32;
    let ig: i32 = (255.999 * colour.e[1]) as i32;
    let ib: i32 = (255.999 * colour.e[2]) as i32;

    println!("{ir} {ig} {ib}");
}
