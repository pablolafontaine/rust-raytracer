use std::ops;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
    pub fn length_squared(self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }
    pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
        v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}
impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}
impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}
impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] / other, self.e[1] / other, self.e[2] / other],
        }
    }
}
