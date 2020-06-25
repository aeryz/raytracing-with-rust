

use std::ops;

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn from(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            x, y, z
        }
    }

    pub fn negative(&self) -> Self {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(
            self.x * self.x + self.y * self.y + self.z * self.z
        )
    }

    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    pub fn normalized(&self) -> Self {
        let m = f32::sqrt(
            self.x * self.x + self.y * self.y + self.z * self.z
        );
        Vec3 {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m
        }
    }

    pub fn normalize(&mut self) {
        let m = f32::sqrt(
            self.x * self.x + self.y * self.y + self.z * self.z
        );
        self.x /= m;
        self.y /= m;
        self.z /= m;
    }

    pub fn scalar_mul(&self, s: f32) -> Self {
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s
        }
    }

    pub fn scalar_div(&self, s: f32) -> Self {
        Vec3 {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s
        }
    }
}

impl ops::Add for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Mul for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl ops::Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}