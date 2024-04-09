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

/// Balanced stationary orthoganal reference frame (i.e. clarke)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AlphaBeta<T> {
    pub alpha: T,
    pub beta: T,
}

/// Unbalanced stationary orthoganal reference frame (i.e. clarke)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AlphaBeta0<T> {
    pub alpha: T,
    pub beta: T,
    pub zero: T,
}

impl<T: From<f32>> AlphaBeta<T> {
    pub fn zero() -> Self {
        Self {
            alpha: 0.0.into(),
            beta: 0.0.into(),
        }
    }
}

impl<T: From<f32>> AlphaBeta0<T> {
    pub fn zero() -> Self {
        Self {
            alpha: 0.0.into(),
            beta: 0.0.into(),
            zero: 0.0.into(),
        }
    }
}
