use crate::vec3::Vec3;
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

trait Hittable {
    pub fn hit(r: &Ray, t_min: f32, rec: &HitRecord) -> bool {
        false
    }
}
