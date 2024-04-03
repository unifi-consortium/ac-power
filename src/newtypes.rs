use crate::trig::{Cos, Sin};
use core::ops::{Mul, MulAssign};
use derive_more::{Add, AddAssign, From, Into, Neg, Sub};

macro_rules! impl_trig_ops {
    ($tr: ty, $nt: ty) => {
        impl Mul<$nt> for $tr {
            fn mul(self, rhs: $nt) -> $nt {
                rhs * self
            }
            type Output = $nt;
        }

        impl Mul<$tr> for $nt {
            fn mul(self, other: $tr) -> $nt {
                self * f32::from(other)
            }
            type Output = $nt;
        }

        impl MulAssign<$tr> for $nt {
            fn mul_assign(&mut self, other: $tr) {
                self.0 *= f32::from(other);
            }
        }
    };
}

macro_rules! impl_number {
    ($t:ty) => {
        impl Mul<f32> for $t {
            fn mul(self, other: f32) -> $t {
                (self.0 * other).into()
            }
            type Output = $t;
        }

        impl Mul<$t> for f32 {
            fn mul(self, other: $t) -> $t {
                (self * other.0).into()
            }
            type Output = $t;
        }

        impl_trig_ops!(Sin, $t);
        impl_trig_ops!(Cos, $t);
    };
}

/// A newtype representing an electric voltage (wraps f32)
#[derive(Neg, AddAssign, Add, Sub, Debug, Copy, Clone, PartialEq, From, Into)]
pub struct Voltage(f32);

/// A newtype representing an electric current (wraps f32)
#[derive(Neg, AddAssign, Add, Sub, Debug, Copy, Clone, PartialEq, From, Into)]
pub struct Current(f32);

/// A newtype representing an electric power (wraps f32)
#[derive(Neg, AddAssign, Add, Sub, Debug, Copy, Clone, PartialEq, From, Into)]
pub struct Power(f32);

/// A newtype representing an electric impedance (wraps f32)
#[derive(Neg, AddAssign, Add, Sub, Debug, Copy, Clone, PartialEq, From, Into)]
pub struct Impedance(f32);

// derive operations for the new-types
impl_number!(Voltage);
impl_number!(Current);
impl_number!(Power);
impl_number!(Impedance);

// derive special multiplies that yeild a unit change (i.e. Voltage * Current = Power)
impl Mul<Current> for Voltage {
    fn mul(self, rhs: Current) -> Power {
        Power(self.0 * rhs.0)
    }
    type Output = Power;
}

impl Mul<Voltage> for Current {
    fn mul(self, rhs: Voltage) -> Power {
        Power(self.0 * rhs.0)
    }
    type Output = Power;
}

impl Mul<Current> for Impedance {
    fn mul(self, rhs: Current) -> Voltage {
        Voltage(self.0 * rhs.0)
    }
    type Output = Voltage;
}

impl Mul<Impedance> for Current {
    fn mul(self, rhs: Impedance) -> Voltage {
        Voltage(self.0 * rhs.0)
    }
    type Output = Voltage;
}
