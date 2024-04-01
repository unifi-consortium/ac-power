use crate::trig::rotate;
use crate::trig::{Cos, Sin};
use core::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dq {
    pub d: f32,
    pub q: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dq0 {
    pub d: f32,
    pub q: f32,
    pub zero: f32,
}

impl Add<Dq> for Dq {
    fn add(self, other: Dq) -> Dq {
        let d = self.d + other.d;
        let q = self.q + other.q;
        Dq { d, q }
    }
    type Output = Dq;
}

impl Add<Dq0> for Dq {
    fn add(self, other: Dq0) -> Dq0 {
        let d = self.d + other.d;
        let q = self.q + other.q;
        let zero = other.zero;
        Dq0 { d, q, zero }
    }
    type Output = Dq0;
}

impl Add<Dq0> for Dq0 {
    fn add(self, other: Dq0) -> Dq0 {
        let d = self.d + other.d;
        let q = self.q + other.q;
        let zero = self.zero + other.zero;
        Self { d, q, zero }
    }
    type Output = Dq0;
}

impl Add<Dq> for Dq0 {
    fn add(self, other: Dq) -> Dq0 {
        let d = self.d + other.d;
        let q = self.q + other.q;
        let zero = self.zero;
        Self { d, q, zero }
    }
    type Output = Dq0;
}

impl Sub<Dq> for Dq {
    fn sub(self, other: Dq) -> Dq {
        let d = self.d - other.d;
        let q = self.q - other.q;
        Self { d, q }
    }
    type Output = Dq;
}

impl Sub<Dq0> for Dq {
    fn sub(self, other: Dq0) -> Dq0 {
        let d = self.d - other.d;
        let q = self.q - other.q;
        let zero = -other.zero;
        Dq0 { d, q, zero }
    }
    type Output = Dq0;
}

impl Sub<Dq0> for Dq0 {
    fn sub(self, other: Dq0) -> Dq0 {
        let d = self.d - other.d;
        let q = self.q - other.q;
        let zero = self.zero - other.zero;
        Self { d, q, zero }
    }
    type Output = Dq0;
}

impl Sub<Dq> for Dq0 {
    fn sub(self, other: Dq) -> Dq0 {
        let d = self.d - other.d;
        let q = self.q - other.q;
        let zero = self.zero;
        Self { d, q, zero }
    }
    type Output = Dq0;
}

impl Dq {
    pub const ZERO: Dq = Dq { d: 0.0, q: 0.0 };
    pub fn rotate(&self, cos: Cos, sin: Sin) -> Dq {
        let (d, q) = rotate(self.d, self.q, cos, sin);
        Dq { d, q }
    }
}

impl Dq0 {
    pub const ZERO: Dq0 = Dq0 {
        d: 0.0,
        q: 0.0,
        zero: 0.0,
    };
}
