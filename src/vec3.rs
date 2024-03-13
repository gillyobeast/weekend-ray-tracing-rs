use std::ops::{Add, Div, Mul, Neg};

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

// dot product
// impl<T> Mul<Vec3<T>> for Vec3<T>
// where
//     T: Mul<Output = T>,
//     T: Add<Output = T>,
// {
//     type Output = T;
//
//     fn mul(self, other: Vec3<T>) -> T {
//         self.x * other.x + self.y * other.y + self.z * other.z
//     }
// }
