use crate::vector::Vec3;
use crate::ray::Ray;


pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn from(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }

    pub fn intersects(&self, ray: &Ray) -> f32 {
        let q = &ray.origin - &self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&q);
        let c = &q.dot(&q) - self.radius * self.radius;
        let d = b * b - 4.0 * a * c; // discriminant

        return if f32::lt(&d, &0.0) {
            -1.0
        } else if f32::eq(&d, &0.0) {
            -b / (2.0 * a) - 0.00001
        } else {
            (-b - f32::sqrt(d)) / (2.0 * a) - 0.00001
        }
    }

    pub fn normal_at(&self, pos: &Vec3) -> Vec3 {
        let mut normal = pos - &self.center;
        normal.normalize();
        normal
    }
}