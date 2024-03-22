use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use rand::{random, thread_rng, Rng};

// unsure if pub is a good idea...
#[derive(Debug)]
pub(crate) struct Vec3 {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}
pub(crate) type Point3 = Vec3;

impl Vec3 {
    pub fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub(crate) const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub(crate) fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub(crate) fn unit_vector(self) -> Self {
        let x = self.length();
        self / x
    }

    pub(crate) fn random() -> Self {
        Self {
            x: random(),
            y: random(),
            z: random(),
        }
    }
    pub(crate) fn random_in_range(min: f64, max: f64) -> Self {
        let mut r = thread_rng();

        Self {
            x: r.gen_range(min..max),
            y: r.gen_range(min..max),
            z: r.gen_range(min..max),
        }
    }

    pub(crate) fn random_in_unit_sphere() -> Vec3 {
        loop {
            // get a random vector in the unit box
            let random = Vec3::random_in_range(-1.0, 1.0);
            if random.length_squared() < 1.0 {
                return random;
            }
        }
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
impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
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

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
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

/// dot product
impl Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
impl Mul<&Vec3> for &Vec3 {
    type Output = f64;

    fn mul(self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Vec3 {}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_product() {
        let vec = Vec3::origin();
        assert_eq!(vec * vec, 0.0);

        let vec = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(vec * vec, 3.0);

        let vec1 = Vec3::new(1.0, 0.0, 0.0);
        let vec2 = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(vec1 * vec2, 0.0);

        let vec1 = Vec3::new(1.0, 1.0, 0.0);
        assert_eq!(vec1 * vec2, 1.0);
    }

    #[test]
    fn add() {
        let vec = Vec3::origin();
        assert_eq!(vec + vec, vec);

        let vec = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(vec + vec, Vec3::new(2.0, 2.0, 2.0));

        assert_eq!(
            Vec3::new(1.0, 0.0, 0.0) + Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0)
        );

        assert_eq!(
            Vec3::new(1.0, 1.0, 0.0) + Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 2.0, 0.0)
        );
    }

    #[test]
    fn length() {
        let all_twos = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(all_twos.length_squared(), 12.0);
        assert_eq!(all_twos.length(), 12.0_f64.sqrt());

        let all_ones = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(all_ones.length_squared(), 3.0);
        assert_eq!(all_ones.length(), 3.0_f64.sqrt());
    }
}
