use crate::trig::cos_sin;
use crate::trig::Sin;
use crate::trig::Theta;
use core::convert::From;

use core::ops::{Mul, MulAssign, Neg, Sub};
// use core::panic;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cos(f32);

impl Cos {
    pub fn from_theta(theta: Theta) -> Self {
        let (cos, _) = cos_sin(theta);
        cos
    }
    pub fn from_degrees(degrees: f32) -> Self {
        let theta = Theta::from_degrees(degrees);
        Self::from_theta(theta)
    }
    pub fn from_radians(radians: f32) -> Self {
        let theta = Theta::from_radians(radians);
        Self::from_theta(theta)
    }
}

impl From<i32> for Cos {
    fn from(item: i32) -> Self {
        Self((item as f32) / 2147483648.)
    }
}

impl Mul<f32> for Cos {
    fn mul(self, other: f32) -> f32 {
        self.0 * other
    }
    type Output = f32;
}

impl Mul<Cos> for Cos {
    fn mul(self, other: Cos) -> f32 {
        self.0 * other.0
    }
    type Output = f32;
}

impl Mul<Cos> for f32 {
    fn mul(self, other: Cos) -> f32 {
        self * other.0
    }
    type Output = f32;
}

impl Sub<Cos> for f32 {
    fn sub(self, other: Cos) -> f32 {
        self - other.0
    }
    type Output = f32;
}

impl MulAssign<Cos> for f32 {
    fn mul_assign(&mut self, other: Cos) {
        *self *= other.0;
    }
}

impl From<Cos> for f32 {
    fn from(item: Cos) -> Self {
        item.0
    }
}

impl From<f32> for Cos {
    fn from(item: f32) -> Self {
        // if item > 1.0 || item < -1.0 {
        //     panic!("A Cos type must be between -1.0 and +1.0");
        // }
        Self(item)
    }
}

impl Neg for Cos {
    fn neg(self) -> Self {
        Self(-self.0)
    }
    type Output = Self;
}

impl Mul<Sin> for Cos {
    fn mul(self, rhs: Sin) -> f32 {
        self.0 * rhs
    }
    type Output = f32;
}
