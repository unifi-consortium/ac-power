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
#![feature(const_trait_impl)]

pub mod constants;
pub mod newtypes;
mod ops;
pub mod reference_frames;
mod transforms;
pub mod trig;
// pub mod wavegen;

#[cfg(test)]
mod tests {}
