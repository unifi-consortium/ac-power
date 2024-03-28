use crate::constants::PI;
use core::convert::From;
use core::fmt;
use core::ops::{Mul, MulAssign, Sub};
use derive_more::From as dmFrom;
use derive_more::Into as dmInto;

#[derive(Copy, Clone, PartialEq)]
pub struct Theta(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, dmFrom, dmInto)]
pub struct Sin(f32);

#[derive(Debug, Copy, Clone, PartialEq, dmFrom, dmInto)]
pub struct Cos(f32);

impl From<i32> for Sin {
    fn from(item: i32) -> Self {
        Self((item as f32) / 2147483648.)
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

impl Mul<f32> for Sin {
    fn mul(self, other: f32) -> f32 {
        self.0 * other
    }
    type Output = f32;
}

impl Mul<Cos> for Sin {
    fn mul(self, other: Cos) -> f32 {
        self.0 * other.0
    }
    type Output = f32;
}

impl Mul<Sin> for Cos {
    fn mul(self, other: Sin) -> f32 {
        self.0 * other.0
    }
    type Output = f32;
}

impl Mul<Sin> for Sin {
    fn mul(self, other: Sin) -> f32 {
        self.0 * other.0
    }
    type Output = f32;
}

impl Mul<Cos> for Cos {
    fn mul(self, other: Cos) -> f32 {
        self.0 * other.0
    }
    type Output = f32;
}

impl Mul<Sin> for f32 {
    fn mul(self, other: Sin) -> f32 {
        self * other.0
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

impl Sub<Sin> for f32 {
    fn sub(self, other: Sin) -> f32 {
        self - other.0
    }
    type Output = f32;
}

impl MulAssign<Sin> for f32 {
    fn mul_assign(&mut self, other: Sin) {
        *self *= other.0;
    }
}

impl MulAssign<Cos> for f32 {
    fn mul_assign(&mut self, other: Cos) {
        *self *= other.0;
    }
}

impl Theta {
    pub fn from_degrees(mut degrees: f32) -> Self {
        while degrees > 180.0 {
            degrees -= 360.0;
        }
        while degrees < -180.0 {
            degrees += 360.0
        }

        let theta = ((degrees / 360.0) * (u32::MAX as f32)) as i32;
        Self(theta)
    }

    pub fn to_degrees(&self) -> f32 {
        360.0 * (self.0 as f32) / (u32::MAX as f32)
    }

    pub fn from_radians(mut radians: f32) -> Self {
        while radians > PI {
            radians -= 2.0 * PI;
        }
        while radians < -PI {
            radians += 2.0 * PI;
        }

        let theta = ((radians / (2.0 * PI)) * (u32::MAX as f32)) as i32;
        Self(theta)
    }
}

impl fmt::Debug for Theta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "∠{}°", self.to_degrees())
    }
}
