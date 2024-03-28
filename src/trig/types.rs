use crate::constants::PI;
use core::fmt;
use derive_more::Add;

#[derive(Copy, Clone, PartialEq)]
pub struct Theta(pub i32);

// #[derive(Copy, Clone, PartialEq, Add)]
// pub struct Sin(f32);

// #[derive(Copy, Clone, PartialEq, Add)]
// pub struct Cos(f32);

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
