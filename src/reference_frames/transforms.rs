use crate::constants::{ONE_HALF, ONE_THIRD, SQRT_3_OVER_2, SQRT_3_OVER_3, TWO_THIRDS};
use crate::number::Num;
use crate::reference_frames::{Abc, AlphaBeta, AlphaBeta0, Dq, Dq0, Polar};
use crate::trig::{shift_left_120, shift_right_120, Cos, Sin};

impl<T: Num> From<Polar<T>> for Abc<T> {
    fn from(polar: Polar<T>) -> Self {
        Self::from_polar(polar.amplitude, polar.theta)
    }
}

impl<T: Num> From<Abc<T>> for AlphaBeta<T> {
    fn from(abc: Abc<T>) -> Self {
        let alpha = (abc.a * TWO_THIRDS) - (abc.b * ONE_THIRD) - (abc.c * ONE_THIRD);
        let beta = (abc.b * SQRT_3_OVER_3) - (abc.c * SQRT_3_OVER_3);

        Self { alpha, beta }
    }
}

impl<T: Num> From<AlphaBeta<T>> for Abc<T> {
    fn from(alpha_beta: AlphaBeta<T>) -> Self {
        let a = alpha_beta.alpha;
        let b = -(alpha_beta.alpha * ONE_HALF) + alpha_beta.beta * SQRT_3_OVER_2;
        let c = -alpha_beta.alpha * ONE_HALF - alpha_beta.beta * SQRT_3_OVER_2;
        Self { a, b, c }
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
    pub fn to_dq0(&self, cos: Cos, sin: Sin) -> Dq0<T> {
        let d = (self.alpha * sin) - (self.beta * cos);
        let q = (self.alpha * cos) + (self.beta * sin);

        Dq0 {
            d,
            q,
            zero: 0.0.into(),
        }
    }

    pub fn to_dq(&self, cos: Cos, sin: Sin) -> Dq<T> {
        let d = (self.alpha * sin) - (self.beta * cos);
        let q = (self.alpha * cos) + (self.beta * sin);

        Dq { d, q }
    }
}

impl<T: Num> AlphaBeta0<T> {
    pub fn to_dq0(&self, cos: Cos, sin: Sin) -> Dq0<T> {
        let d = (self.alpha * sin) - (self.beta * cos);
        let q = (self.alpha * cos) + (self.beta * sin);

        Dq0 {
            d,
            q,
            zero: self.zero,
        }
    }

    pub fn to_dq(&self, cos: Cos, sin: Sin) -> Dq<T> {
        let d = (self.alpha * sin) - (self.beta * cos);
        let q = (self.alpha * cos) + (self.beta * sin);

        Dq { d, q }
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
