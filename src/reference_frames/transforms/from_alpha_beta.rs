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

use crate::constants::{ONE_HALF, SQRT_3_OVER_2};
use crate::number::Num;
use crate::reference_frames::{Abc, AlphaBeta, AlphaBeta0, Dq, Dq0};
use crate::trig::{Cos, Sin};

impl<T: Num> From<AlphaBeta<T>> for Abc<T> {
    fn from(alpha_beta: AlphaBeta<T>) -> Self {
        let a = alpha_beta.alpha;
        let b = -(alpha_beta.alpha * ONE_HALF) + alpha_beta.beta * SQRT_3_OVER_2;
        let c = -alpha_beta.alpha * ONE_HALF - alpha_beta.beta * SQRT_3_OVER_2;
        Self { a, b, c }
    }
}

impl<T: Num> From<AlphaBeta<T>> for AlphaBeta0<T> {
    fn from(alpha_beta: AlphaBeta<T>) -> Self {
        Self {
            alpha: alpha_beta.alpha,
            beta: alpha_beta.beta,
            zero: 0.0.into(),
        }
    }
}

impl<T> From<AlphaBeta0<T>> for AlphaBeta<T> {
    fn from(alpha_beta_0: AlphaBeta0<T>) -> Self {
        Self {
            alpha: alpha_beta_0.alpha,
            beta: alpha_beta_0.beta,
        }
    }
}

impl<T: Num> From<AlphaBeta0<T>> for Abc<T> {
    fn from(alpha_beta_0: AlphaBeta0<T>) -> Self {
        let a = alpha_beta_0.alpha + alpha_beta_0.zero;
        let b =
            -alpha_beta_0.alpha * ONE_HALF + alpha_beta_0.beta * SQRT_3_OVER_2 + alpha_beta_0.zero;
        let c =
            -alpha_beta_0.alpha * ONE_HALF - alpha_beta_0.beta * SQRT_3_OVER_2 + alpha_beta_0.zero;
        Self { a, b, c }
    }
}

impl<T: Num> AlphaBeta<T> {
    /// Transform from AlphaBeta to Dq0 reference frame using Clarke transfom
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use ac_power::{Abc, AlphaBeta};
    /// use ac_power::trig::{Theta, cos_sin};
    ///
    /// let theta = Theta::from_degrees(45.0);
    /// let (cos, sin) = cos_sin(theta);
    /// let alpha_beta = AlphaBeta::from(Abc::from_polar(100.0, theta));
    /// let dq0 = alpha_beta.to_dq0(cos, sin);
    /// ```
    pub fn to_dq0(&self, cos: Cos, sin: Sin) -> Dq0<T> {
        let d = (self.alpha * sin) - (self.beta * cos);
        let q = (self.alpha * cos) + (self.beta * sin);

        Dq0 {
            d,
            q,
            zero: 0.0.into(),
        }
    }

    /// Transform from AlphaBeta to Dq reference frame using Clarke transfom
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use ac_power::{Abc, AlphaBeta};
    /// use ac_power::trig::{Theta, cos_sin};
    ///
    /// let theta = Theta::from_degrees(45.0);
    /// let (cos, sin) = cos_sin(theta);
    /// let alpha_beta = AlphaBeta::from(Abc::from_polar(100.0, theta));
    /// let dq = alpha_beta.to_dq(cos, sin);
    /// ```
    pub fn to_dq(&self, cos: Cos, sin: Sin) -> Dq<T> {
        let d = (self.alpha * sin) - (self.beta * cos);
        let q = (self.alpha * cos) + (self.beta * sin);

        Dq { d, q }
    }
}

impl<T: Num> AlphaBeta0<T> {
    //// Transform from AlphaBeta0 to Dq0 reference frame using Clarke transfom
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use ac_power::{Abc, AlphaBeta0};
    /// use ac_power::trig::{Theta, cos_sin};
    ///
    /// let theta = Theta::from_degrees(45.0);
    /// let (cos, sin) = cos_sin(theta);
    /// let alpha_beta = AlphaBeta0::from(Abc::from_polar(100.0, theta));
    /// let dq0 = alpha_beta.to_dq0(cos, sin);
    /// ```
    pub fn to_dq0(&self, cos: Cos, sin: Sin) -> Dq0<T> {
        let d = (self.alpha * sin) - (self.beta * cos);
        let q = (self.alpha * cos) + (self.beta * sin);

        Dq0 {
            d,
            q,
            zero: self.zero,
        }
    }

