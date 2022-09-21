use crate::vec3::Vec3;

pub struct Sphere {}

impl Hittable for Sphere {
    fn hit(r: &Ray, t_min: f32, rec: &HitRecord) -> bool {}
}
