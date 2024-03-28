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
# `ac-power`

Reference frames and transforms for ac power analysis.

`ac-power` is a `#![no_std]` library for creating and manipulating [ac](https://en.wikipedia.org/wiki/AC_power) power signals in commonly used reference frames for conducting ac power analysis and signal processing.  The crate is designed for `#![no_std]` so can be used in microcontrollers for devices such as inverters and power meters which need to process ac power signals.

# How to use


```rust

use ac_power::reference_frames::{Abc, Dq0, AlphaBeta0};
use ac_power::trig::{Theta, cos_sin};

let abc = Abc{a: 100.0, b: 200.0, c:300.0};
let alpha_beta_zero = AlphaBeta0::from(abc);

let theta = Theta::from_degrees(170.0);
let (cos, sin) = cos_sin(theta);

let dq0 = abc.to_dq0(cos, sin);

```

The crate supports 5 difference reference frames

1.  [Abc](crate::reference_frames::abc::Abc)
2.  [AlphaBeta](crate::reference_frames::alpha_beta::AlphaBeta)
3.  [AlphaBeta0](crate::reference_frames::alpha_beta::AlphaBeta0)
4.  [Dq](crate::reference_frames::dq::Dq)
5.  [Dq0](crate::reference_frames::dq::Dq0)

*/

#![cfg_attr(not(test), no_std)]

pub mod constants;
pub mod reference_frames;
pub mod transforms;
pub mod trig;

#[cfg(test)]
mod tests {}
