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

use crate::constants::{ONE_HALF, SQRT_3_OVER_2};
use crate::number::Num;
use crate::trig::{rotate, Cos, Sin, UnitVector};
use crate::{Current, Impedance, Voltage};
use core::ops::{Add, Mul, Neg, Sub};
#[allow(unused_imports)]
use num_traits::Float;

fn inv_sqrt(x: f32) -> f32 {
    let i = x.to_bits();
    let i = 0x5f375a86 - (i >> 1);
    let y = f32::from_bits(i);

    y * (1.5 - 0.5 * x * y * y)
}

/// Balanced rotating reference frame
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dq<T> {
    pub d: T,
    pub q: T,
}

/// Unbalanced rotating reference frame
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dq0<T> {
    pub d: T,
    pub q: T,
    pub zero: T,
}

impl<T: Num> Neg for Dq<T> {
    fn neg(self) -> Dq<T> {
        let d = -self.d;
        let q = -self.q;
        Dq { d, q }
    }
    type Output = Dq<T>;
}

impl<T: Num> Neg for Dq0<T> {
    fn neg(self) -> Dq0<T> {
        let d = -self.d;
        let q = -self.q;
        let zero = -self.zero;
        Dq0 { d, q, zero }
    }
    type Output = Dq0<T>;
}

impl<T: Add<Output = T>> Add<Dq<T>> for Dq<T> {
    fn add(self, other: Dq<T>) -> Dq<T> {
        let d = self.d + other.d;
        let q = self.q + other.q;
        Dq { d, q }
    }
    type Output = Dq<T>;
}

impl<T: Add<Output = T>> Add<Dq0<T>> for Dq<T> {
    fn add(self, other: Dq0<T>) -> Dq0<T> {
        let d = self.d + other.d;
        let q = self.q + other.q;
        let zero = other.zero;
        Dq0 { d, q, zero }
    }
    type Output = Dq0<T>;
}

impl<T: Add<Output = T>> Add<Dq0<T>> for Dq0<T> {
    fn add(self, other: Dq0<T>) -> Dq0<T> {
        let d = self.d + other.d;
        let q = self.q + other.q;
        let zero = self.zero + other.zero;
        Self { d, q, zero }
    }
    type Output = Dq0<T>;
}

impl<T: Add<Output = T>> Add<Dq<T>> for Dq0<T> {
    fn add(self, other: Dq<T>) -> Dq0<T> {
        let d = self.d + other.d;
        let q = self.q + other.q;
        let zero = self.zero;
        Self { d, q, zero }
    }
    type Output = Dq0<T>;
}

impl<T: Sub<Output = T>> Sub<Dq<T>> for Dq<T> {
    fn sub(self, other: Dq<T>) -> Dq<T> {
        let d = self.d - other.d;
        let q = self.q - other.q;
        Self { d, q }
    }
    type Output = Dq<T>;
}

impl<T: Sub<Output = T> + Neg<Output = T>> Sub<Dq0<T>> for Dq<T> {
    fn sub(self, other: Dq0<T>) -> Dq0<T> {
        let d = self.d - other.d;
        let q = self.q - other.q;
        let zero = -other.zero;
        Dq0 { d, q, zero }
    }
    type Output = Dq0<T>;
}

impl<T: Sub<Output = T>> Sub<Dq0<T>> for Dq0<T> {
    fn sub(self, other: Dq0<T>) -> Dq0<T> {
        let d = self.d - other.d;
        let q = self.q - other.q;
        let zero = self.zero - other.zero;
        Self { d, q, zero }
    }
    type Output = Dq0<T>;
}

impl<T: Sub<Output = T>> Sub<Dq<T>> for Dq0<T> {
    fn sub(self, other: Dq<T>) -> Dq0<T> {
        let d = self.d - other.d;
        let q = self.q - other.q;
        let zero = self.zero;
        Self { d, q, zero }
    }
    type Output = Dq0<T>;
}

impl Mul<Impedance> for Dq<Current> {
    fn mul(self, other: Impedance) -> Dq<Voltage> {
        let d = self.d * other;
        let q = self.q * other;
        Dq::<Voltage> { d, q }
    }
    type Output = Dq<Voltage>;
}

impl Mul<Impedance> for Dq0<Current> {
    fn mul(self, other: Impedance) -> Dq0<Voltage> {
        let d = self.d * other;
        let q = self.q * other;
        let zero = self.zero * other;
        Dq0::<Voltage> { d, q, zero }
    }
    type Output = Dq0<Voltage>;
}

impl<T: Num> Mul<f32> for Dq<T> {
    fn mul(self, other: f32) -> Dq<T> {
        let d = self.d * other;
        let q = self.q * other;
        Dq::<T> { d, q }
    }
    type Output = Dq<T>;
}

impl<T: Num> Mul<f32> for Dq0<T> {
    fn mul(self, other: f32) -> Dq0<T> {
        let d = self.d * other;
        let q = self.q * other;
        let zero = self.zero * other;
        Dq0::<T> { d, q, zero }
    }
    type Output = Dq0<T>;
}

