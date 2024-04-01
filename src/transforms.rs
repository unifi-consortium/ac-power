use crate::constants::{ONE_HALF, ONE_THIRD, SQRT_3_OVER_2, SQRT_3_OVER_3, TWO_THIRDS};
use crate::reference_frames::{Abc, AlphaBeta, AlphaBeta0, Dq, Dq0, Polar};
use crate::trig::{shift_left_120, shift_right_120, Cos, Sin};

impl From<Polar> for Abc {
    fn from(polar: Polar) -> Self {
        Self::from_polar(polar.amplitude, polar.theta)
    }
}

impl From<Abc> for AlphaBeta {
    fn from(abc: Abc) -> Self {
        let alpha = (abc.a * TWO_THIRDS) - (abc.b * ONE_THIRD) - (abc.c * ONE_THIRD);
        let beta = (abc.b * SQRT_3_OVER_3) - (abc.c * SQRT_3_OVER_3);

        Self { alpha, beta }
    }
}

impl From<AlphaBeta> for Abc {
    fn from(alpha_beta: AlphaBeta) -> Self {
        let a = alpha_beta.alpha;
        let b = -ONE_HALF * alpha_beta.alpha + SQRT_3_OVER_2 * alpha_beta.beta;
        let c = -ONE_HALF * alpha_beta.alpha - SQRT_3_OVER_2 * alpha_beta.beta;
        Self { a, b, c }
    }
}

impl From<Abc> for AlphaBeta0 {
    fn from(abc: Abc) -> Self {
        let alpha = (abc.a * TWO_THIRDS) - (abc.b * ONE_THIRD) - (abc.c * ONE_THIRD);
        let beta = (abc.b * SQRT_3_OVER_3) - (abc.c * SQRT_3_OVER_3);
        let zero = ONE_THIRD * (abc.a + abc.b + abc.c);

        Self { alpha, beta, zero }
    }
}

impl From<AlphaBeta> for AlphaBeta0 {
    fn from(alpha_beta: AlphaBeta) -> Self {
        Self {
            alpha: alpha_beta.alpha,
            beta: alpha_beta.beta,
            zero: 0.0,
        }
    }
}

impl From<AlphaBeta0> for AlphaBeta {
    fn from(alpha_beta_0: AlphaBeta0) -> Self {
        Self {
            alpha: alpha_beta_0.alpha,
            beta: alpha_beta_0.beta,
        }
    }
}

impl From<AlphaBeta0> for Abc {
    fn from(alpha_beta_0: AlphaBeta0) -> Self {
        let a = alpha_beta_0.alpha + alpha_beta_0.zero;
        let b =
            -ONE_HALF * alpha_beta_0.alpha + SQRT_3_OVER_2 * alpha_beta_0.beta + alpha_beta_0.zero;
        let c =
            -ONE_HALF * alpha_beta_0.alpha - SQRT_3_OVER_2 * alpha_beta_0.beta + alpha_beta_0.zero;
        Self { a, b, c }
    }
}

impl AlphaBeta {
    pub fn to_dq0(&self, cos: Cos, sin: Sin) -> Dq0 {
        let d = (self.alpha * sin) - (self.beta * cos);
        let q = (self.alpha * cos) + (self.beta * sin);

        Dq0 { d, q, zero: 0.0 }
    }

    pub fn to_dq(&self, cos: Cos, sin: Sin) -> Dq {
        let d = (self.alpha * sin) - (self.beta * cos);
        let q = (self.alpha * cos) + (self.beta * sin);

        Dq { d, q }
    }
}

impl AlphaBeta0 {
    pub fn to_dq0(&self, cos: Cos, sin: Sin) -> Dq0 {
        let d = (self.alpha * sin) - (self.beta * cos);
        let q = (self.alpha * cos) + (self.beta * sin);

        Dq0 {
            d,
            q,
            zero: self.zero,
        }
    }

    pub fn to_dq(&self, cos: Cos, sin: Sin) -> Dq {
        let d = (self.alpha * sin) - (self.beta * cos);
        let q = (self.alpha * cos) + (self.beta * sin);

        Dq { d, q }
    }
}

impl Abc {
    pub fn to_dq(&self, cos: Cos, sin: Sin) -> Dq {
        /* sin and cos with 120 degree offsets */
        let (cos_m, sin_m) = shift_left_120(cos, sin);
        let (cos_p, sin_p) = shift_right_120(cos, sin);

        let d = TWO_THIRDS * ((self.a * sin) + (self.b * sin_m) + (self.b * sin_p));
        let q = TWO_THIRDS * ((self.a * cos) + (self.b * cos_m) + (self.b * cos_p));

        Dq { d, q }
    }

    pub fn to_dq0(&self, cos: Cos, sin: Sin) -> Dq0 {
        /* sin and cos with 120 degree offsets */
        let (cos_m, sin_m) = shift_left_120(cos, sin);
        let (cos_p, sin_p) = shift_right_120(cos, sin);

        let d = TWO_THIRDS * ((self.a * sin) + (self.b * sin_m) + (self.b * sin_p));
        let q = TWO_THIRDS * ((self.a * cos) + (self.b * cos_m) + (self.b * cos_p));

        Dq0 {
            d,
            q,
            zero: (*self).into(),
        }
    }
}

