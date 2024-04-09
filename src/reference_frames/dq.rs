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

use crate::trig::rotate;
use crate::trig::{Cos, Sin};
use core::ops::{Add, Mul, Neg, Sub};

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

impl<
        T: Mul<Sin, Output = T>
            + Mul<Cos, Output = T>
            + Sub<Output = T>
            + Add<Output = T>
            + Copy
            + From<f32>,
    > Dq<T>
{
    pub fn zero() -> Self {
        Self {
            d: 0.0.into(),
            q: 0.0.into(),
        }
    }
    pub fn rotate(&self, cos: Cos, sin: Sin) -> Dq<T> {
        let (d, q) = rotate(self.d, self.q, cos, sin);
        Dq { d, q }
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
