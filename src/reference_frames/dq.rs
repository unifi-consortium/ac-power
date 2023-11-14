use crate::trig::rotate;
use core::ops::{Add, Sub};
use fixed::types::I1F31;
use fixed::FixedI32;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dq<const FRAC: i32> {
    pub d: FixedI32<FRAC>,
    pub q: FixedI32<FRAC>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dq0<const FRAC: i32> {
    pub d: FixedI32<FRAC>,
    pub q: FixedI32<FRAC>,
    pub zero: FixedI32<FRAC>,
}

impl<const FRAC: i32> Add<Dq<FRAC>> for Dq<FRAC> {
    fn add(self, other: Dq<FRAC>) -> Dq<FRAC> {
        let d = self.d + other.d;
        let q = self.q + other.q;
        Dq { d, q }
    }
    type Output = Dq<FRAC>;
}

impl<const FRAC: i32> Add<Dq0<FRAC>> for Dq<FRAC> {
    fn add(self, other: Dq0<FRAC>) -> Dq0<FRAC> {
        let d = self.d + other.d;
        let q = self.q + other.q;
        let zero = other.zero;
        Dq0 { d, q, zero }
    }
    type Output = Dq0<FRAC>;
}

impl<const FRAC: i32> Add<Dq0<FRAC>> for Dq0<FRAC> {
    fn add(self, other: Dq0<FRAC>) -> Dq0<FRAC> {
        let d = self.d + other.d;
        let q = self.q + other.q;
        let zero = self.zero + other.zero;
        Self { d, q, zero }
    }
    type Output = Dq0<FRAC>;
}

impl<const FRAC: i32> Add<Dq<FRAC>> for Dq0<FRAC> {
    fn add(self, other: Dq<FRAC>) -> Dq0<FRAC> {
        let d = self.d + other.d;
        let q = self.q + other.q;
        let zero = self.zero;
        Self { d, q, zero }
    }
    type Output = Dq0<FRAC>;
}

impl<const FRAC: i32> Sub<Dq<FRAC>> for Dq<FRAC> {
    fn sub(self, other: Dq<FRAC>) -> Dq<FRAC> {
        let d = self.d - other.d;
        let q = self.q - other.q;
        Self { d, q }
    }
    type Output = Dq<FRAC>;
}

impl<const FRAC: i32> Sub<Dq0<FRAC>> for Dq<FRAC> {
    fn sub(self, other: Dq0<FRAC>) -> Dq0<FRAC> {
        let d = self.d - other.d;
        let q = self.q - other.q;
        let zero = -other.zero;
        Dq0 { d, q, zero }
    }
    type Output = Dq0<FRAC>;
}

impl<const FRAC: i32> Sub<Dq0<FRAC>> for Dq0<FRAC> {
    fn sub(self, other: Dq0<FRAC>) -> Dq0<FRAC> {
        let d = self.d - other.d;
        let q = self.q - other.q;
        let zero = self.zero - other.zero;
        Self { d, q, zero }
    }
    type Output = Dq0<FRAC>;
}

impl<const FRAC: i32> Sub<Dq<FRAC>> for Dq0<FRAC> {
    fn sub(self, other: Dq<FRAC>) -> Dq0<FRAC> {
        let d = self.d - other.d;
        let q = self.q - other.q;
        let zero = self.zero;
        Self { d, q, zero }
    }
    type Output = Dq0<FRAC>;
}

impl<const FRAC: i32> Dq<FRAC> {
    pub const ZERO: Dq<FRAC> = Dq::<FRAC> {
        d: FixedI32::<FRAC>::ZERO,
        q: FixedI32::<FRAC>::ZERO,
    };
    pub fn rotate(&self, cos: I1F31, sin: I1F31) -> Dq<FRAC> {
        let (d, q) = rotate(self.d, self.q, cos, sin);
        Dq { d, q }
    }
}
