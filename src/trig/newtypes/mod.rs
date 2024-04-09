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

mod cos;
mod sin;
mod theta;

use core::ops::Mul;
pub use cos::Cos;
pub use sin::Sin;
pub use theta::Theta;

// impliment the trig multiplies for f32, our base primitive
impl Mul<f32> for Sin {
    fn mul(self, rhs: f32) -> f32 {
        f32::from(self) * rhs
    }
    type Output = f32;
}

impl Mul<Sin> for f32 {
    fn mul(self, rhs: Sin) -> f32 {
        self * f32::from(rhs)
    }
    type Output = f32;
}

impl Mul<f32> for Cos {
    fn mul(self, rhs: f32) -> f32 {
        f32::from(self) * rhs
    }
    type Output = f32;
}

impl Mul<Cos> for f32 {
    fn mul(self, rhs: Cos) -> f32 {
        self * f32::from(rhs)
    }
    type Output = f32;
}
