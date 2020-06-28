use crate::ray::Ray;
use crate::vector::Vec3;

pub struct Camera {
    pub pos: Vec3,
    pub up: Vec3,
    pub right: Vec3,
}

impl Camera {
    pub fn new(mut pos: Vec3, mut up: Vec3, look_at: Vec3) -> Self {
        let rev_dir = (&pos - &look_at).normalized();
        let right = up.cross(&rev_dir).normalized();
        up = right.cross(&rev_dir).normalized().negative();
        Camera { pos, up, right }
    }

    pub fn ray_with_offset(&self, x: f32, y: f32) -> Ray {
        let direction = self.up.cross(&self.right);
        Ray::from(
            self.pos.clone(),
            &direction + &(&self.right.scalar_mul(x - 0.5) + &self.up.scalar_mul(1.0 - y - 0.5)),
        )
    }
}