    //// Transform from AlphaBeta0 to Dq reference frame using Clarke transfom
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use ac_power::{Abc, AlphaBeta0};
    /// use ac_power::trig::{Theta, cos_sin};
    ///
    /// let theta = Theta::from_degrees(45.0);
    /// let (cos, sin) = cos_sin(theta);
    /// let alpha_beta = AlphaBeta0::from(Abc::from_polar(100.0, theta));
    /// let dq = alpha_beta.to_dq(cos, sin);
    /// ```
    pub fn to_dq(&self, cos: Cos, sin: Sin) -> Dq<T> {
        let d = (self.alpha * sin) - (self.beta * cos);
        let q = (self.alpha * cos) + (self.beta * sin);

        Dq { d, q }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::trig::cos_sin;
    use crate::trig::Theta;
    use approx::assert_abs_diff_eq;

    #[test]
    fn alpha_beta_0_to_dq0() {
        let alpha_beta_0 = AlphaBeta0 {
            alpha: 1.0,
            beta: 2.0,
            zero: 3.0,
        };
        let (cos, sin) = cos_sin(Theta::from_degrees(45.0));
        let dq0 = alpha_beta_0.to_dq0(cos, sin);

        // verified against results from https://pypi.org/project/ClarkePark/
        assert_abs_diff_eq!(dq0.d, -0.7071067, epsilon = 0.0001);
        assert_abs_diff_eq!(dq0.q, 2.1213203435, epsilon = 0.0001);
        assert_abs_diff_eq!(dq0.zero, 3.0, epsilon = 0.0001);
    }

    #[test]
    fn alpha_beta_to_abc() {
        let alpha_beta = AlphaBeta {
            alpha: 1.0,
            beta: 2.0,
        };
        let abc = Abc::from(alpha_beta);

        // verified against results from https://pypi.org/project/ClarkePark/
        assert_abs_diff_eq!(abc.a, 1.0, epsilon = 0.0001);
        assert_abs_diff_eq!(abc.b, 1.2320508, epsilon = 0.0001);
        assert_abs_diff_eq!(abc.c, -2.232050, epsilon = 0.0001);
    }

    #[test]
    fn alpha_beta_to_alpha_beta_0() {
        let alpha_beta = AlphaBeta {
            alpha: 1.0,
            beta: 2.0,
        };
        let alpha_beta0 = AlphaBeta0::<f32>::from(alpha_beta);

        assert_abs_diff_eq!(alpha_beta0.alpha, alpha_beta.alpha, epsilon = 0.0001);
        assert_abs_diff_eq!(alpha_beta0.beta, alpha_beta.beta, epsilon = 0.0001);
        assert_abs_diff_eq!(alpha_beta0.zero, 0.0, epsilon = 0.0001);
    }

    #[test]
    fn alpha_beta_to_dq0() {
        let alpha_beta = AlphaBeta {
            alpha: 1.0,
            beta: 2.0,
        };
        let (cos, sin) = cos_sin(Theta::from_degrees(45.0));
        let dq0 = alpha_beta.to_dq0(cos, sin);

        // verified against results from https://pypi.org/project/ClarkePark/
        assert_abs_diff_eq!(dq0.d, -0.7071067, epsilon = 0.0001);
        assert_abs_diff_eq!(dq0.q, 2.1213203435, epsilon = 0.0001);
        assert_abs_diff_eq!(dq0.zero, 0.0, epsilon = 0.0001);
    }

    #[test]
    fn alpha_beta_0_to_alpha_beta() {
        let alpha_beta_0 = AlphaBeta0 {
            alpha: 1.0,
            beta: 2.0,
            zero: 3.0,
        };
        let alpha_beta = AlphaBeta::from(alpha_beta_0);

        assert_abs_diff_eq!(alpha_beta.alpha, alpha_beta_0.alpha, epsilon = 0.0001);
        assert_abs_diff_eq!(alpha_beta.beta, alpha_beta_0.beta, epsilon = 0.0001);
    }

    #[test]
    fn alpha_beta_0_to_dq() {
        let alpha_beta_0 = AlphaBeta0 {
            alpha: 1.0,
            beta: 2.0,
            zero: 3.0,
        };
        let (cos, sin) = cos_sin(Theta::from_degrees(45.0));
        let dq = alpha_beta_0.to_dq(cos, sin);

        // verified against results from https://pypi.org/project/ClarkePark/
        assert_abs_diff_eq!(dq.d, -0.7071067, epsilon = 0.0001);
        assert_abs_diff_eq!(dq.q, 2.1213203435, epsilon = 0.0001);
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