impl<T: Num> Dq<T> {
    pub fn zero() -> Self {
        Self {
            d: 0.0.into(),
            q: 0.0.into(),
        }
    }
    pub fn rotate(&mut self, phase: UnitVector) {
        (self.d, self.q) = rotate(self.d, self.q, phase.cos, phase.sin);
    }
    pub fn rotate_120(&mut self) {
        (self.d, self.q) = rotate(self.d, self.q, (-ONE_HALF).into(), SQRT_3_OVER_2.into());
    }
    pub fn rotate_240(&mut self) {
        (self.d, self.q) = rotate(self.d, self.q, (-ONE_HALF).into(), (-SQRT_3_OVER_2).into());
    }
    pub fn rotated(&self, phase: UnitVector) -> Dq<T> {
        let (d, q) = rotate(self.d, self.q, phase.cos, phase.sin);
        Dq { d, q }
    }
    pub fn rotated_120(&self) -> Dq<T> {
        let (d, q) = rotate(self.d, self.q, (-ONE_HALF).into(), SQRT_3_OVER_2.into());
        Dq { d, q }
    }
    pub fn rotated_240(&self) -> Dq<T> {
        let (d, q) = rotate(self.d, self.q, (-ONE_HALF).into(), (-SQRT_3_OVER_2).into());
        Dq { d, q }
    }

    pub fn normalize(&self) -> UnitVector {
        let d: f32 = self.d.into();
        let q: f32 = self.q.into();
        let scale = inv_sqrt(d * d + q * q);
        let cos = Cos::from(scale * d);
        let sin = Sin::from(scale * q);
        UnitVector { cos, sin }
    }

    pub fn abs(&self) -> T {
        let d: f32 = self.d.into();
        let q: f32 = self.q.into();
        (d * d + q * q).sqrt().into()
    }

    pub fn clipped(&self, limit: T) -> Self {
        // calculate square of limits
        let limit: f32 = limit.into();
        let d: f32 = self.d.into();
        let q: f32 = self.q.into();
        let limit_sqrd = limit * limit;

        let amplitude_sqrd = d * d + q * q;

        // if we are within the limit, return origial vector
        if amplitude_sqrd <= limit_sqrd {
            return *self;
        }

        // calculate scaling factor
        let scale = (limit_sqrd / amplitude_sqrd).sqrt();

        *self * scale
    }

    pub fn clip(&mut self, limit: T) {
        // calculate square of limits
        let limit: f32 = limit.into();
        let d: f32 = self.d.into();
        let q: f32 = self.q.into();
        let limit_sqrd = limit * limit;

        let amplitude_sqrd = d * d + q * q;

        // if we are within the limit, return origial vector
        if amplitude_sqrd <= limit_sqrd {
            return;
        }

        // calculate scaling factor
        let scale = (limit_sqrd / amplitude_sqrd).sqrt();

        *self = *self * scale;
    }
}

impl From<Dq<Voltage>> for Dq<Current> {
    fn from(item: Dq<Voltage>) -> Self {
        Self {
            d: f32::from(item.d).into(),
            q: f32::from(item.q).into(),
        }
    }
}

impl From<Dq<Voltage>> for Dq<f32> {
    fn from(item: Dq<Voltage>) -> Self {
        Self {
            d: item.d.into(),
            q: item.q.into(),
        }
    }
}

impl From<Dq<Current>> for Dq<f32> {
    fn from(item: Dq<Current>) -> Self {
        Self {
            d: item.d.into(),
            q: item.q.into(),
        }
    }
}

impl From<Dq<f32>> for Dq<Voltage> {
    fn from(item: Dq<f32>) -> Self {
        Self {
            d: item.d.into(),
            q: item.q.into(),
        }
    }
}

impl From<Dq<f32>> for Dq<Current> {
    fn from(item: Dq<f32>) -> Self {
        Self {
            d: item.d.into(),
            q: item.q.into(),
        }
    }
}

impl<T: From<f32>> Dq0<T> {
    pub fn zero() -> Self {
        Self {
            d: 0.0.into(),
            q: 0.0.into(),
            zero: 0.0.into(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Current, Impedance, Voltage};
    use approx::assert_abs_diff_eq;

    #[test]
    fn dq_test_clipping() {
        let mut current: Dq<Current> = Dq {
            d: 1.0.into(),
            q: 2.0.into(),
        };
        let limit: f32 = 2.0;
        let current_limit = Current::from(limit);
        let current_limited = current.clipped(current_limit);

        let d: f32 = current_limited.d.into();
        let q: f32 = current_limited.q.into();
        assert_abs_diff_eq!(d * d + q * q, limit * limit, epsilon = 0.0001);

        current.clip(current_limit);
        let d: f32 = current.d.into();
        let q: f32 = current.q.into();
        assert_abs_diff_eq!(d * d + q * q, limit * limit, epsilon = 0.0001);
    }

    #[test]
    fn dq_current_multiply_by_impedance() {
        let current: Dq<Current> = Dq {
            d: 1.0.into(),
            q: 2.0.into(),
        };
        let z = Impedance::from(2.0);
        let _voltage: Dq<Voltage> = current * z;
    }

    #[test]
    fn dq0_current_multiply_by_impedance() {
        let current: Dq0<Current> = Dq0 {
            d: 1.0.into(),
            q: 2.0.into(),
            zero: 3.0.into(),
        };
        let z = Impedance::from(2.0);
        let _voltage: Dq0<Voltage> = current * z;
    }
}
