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
use crate::trig::{neg_shift_120, pos_shift_120, Cos, Sin};
use crate::{Abc, AlphaBeta, AlphaBeta0, Dq, Dq0, Sequence};

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
    pub fn to_abc(&self, cos: Cos, sin: Sin, sequence: Sequence) -> Abc<T> {
        let (cos_b, sin_b, cos_c, sin_c): (Cos, Sin, Cos, Sin);
        match sequence {
            Sequence::POSITIVE => {
                (cos_b, sin_b) = neg_shift_120(cos, sin);
                (cos_c, sin_c) = pos_shift_120(cos, sin);
            }
            Sequence::NEGATIVE => {
                (cos_c, sin_c) = neg_shift_120(cos, sin);
                (cos_b, sin_b) = pos_shift_120(cos, sin);
            }
            Sequence::ZERO => {
                (cos_c, sin_c) = (cos, sin);
                (cos_b, sin_b) = (cos, sin);
            }
        }

        let a = (self.d * cos) - (self.q * sin);
        let b = (self.d * cos_b) - (self.q * sin_b);
        let c = (self.d * cos_c) - (self.q * sin_c);

        Abc { a, b, c }
    }

    pub fn to_alpha_beta(&self, cos: Cos, sin: Sin) -> AlphaBeta<T> {
        let alpha = (self.d * cos) - (self.q * sin);
        let beta = (self.q * cos) + (self.d * sin);

        AlphaBeta { alpha, beta }
    }

    pub fn to_alpha_beta_0(&self, cos: Cos, sin: Sin) -> AlphaBeta0<T> {
        let alpha_beta = self.to_alpha_beta(cos, sin);

        AlphaBeta0 {
            alpha: alpha_beta.alpha,
            beta: alpha_beta.beta,
            zero: 0.0.into(),
        }
    }
}

impl<T: Num> Dq0<T> {
    pub fn to_abc(&self, cos: Cos, sin: Sin, sequence: Sequence) -> Abc<T> {
        Dq {
            d: self.d,
            q: self.q,
        }
        .to_abc(cos, sin, sequence)
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

    use approx::assert_abs_diff_eq;

    #[test]
    fn dq_to_dq0() {
        let dq = Dq { d: 1.0, q: 2.0 };
        let dq0 = Dq0::from(dq);
        assert_abs_diff_eq!(dq0.d, dq.d, epsilon = 0.0001);
        assert_abs_diff_eq!(dq0.q, dq.q, epsilon = 0.0001);
        assert_abs_diff_eq!(dq0.zero, 0.0, epsilon = 0.0001);
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