impl From<Abc> for f32 {
    fn from(abc: Abc) -> Self {
        ONE_THIRD * (abc.a + abc.b + abc.c)
    }
}

impl From<Dq> for Dq0 {
    fn from(dq: Dq) -> Self {
        Self {
            d: dq.d,
            q: dq.q,
            zero: 0.0,
        }
    }
}

impl From<Dq0> for Dq {
    fn from(dq0: Dq0) -> Self {
        Self { d: dq0.d, q: dq0.q }
    }
}

impl Dq {
    pub fn to_abc(&self, cos: Cos, sin: Sin) -> Abc {
        /* sin and cos with 120 degree offsets */
        let (cos_m, sin_m) = shift_left_120(cos, sin);
        let (cos_p, sin_p) = shift_right_120(cos, sin);

        let a = (self.d * sin) + (self.q * cos);
        let b = (self.d * sin_m) + (self.q * cos_m);
        let c = (self.d * sin_p) + (self.q * cos_p);

        Abc { a, b, c }
    }

    pub fn to_alpha_beta(&self, cos: Cos, sin: Sin) -> AlphaBeta {
        let alpha = (self.d * sin) + (self.q * cos);
        let beta = (self.q * sin) - (self.d * cos);

        AlphaBeta { alpha, beta }
    }

    pub fn to_alpha_beta_0(&self, cos: Cos, sin: Sin) -> AlphaBeta0 {
        let alpha = (self.d * sin) + (self.q * cos);
        let beta = (self.q * sin) - (self.d * cos);

        AlphaBeta0 {
            alpha,
            beta,
            zero: 0.0,
        }
    }
}

impl Dq0 {
    pub fn to_abc(&self, cos: Cos, sin: Sin) -> Abc {
        Dq {
            d: self.d,
            q: self.q,
        }
        .to_abc(cos, sin)
            + self.zero
    }

    pub fn to_alpha_beta(&self, cos: Cos, sin: Sin) -> AlphaBeta {
        let alpha = (self.d * sin) + (self.q * cos);
        let beta = (self.q * sin) - (self.d * cos);

        AlphaBeta { alpha, beta }
    }

    pub fn to_alpha_beta_0(&self, cos: Cos, sin: Sin) -> AlphaBeta0 {
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

    #[test]
    fn abc_to_alpha_beta_0() {
        let abc = Abc::ZERO;
        let _alpha_beta_0 = AlphaBeta0::from(abc);
    }

    #[test]
    fn abc_to_dq0() {
        let abc = Abc::ZERO;
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _dq0 = abc.to_dq0(cos, sin);
    }

    #[test]
    fn alpha_beta_0_to_dq0() {
        let alpha_beta_0 = AlphaBeta0::ZERO;
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _dq0 = alpha_beta_0.to_dq0(cos, sin);
    }

    #[test]
    fn dq0_to_abc() {
        let dq0 = Dq0::ZERO;
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _abc = dq0.to_abc(cos, sin);
    }

    #[test]
    fn dq0_to_alpha_beta_0() {
        let dq0 = Dq0::ZERO;
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _abc = dq0.to_alpha_beta_0(cos, sin);
    }

    #[test]
    fn alpha_beta_to_abc() {
        let alpha_beta = AlphaBeta::ZERO;
        let _abc = Abc::from(alpha_beta);
    }

    #[test]
    fn alpha_beta_to_alpha_beta_0() {
        let alpha_beta = AlphaBeta::ZERO;
        let _alpha_beta0 = AlphaBeta0::from(alpha_beta);
    }

    #[test]
    fn alpha_beta_to_dq0() {
        let alpha_beta = AlphaBeta::ZERO;
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _dq0 = alpha_beta.to_dq0(cos, sin);
    }

    #[test]
    fn dq_to_abc() {
        let dq = Dq::ZERO;
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _abc = dq.to_abc(cos, sin);
    }

    #[test]
    fn dq_to_alpha_beta_0() {
        let dq = Dq::ZERO;
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _alpha_beta_0 = dq.to_alpha_beta_0(cos, sin);
    }

    #[test]
    fn dq_to_dq0() {
        let dq = Dq::ZERO;
        let _dq0 = Dq0::from(dq);
    }

    #[test]
    fn abc_to_alpha_beta() {
        let abc = Abc::ZERO;
        let _alpha_beta = AlphaBeta::from(abc);
    }

    #[test]
    fn abc_to_dq() {
        let abc = Abc::ZERO;
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _dq = abc.to_dq(cos, sin);
    }

    #[test]
    fn alpha_beta_0_to_alpha_beta() {
        let alpha_beta_0 = AlphaBeta0::ZERO;
        let _alpha_beta = AlphaBeta::from(alpha_beta_0);
    }

    #[test]
    fn alpha_beta_0_to_dq() {
        let alpha_beta_0 = AlphaBeta0::ZERO;
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _dq = alpha_beta_0.to_dq(cos, sin);
    }

    #[test]
    fn dq0_to_alpha_beta() {
        let dq0 = Dq0::ZERO;
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _alpha_beta = dq0.to_alpha_beta(cos, sin);
    }

    #[test]
    fn dq0_to_dq() {
        let dq0 = Dq0::ZERO;
        let _dq = Dq::from(dq0);
    }
}
