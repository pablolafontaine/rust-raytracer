use crate::ray::Ray;
use crate::vec3::Vec3;
pub struct Camera {
    ori: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn camera() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let hor = Vec3::new(viewport_width, 0.0, 0.0);
        let vert = Vec3::new(0.0, viewport_height, 0.0);
        let o = Vec3::new(0.0, 0.0, 0.0);
        Camera {
            ori: o,
            horizontal: hor,
            vertical: vert,
            lower_left_corner: o - hor / 2.0 - vert / 2.0 - Vec3::new(0.0, 0.0, focal_length),
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.ori,
            direction: (self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.ori),
        }
    }
}
