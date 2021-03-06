use std::ops;

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    #[inline]
    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    #[inline]
    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub fn cross(&self, rhs: &Vec3) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn normalized(&self) -> Self {
        let m = f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        Vec3 {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
        }
    }

    pub fn normalize(&mut self) {
        let m = f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        self.x /= m;
        self.y /= m;
        self.z /= m;
    }
}

impl ops::Add for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<f32> for &Vec3{
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    pub fn test_cross() {
        let vec1 = Vec3::new(3.16, -1.142, 22.11);
        let vec2 = Vec3::new(-34.41, 22.3, 13.4);
        let cross = vec1.cross(&vec2);
        assert!(cross.x.eq(&-508.3558));
        assert!(cross.y.eq(&-803.1491));
        assert!(cross.z.eq(&31.171783));
    }

    #[test]
    pub fn test_dot() {
        let vec1 = Vec3::new(1.2, -3.4, 16.2);
        let vec2 = Vec3::new(65.0, 0.0, -72.3);
        let dot = vec1.dot(&vec2);
        assert_eq!(dot, -1093.2601);
    }
}
