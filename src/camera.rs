use crate::ray::Ray;
use crate::vector::Vec3;

pub struct Camera {
    pub pos: Vec3,
    pub up: Vec3,
    pub right: Vec3,
}

impl Camera {
    pub fn new(pos: Vec3, mut up: Vec3, look_at: Vec3) -> Self {
        let direction = (&look_at - &pos).normalized();
        up.normalize();
        let right = up.cross(&direction).normalized();
        up = right.cross(&direction);
        Camera { pos, up, right }
    }

    pub fn ray_with_offset(&self, x: f32, y: f32) -> Ray {
        let direction = self.up.cross(&self.right).negative();
        Ray::from(
            self.pos.clone(),
            &direction + &(&self.up.scalar_mul(1.0 - y - 0.5) + &self.right.scalar_mul(x - 0.5)),
        )
    }
}
