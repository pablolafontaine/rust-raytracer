use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Default)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

/*impl HitRecord {
    pub fn set_p(&mut self, val: Vec3) {
        self.p = val
    }
    pub fn set_t(&mut self, val: f32) {
        self.t = val
    }
    pub fn set_normal(&mut self, val: Vec3) {
        self.normal = val
    }
}*/

pub trait Hittable: Send + Sync {
    fn hit(&self, _r: &Ray, _t_min: f32, _t_max: f32, _rec: &mut HitRecord) -> bool {
        false
    }
    fn set_normal(&self, outward_normal: &Vec3, front_face: bool) -> Vec3 {
        if front_face {
            return *outward_normal;
        } else {
            return *outward_normal * -1.0;
        };
    }
}

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.list {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.t = closest_so_far;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
            }
        }
        hit_anything
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        let ptemp = r.point_at_parameter(root);
        let outward_normal = (ptemp - self.center) / self.radius;
        let fftemp = Vec3::dot(r.direction, outward_normal) < 0.0;
        *rec = HitRecord {
            p: ptemp,
            t: root,
            normal: self.set_normal(&outward_normal, fftemp),
        };
        true
    }
}
