use crate::constants::PI;

use core::convert::From;
use core::fmt;
use core::ops::AddAssign;

#[derive(Copy, Clone, PartialEq)]
pub struct Theta(i32);

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

impl AddAssign<i32> for Theta {
    fn add_assign(&mut self, rhs: i32) {
        self.0 = self.0.wrapping_add(rhs);
    }
}

impl AddAssign<Theta> for Theta {
    fn add_assign(&mut self, rhs: Theta) {
        self.0 = self.0.wrapping_add(rhs.0);
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
    fn theta() {
        let theta = Theta::from_degrees(90.0);
        assert_eq!(i32::from(theta), 1073741824);

        let theta = Theta::from_radians(0.5 * PI);
        assert_eq!(i32::from(theta), 1073741824);
    }
}
