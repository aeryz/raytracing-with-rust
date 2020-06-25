use crate::vector::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn from(origin: Vec3, mut direction: Vec3) -> Self {
        direction.normalize();
        Ray { origin, direction }
    }
}