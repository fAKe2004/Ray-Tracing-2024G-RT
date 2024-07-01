use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

#[derive(Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn norm_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.y * other.z - self.z * other.y, 
            -(self.x * other.z - self.z * other.x), 
            self.x * other.y - self.y * other.x)
    }

    pub fn func_cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
        Vec3::new(lhs.y * rhs.z - lhs.z * rhs.y, 
            -(lhs.x * rhs.z - lhs.z * rhs.x), 
            lhs.x * rhs.y - lhs.y * rhs.x)
    }

    pub fn elemul(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x * other.x,
                self.y * other.y,
                self.z * other.z)
    }

    pub fn func_elemul(lhs: Vec3, rhs: Vec3) -> Vec3 {
        Vec3::new(lhs.x * rhs.x,
            lhs.y * rhs.y,
            lhs.z * rhs.z)
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.norm();
        if len == 0.0 {
            panic!("Vec3::normalize: Attempt to normalize a zero vector.");
        }
        self.clone() / self.norm()
    }
}

pub type Point3 = Vec3;

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl Mul for Vec3 {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        } 
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        } 
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Copy for Vec3 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(3.0, 4.0, 5.0)
        )
    }

    #[test]
    fn test_add_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(3.0, 4.0, 5.0))
    }

    #[test]
    fn test_add_f64() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + 233.0,
            Vec3::new(234.0, 233.0, 232.0)
        )
    }

    // /*
    #[test]
    fn test_add_assign_f64() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += 233.0;
        assert_eq!(x, Vec3::new(234.0, 233.0, 232.0))
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) - Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(-1.0, -4.0, -7.0)
        )
    }

    #[test]
    fn test_sub_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x -= Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(-1.0, -4.0, -7.0))
    }

    #[test]
    fn test_sub_f64() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) - 1.0, Vec3::new(0.0, -1.0, -2.0))
    }

    #[test]
    fn test_sub_assign_f64() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x -= 1.0;
        assert_eq!(x, Vec3::new(0.0, -1.0, -2.0))
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) * Vec3::ones(), 0.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x *= 2.0;
        assert_eq!(x, Vec3::new(2.0, 0.0, -2.0));
    }

    #[test]
    fn test_mul_f64() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) * 1.0, Vec3::new(1.0, 0.0, -1.0));
    }

    #[test]
    fn test_div() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0) / 2.0, Vec3::new(0.5, -1.0, 0.0));
    }

    #[test]
    fn test_elemul() {
        assert_eq!(
            // Vec3::elemul(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0)),
            Vec3::new(1.0, 2.0, 3.0).elemul(&(Vec3::new(1.0, 2.0, 3.0))),
            Vec3::new(1.0, 4.0, 9.0)
        );
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            // Vec3::cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0)),
            Vec3::new(1.0, 2.0, 3.0).cross(&(Vec3::new(2.0, 3.0, 4.0))),
            Vec3::new(8.0 - 9.0, 6.0 - 4.0, 3.0 - 4.0)
        );
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0));
    }
    // */

    #[test]
    fn test_squared_length() {
        // assert_eq!(Vec3::new(1.0, 2.0, 3.0).squared_length(), 14.0 as f64);
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).norm_squared(), 14.0 as f64);
    }

    // /*
    #[test]
    fn test_length() {
        assert_eq!(
            // Vec3::new(3.0, 4.0, 5.0).length(),
            Vec3::new(3.0, 4.0, 5.0).norm(),
            ((3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0) as f64).sqrt()
        );
    }

    #[test]
    fn test_unit() {
        // assert_eq!(Vec3::new(233.0, 0.0, 0.0).unit(), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(Vec3::new(233.0, 0.0, 0.0).normalize(), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(
            // Vec3::new(-233.0, 0.0, 0.0).unit(),
            Vec3::new(-233.0, 0.0, 0.0).normalize(),
            Vec3::new(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    #[should_panic]
    fn test_unit_panic() {
        // Vec3::new(0.0, 0.0, 0.0).unit();
        Vec3::new(0.0, 0.0, 0.0).normalize();
    }
    // */
}
