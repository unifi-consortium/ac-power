use crate::newtypes::{Current, Power, Voltage};
use crate::trig::{Cos, Sin};
use core::ops::{Add, Mul, Neg, Sub};

/* Generic type with a trait bound for acceptable number types for use with reference frame structures */
// pub trait Num<Rhs = Self, Output = Self>: Add<Rhs, Output = Output>
//                                         + Sub<Rhs, Output = Output>
//                                         + Mul<f32, Output = Output>
//                                         + Div<Rhs, Output = Output>
//                                         + Rem<Rhs, Output = Output>
//                                         + Neg<Output = Output>
//                                         + From<f32>
//                                         + Copy
pub trait Num:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<f32, Output = Self>
    + Mul<Cos, Output = Self>
    + Mul<Sin, Output = Self>
    + Neg<Output = Self>
    + From<f32>
    + Copy
{
}

impl Num for f32 {}
impl Num for Voltage {}
impl Num for Current {}
impl Num for Power {}
