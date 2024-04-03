use crate::newtypes::{Current, Power, Voltage};
use crate::trig::{Cos, Sin};
use core::fmt::Debug;
use core::ops::{Add, AddAssign, Mul, Neg, Sub};

/// Generic type with a trait bound for acceptable number types for use with reference frame structures
pub trait Num:
    Add<Output = Self>
    + AddAssign<Self>
    + Sub<Output = Self>
    + Mul<f32, Output = Self>
    + Mul<Cos, Output = Self>
    + Mul<Sin, Output = Self>
    + Neg<Output = Self>
    + From<f32>
    + Into<f32>
    + Copy
    + Debug
{
}

impl Num for f32 {}
impl Num for Voltage {}
impl Num for Current {}
impl Num for Power {}
