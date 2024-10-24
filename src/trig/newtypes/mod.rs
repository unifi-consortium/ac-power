// Copyright 2023 Enphase Energy, Inc and Universal Interoperability for
// Grid-Forming Inverters (UNIFI) Consortium.
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

mod cos;
mod sin;
mod theta;

use crate::trig::{cos_sin, rotate};
use core::ops::{Add, Mul, Neg, Sub};
pub use cos::Cos;
#[allow(unused_imports)]
use num_traits::Float;
pub use sin::Sin;
pub use theta::Theta;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UnitVector {
    pub cos: Cos,
    pub sin: Sin,
}

impl Add<UnitVector> for UnitVector {
    fn add(self, rhs: UnitVector) -> Self {
        let (cos, sin) = rotate(f32::from(self.cos), f32::from(self.sin), rhs.cos, rhs.sin);
        Self {
            cos: cos.into(),
            sin: sin.into(),
        }
    }
    type Output = UnitVector;
}

impl Sub<UnitVector> for UnitVector {
    fn sub(self, rhs: UnitVector) -> Self {
        let (cos, sin) = rotate(f32::from(self.cos), f32::from(self.sin), rhs.cos, -rhs.sin);
        Self {
            cos: cos.into(),
            sin: sin.into(),
        }
    }
    type Output = UnitVector;
}

impl Neg for UnitVector {
    fn neg(self) -> Self {
        Self {
            cos: self.cos,
            sin: -self.sin,
        }
    }
    type Output = UnitVector;
}

impl From<Theta> for UnitVector {
    fn from(theta: Theta) -> Self {
        let (cos, sin) = cos_sin(theta);
        Self { cos, sin }
    }
}

impl UnitVector {
    pub fn from_degrees(degrees: f32) -> Self {
        let theta = Theta::from_degrees(degrees);
        theta.into()
    }

    pub fn from_radians(radians: f32) -> Self {
        let theta = Theta::from_radians(radians);
        theta.into()
    }

    pub fn avg(v0: Self, v1: Self) -> Self {
        let cos_sum = f32::from(v0.cos) + f32::from(v1.cos);
        let sin_sum = f32::from(v0.sin) + f32::from(v1.sin);
        let scale = (cos_sum * cos_sum + sin_sum * sin_sum).sqrt().recip();
        let cos = Cos::from(cos_sum * scale);
        let sin = Sin::from(sin_sum * scale);
        Self { cos, sin }
    }
}

// impliment the trig multiplies for f32, our base primitive
impl Mul<f32> for Sin {
    fn mul(self, rhs: f32) -> f32 {
        f32::from(self) * rhs
    }
    type Output = f32;
}

impl Mul<Sin> for f32 {
    fn mul(self, rhs: Sin) -> f32 {
        self * f32::from(rhs)
    }
    type Output = f32;
}

impl Mul<f32> for Cos {
    fn mul(self, rhs: f32) -> f32 {
        f32::from(self) * rhs
    }
    type Output = f32;
}

impl Mul<Cos> for f32 {
    fn mul(self, rhs: Cos) -> f32 {
        self * f32::from(rhs)
    }
    type Output = f32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn add_unit_vector() {
        let (r0, r1) = (75.3, 102.6);
        let phase0 = UnitVector::from_radians(r0);
        let phase1 = UnitVector::from_radians(r1);
        let phase2 = phase0 + phase1;
        assert_abs_diff_eq!(f32::from(phase2.sin), (r0 + r1).sin(), epsilon = 0.0001);
        assert_abs_diff_eq!(f32::from(phase2.cos), (r0 + r1).cos(), epsilon = 0.0001);
    }

    #[test]
    fn sub_unit_vector() {
        let (r0, r1) = (75.3, 102.6);
        let phase0 = UnitVector::from_radians(r0);
        let phase1 = UnitVector::from_radians(r1);
        let phase2 = phase0 - phase1;
        assert_abs_diff_eq!(f32::from(phase2.sin), (r0 - r1).sin(), epsilon = 0.0001);
        assert_abs_diff_eq!(f32::from(phase2.cos), (r0 - r1).cos(), epsilon = 0.0001);
    }

    #[test]
    fn neg_unit_vector() {
        let r0 = 75.3;
        let phase0 = UnitVector::from_radians(r0);
        let phase1 = -phase0;
        assert_abs_diff_eq!(f32::from(phase1.sin), (-r0).sin(), epsilon = 0.0001);
        assert_abs_diff_eq!(f32::from(phase1.cos), (-r0).cos(), epsilon = 0.0001);
    }
}
