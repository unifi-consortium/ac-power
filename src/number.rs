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

/*!
Definition of the numeric trait required for the reference frames elements
*/

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
