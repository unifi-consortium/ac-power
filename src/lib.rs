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

#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

pub mod constants;
mod newtypes;
pub mod number;
mod pq;
mod reference_frames;
pub mod trig;
pub mod wavegen;

pub use newtypes::{Current, Impedance, Power, Voltage};
pub use pq::Pq;
pub use reference_frames::{Abc, AlphaBeta, AlphaBeta0, Dq, Dq0, Polar};

#[cfg(test)]
mod tests {}
