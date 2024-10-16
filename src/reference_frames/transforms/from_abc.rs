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

use crate::constants::{ONE_THIRD, SQRT_3_OVER_3, TWO_THIRDS};
use crate::number::Num;
use crate::reference_frames::{Abc, AlphaBeta, AlphaBeta0, Dq, Dq0};
use crate::trig::{shift_left_120, shift_right_120, Cos, Sin};

impl<T: Num> From<Abc<T>> for AlphaBeta<T> {
    fn from(abc: Abc<T>) -> Self {
        let alpha = (abc.a * TWO_THIRDS) - (abc.b * ONE_THIRD) - (abc.c * ONE_THIRD);
        let beta = (abc.b * SQRT_3_OVER_3) - (abc.c * SQRT_3_OVER_3);

        Self { alpha, beta }
    }
}

impl<T: Num> From<Abc<T>> for AlphaBeta0<T> {
    fn from(abc: Abc<T>) -> Self {
        let alpha = (abc.a * TWO_THIRDS) - (abc.b * ONE_THIRD) - (abc.c * ONE_THIRD);
        let beta = (abc.b * SQRT_3_OVER_3) - (abc.c * SQRT_3_OVER_3);
        let zero = (abc.a + abc.b + abc.c) * ONE_THIRD;

        Self { alpha, beta, zero }
    }
}

impl<T: Num> Abc<T> {
    pub fn to_dq(&self, cos: Cos, sin: Sin) -> Dq<T> {
        /* sin and cos with 120 degree offsets */
        let (cos_m, sin_m) = shift_left_120(cos, sin);
        let (cos_p, sin_p) = shift_right_120(cos, sin);

        let d = ((self.a * sin) + (self.b * sin_m) + (self.c * sin_p)) * TWO_THIRDS;
        let q = ((self.a * cos) + (self.b * cos_m) + (self.c * cos_p)) * TWO_THIRDS;

        Dq { d, q }
    }

    pub fn to_dq0(&self, cos: Cos, sin: Sin) -> Dq0<T> {
        /* sin and cos with 120 degree offsets */
        let (cos_m, sin_m) = shift_left_120(cos, sin);
        let (cos_p, sin_p) = shift_right_120(cos, sin);

        let d = ((self.a * sin) + (self.b * sin_m) + (self.c * sin_p)) * TWO_THIRDS;
        let q = ((self.a * cos) + (self.b * cos_m) + (self.c * cos_p)) * TWO_THIRDS;
        let zero = (self.a + self.b + self.c) * ONE_THIRD;
        Dq0 { d, q, zero }
    }
}

impl<T: Num> From<Abc<T>> for f32
where
    f32: From<T>,
{
    fn from(abc: Abc<T>) -> Self {
        ((abc.a + abc.b + abc.c) * ONE_THIRD).into()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::trig::cos_sin;
    use crate::trig::Theta;
    use approx::assert_abs_diff_eq;

    #[test]
    fn abc_to_alpha_beta_0() {
        let abc = Abc {
            a: 1.0,
            b: 2.0,
            c: 3.0,
        };
        let alpha_beta_0 = AlphaBeta0::from(abc);

        // verified against results from https://pypi.org/project/ClarkePark/
        assert_abs_diff_eq!(alpha_beta_0.alpha, -1.0, epsilon = 0.0001);
        assert_abs_diff_eq!(alpha_beta_0.beta, -0.577350, epsilon = 0.0001);
        assert_abs_diff_eq!(alpha_beta_0.zero, 2.0, epsilon = 0.0001);
    }

    #[test]
    fn abc_to_dq0() {
        let abc = Abc {
            a: 1.0,
            b: 2.0,
            c: 3.0,
        };

        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let dq0 = abc.to_dq0(cos, sin);

        // verified against results from https://pypi.org/project/ClarkePark/
        assert_abs_diff_eq!(dq0.d, -1.0, epsilon = 0.0001);
        assert_abs_diff_eq!(dq0.q, -0.577350, epsilon = 0.0001);
        assert_abs_diff_eq!(dq0.zero, 2.0, epsilon = 0.0001);
    }

    #[test]
    fn abc_to_alpha_beta() {
        let abc = Abc {
            a: 1.0,
            b: 2.0,
            c: 3.0,
        };
        let alpha_beta = AlphaBeta::from(abc);

        // verified against results from https://pypi.org/project/ClarkePark/
        assert_abs_diff_eq!(alpha_beta.alpha, -1.0, epsilon = 0.0001);
        assert_abs_diff_eq!(alpha_beta.beta, -0.577350, epsilon = 0.0001);
    }

    #[test]
    fn abc_to_dq() {
        let abc = Abc {
            a: 1.0,
            b: 2.0,
            c: 3.0,
        };

        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let dq = abc.to_dq(cos, sin);

        // verified against results from https://pypi.org/project/ClarkePark/
        assert_abs_diff_eq!(dq.d, -1.0, epsilon = 0.0001);
        assert_abs_diff_eq!(dq.q, -0.577350, epsilon = 0.0001);
    }
}
