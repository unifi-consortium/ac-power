use crate::{
    impl_trig_ops,
    trig::{Cos, Sin},
};
use core::ops::{Mul, MulAssign};
use derive_more::{Add, AddAssign, Neg, Sub};

/// A newtype representing an electric voltage (wraps f32)
#[derive(Neg, AddAssign, Add, Sub, Debug, Copy, Clone, PartialEq)]
pub struct Voltage(f32);

/// A newtype representing an electric current (wraps f32)
#[derive(Neg, AddAssign, Add, Sub, Debug, Copy, Clone, PartialEq)]
pub struct Current(f32);

/// A newtype representing an electric power (wraps f32)
#[derive(Neg, AddAssign, Add, Sub, Debug, Copy, Clone, PartialEq)]
pub struct Power(f32);

/// A newtype representing an electric impedance (wraps f32)
#[derive(Neg, AddAssign, Add, Sub, Debug, Copy, Clone, PartialEq)]
pub struct Impedance(f32);

// derive the trig multiplications
impl_trig_ops!(Sin, Voltage);
impl_trig_ops!(Cos, Voltage);
impl_trig_ops!(Sin, Current);
impl_trig_ops!(Cos, Current);
impl_trig_ops!(Sin, Power);
impl_trig_ops!(Cos, Power);
impl_trig_ops!(Sin, Impedance);
impl_trig_ops!(Cos, Impedance);

impl From<f32> for Voltage {
    fn from(number: f32) -> Voltage {
        Voltage(number)
    }
}

impl From<Voltage> for f32 {
    fn from(voltage: Voltage) -> f32 {
        voltage.0
    }
}

impl From<f32> for Current {
    fn from(number: f32) -> Current {
        Current(number)
    }
}

impl From<Current> for f32 {
    fn from(current: Current) -> f32 {
        current.0
    }
}

impl From<f32> for Power {
    fn from(number: f32) -> Power {
        Power(number)
    }
}

impl From<Power> for f32 {
    fn from(power: Power) -> f32 {
        power.0
    }
}

impl From<f32> for Impedance {
    fn from(number: f32) -> Impedance {
        Impedance(number)
    }
}

impl From<Impedance> for f32 {
    fn from(impedance: Impedance) -> f32 {
        impedance.0
    }
}

impl Mul<f32> for Voltage {
    fn mul(self, rhs: f32) -> Voltage {
        Voltage(self.0 * rhs)
    }
    type Output = Voltage;
}

impl Mul<Voltage> for f32 {
    fn mul(self, rhs: Voltage) -> Voltage {
        Voltage(self * rhs.0)
    }
    type Output = Voltage;
}

impl Mul<f32> for Current {
    fn mul(self, rhs: f32) -> Current {
        Current(self.0 * rhs)
    }
    type Output = Current;
}

impl Mul<Current> for f32 {
    fn mul(self, rhs: Current) -> Current {
        Current(self * rhs.0)
    }
    type Output = Current;
}

impl Mul<f32> for Power {
    fn mul(self, rhs: f32) -> Power {
        Power(self.0 * rhs)
    }
    type Output = Power;
}

impl Mul<Power> for f32 {
    fn mul(self, rhs: Power) -> Power {
        Power(self * rhs.0)
    }
    type Output = Power;
}

impl Mul<f32> for Impedance {
    fn mul(self, rhs: f32) -> Impedance {
        Impedance(self.0 * rhs)
    }
    type Output = Impedance;
}

impl Mul<Impedance> for f32 {
    fn mul(self, rhs: Impedance) -> Impedance {
        Impedance(self * rhs.0)
    }
    type Output = Impedance;
}

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
