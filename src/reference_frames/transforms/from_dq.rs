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

use crate::number::Num;
use crate::reference_frames::{Abc, AlphaBeta, AlphaBeta0, Dq, Dq0};
use crate::trig::{shift_left_120, shift_right_120, Cos, Sin};

impl<T: Num> From<Dq<T>> for Dq0<T> {
    fn from(dq: Dq<T>) -> Self {
        Self {
            d: dq.d,
            q: dq.q,
            zero: 0.0.into(),
        }
    }
}

impl<T> From<Dq0<T>> for Dq<T> {
    fn from(dq0: Dq0<T>) -> Self {
        Self { d: dq0.d, q: dq0.q }
    }
}

impl<T: Num> Dq<T> {
    pub fn to_abc(&self, cos: Cos, sin: Sin) -> Abc<T> {
        /* sin and cos with 120 degree offsets */
        let (cos_m, sin_m) = shift_left_120(cos, sin);
        let (cos_p, sin_p) = shift_right_120(cos, sin);

        let a = (self.d * sin) + (self.q * cos);
        let b = (self.d * sin_m) + (self.q * cos_m);
        let c = (self.d * sin_p) + (self.q * cos_p);

        Abc { a, b, c }
    }

    pub fn to_alpha_beta(&self, cos: Cos, sin: Sin) -> AlphaBeta<T> {
        let alpha = (self.d * sin) + (self.q * cos);
        let beta = (self.q * sin) - (self.d * cos);

        AlphaBeta { alpha, beta }
    }

    pub fn to_alpha_beta_0(&self, cos: Cos, sin: Sin) -> AlphaBeta0<T> {
        let alpha = (self.d * sin) + (self.q * cos);
        let beta = (self.q * sin) - (self.d * cos);

        AlphaBeta0 {
            alpha,
            beta,
            zero: 0.0.into(),
        }
    }
}

impl<T: Num> Dq0<T> {
    pub fn to_abc(&self, cos: Cos, sin: Sin) -> Abc<T> {
        Dq {
            d: self.d,
            q: self.q,
        }
        .to_abc(cos, sin)
            + self.zero
    }

    pub fn to_alpha_beta(&self, cos: Cos, sin: Sin) -> AlphaBeta<T> {
        let alpha = (self.d * sin) + (self.q * cos);
        let beta = (self.q * sin) - (self.d * cos);

        AlphaBeta { alpha, beta }
    }

    pub fn to_alpha_beta_0(&self, cos: Cos, sin: Sin) -> AlphaBeta0<T> {
        let alpha = (self.d * sin) + (self.q * cos);
        let beta = (self.q * sin) - (self.d * cos);

        AlphaBeta0 {
            alpha,
            beta,
            zero: self.zero,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::trig::cos_sin;
    use crate::trig::Theta;
    use approx::assert_abs_diff_eq;

    #[test]
    fn dq0_to_abc() {
        let dq0: Dq0<f32> = Dq0 {
            d: 1.0,
            q: 2.0,
            zero: 3.0,
        };
        let (cos, sin) = cos_sin(Theta::from_degrees(45.0));
        let abc = dq0.to_abc(cos, sin);

        // verified against results from https://pypi.org/project/ClarkePark/
        assert_abs_diff_eq!(abc.a, 5.12132034, epsilon = 0.0001);
        assert_abs_diff_eq!(abc.b, 2.551712263, epsilon = 0.0001);
        assert_abs_diff_eq!(abc.c, 1.326967392, epsilon = 0.0001);
    }

    #[test]
    fn dq0_to_alpha_beta_0() {
        let dq0 = Dq0 {
            d: -0.7071067,
            q: 2.1213203435,
            zero: 3.0,
        };
        let (cos, sin) = cos_sin(Theta::from_degrees(45.0));
        let alpha_beta_0 = dq0.to_alpha_beta_0(cos, sin);

        // verified against results from https://pypi.org/project/ClarkePark/
        assert_abs_diff_eq!(alpha_beta_0.alpha, 1.0, epsilon = 0.0001);
        assert_abs_diff_eq!(alpha_beta_0.beta, 2.0, epsilon = 0.0001);
        assert_abs_diff_eq!(alpha_beta_0.zero, 3.0, epsilon = 0.0001);
    }

    #[test]
    fn dq_to_abc() {
        let dq = Dq { d: 1.0, q: 2.0 };
        let (cos, sin) = cos_sin(Theta::from_degrees(45.0));
        let abc = dq.to_abc(cos, sin);

        // verified against results from https://pypi.org/project/ClarkePark/
        assert_abs_diff_eq!(abc.a, 2.1213203435, epsilon = 0.0001);
        assert_abs_diff_eq!(abc.b, -0.448287736, epsilon = 0.0001);
        assert_abs_diff_eq!(abc.c, -1.673032607, epsilon = 0.0001);
    }

    #[test]
    fn dq_to_alpha_beta_0() {
        let dq = Dq {
            d: -0.7071067,
            q: 2.1213203435,
        };
        let (cos, sin) = cos_sin(Theta::from_degrees(45.0));
        let alpha_beta_0 = dq.to_alpha_beta_0(cos, sin);

        // verified against results from https://pypi.org/project/ClarkePark/
        assert_abs_diff_eq!(alpha_beta_0.alpha, 1.0, epsilon = 0.0001);
        assert_abs_diff_eq!(alpha_beta_0.beta, 2.0, epsilon = 0.0001);
        assert_abs_diff_eq!(alpha_beta_0.zero, 0.0, epsilon = 0.0001);
    }

    #[test]
    fn dq_to_dq0() {
        let dq = Dq { d: 1.0, q: 2.0 };
        let dq0 = Dq0::from(dq);
        assert_abs_diff_eq!(dq0.d, dq.d, epsilon = 0.0001);
        assert_abs_diff_eq!(dq0.q, dq.q, epsilon = 0.0001);
        assert_abs_diff_eq!(dq0.zero, 0.0, epsilon = 0.0001);
    }

    #[test]
    fn dq0_to_alpha_beta() {
        let dq0 = Dq0 {
            d: -0.7071067,
            q: 2.1213203435,
            zero: 3.0,
        };
        let (cos, sin) = cos_sin(Theta::from_degrees(45.0));
        let alpha_beta = dq0.to_alpha_beta(cos, sin);

        // verified against results from https://pypi.org/project/ClarkePark/
        assert_abs_diff_eq!(alpha_beta.alpha, 1.0, epsilon = 0.0001);
        assert_abs_diff_eq!(alpha_beta.beta, 2.0, epsilon = 0.0001);
    }

    #[test]
    fn dq0_to_dq() {
        let dq0 = Dq0 {
            d: 1.0,
            q: 2.0,
            zero: 3.0,
        };
        let dq = Dq::from(dq0);
        assert_abs_diff_eq!(dq.d, dq0.d, epsilon = 0.0001);
        assert_abs_diff_eq!(dq.q, dq0.q, epsilon = 0.0001);
    }
}
