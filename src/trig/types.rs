use crate::constants::PI;
use core::convert::From;
use core::fmt;
use core::ops::{Mul, MulAssign, Sub};
use core::panic;

#[derive(Copy, Clone, PartialEq)]
pub struct Theta(pub i32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sin(f32);

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl From<Sin> for f32 {
    fn from(item: Sin) -> Self {
        item.0
    }
}

impl From<Cos> for f32 {
    fn from(item: Cos) -> Self {
        item.0
    }
}

impl From<f32> for Sin {
    fn from(item: f32) -> Self {
        if item > 1.0 || item < -1.0 {
            panic!("A Sin type must be between -1.0 and +1.0");
        }
        Self(item)
    }
}

impl From<f32> for Cos {
    fn from(item: f32) -> Self {
        if item > 1.0 || item < -1.0 {
            panic!("A Cos type must be between -1.0 and +1.0");
        }
        Self(item)
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
        theta.into()
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
        theta.into()
    }

    pub fn to_radians(&self) -> f32 {
        (2.0 * PI) * (self.0 as f32) / (u32::MAX as f32)
    }
}

impl From<i32> for Theta {
    fn from(item: i32) -> Self {
        Self(item)
    }
}

impl From<Theta> for i32 {
    fn from(item: Theta) -> Self {
        item.0
    }
}

impl fmt::Debug for Theta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "∠{}°", self.to_degrees())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn panics() {
        let result = std::panic::catch_unwind(|| Sin::from(1.1));
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| Sin::from(-1.1));
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| Cos::from(1.1));
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| Cos::from(-1.1));
        assert!(result.is_err());
    }

    #[test]
    fn theta() {
        let theta = Theta::from_degrees(90.0);
        assert_eq!(i32::from(theta), 1073741824);

        let theta = Theta::from_radians(0.5 * PI);
        assert_eq!(i32::from(theta), 1073741824);
    }

    #[test]
    fn multiplies() {
        let cos = Cos::from(1.0);
        let sin = Sin::from(1.0);

        let x = 2.0;
        assert_eq!(x * cos, x);
        assert_eq!(x * sin, x);

        let mut y = x;
        y *= cos;
        assert_eq!(x, y);
        y *= sin;
        assert_eq!(x, y);
    }
}
