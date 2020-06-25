
use crate::vector::Vec3;

pub struct Light {
    pub pos: Vec3,
}

impl Light {
    pub fn from(pos: Vec3) -> Self {
        Light { pos }
    }
}