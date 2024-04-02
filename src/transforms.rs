use crate::constants::{ONE_HALF, ONE_THIRD, SQRT_3_OVER_2, SQRT_3_OVER_3, TWO_THIRDS};
use crate::reference_frames::{Abc, AlphaBeta, AlphaBeta0, Dq, Dq0, Polar};
use crate::trig::{shift_left_120, shift_right_120, Cos, Sin};
use core::ops::{Add, Mul, Neg, Sub};

impl<T: Mul<f32, Output = T> + Mul<Sin, Output = T> + Copy + From<f32>> From<Polar<T>> for Abc<T> {
    fn from(polar: Polar<T>) -> Self {
        Self::from_polar(polar.amplitude, polar.theta)
    }
}

impl<T: Mul<f32, Output = T> + Sub<Output = T> + Copy> From<Abc<T>> for AlphaBeta<T> {
    fn from(abc: Abc<T>) -> Self {
        let alpha = (abc.a * TWO_THIRDS) - (abc.b * ONE_THIRD) - (abc.c * ONE_THIRD);
        let beta = (abc.b * SQRT_3_OVER_3) - (abc.c * SQRT_3_OVER_3);

        Self { alpha, beta }
    }
}

impl<T: Mul<f32, Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy>
    From<AlphaBeta<T>> for Abc<T>
{
    fn from(alpha_beta: AlphaBeta<T>) -> Self {
        let a = alpha_beta.alpha;
        let b = -(alpha_beta.alpha * ONE_HALF) + alpha_beta.beta * SQRT_3_OVER_2;
        let c = -alpha_beta.alpha * ONE_HALF - alpha_beta.beta * SQRT_3_OVER_2;
        Self { a, b, c }
    }
}

impl<T: Mul<f32, Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy>
    From<Abc<T>> for AlphaBeta0<T>
{
    fn from(abc: Abc<T>) -> Self {
        let alpha = (abc.a * TWO_THIRDS) - (abc.b * ONE_THIRD) - (abc.c * ONE_THIRD);
        let beta = (abc.b * SQRT_3_OVER_3) - (abc.c * SQRT_3_OVER_3);
        let zero = (abc.a + abc.b + abc.c) * ONE_THIRD;

        Self { alpha, beta, zero }
    }
}

impl<T: From<f32>> From<AlphaBeta<T>> for AlphaBeta0<T> {
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

impl<T: Mul<f32, Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy>
    From<AlphaBeta0<T>> for Abc<T>
{
    fn from(alpha_beta_0: AlphaBeta0<T>) -> Self {
        let a = alpha_beta_0.alpha + alpha_beta_0.zero;
        let b =
            -alpha_beta_0.alpha * ONE_HALF + alpha_beta_0.beta * SQRT_3_OVER_2 + alpha_beta_0.zero;
        let c =
            -alpha_beta_0.alpha * ONE_HALF - alpha_beta_0.beta * SQRT_3_OVER_2 + alpha_beta_0.zero;
        Self { a, b, c }
    }
}

impl<
        T: Mul<Sin, Output = T>
            + Mul<Cos, Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Neg<Output = T>
            + From<f32>
            + Copy,
    > AlphaBeta<T>
{
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

impl<
        T: Mul<Sin, Output = T>
            + Mul<Cos, Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Neg<Output = T>
            + From<f32>
            + Copy,
    > AlphaBeta0<T>
{
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

impl<
        T: Mul<Sin, Output = T>
            + Mul<Cos, Output = T>
            + Mul<f32, Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Neg<Output = T>
            + From<f32>
            + Copy,
    > Abc<T>
{
    pub fn to_dq(&self, cos: Cos, sin: Sin) -> Dq<T> {
        /* sin and cos with 120 degree offsets */
        let (cos_m, sin_m) = shift_left_120(cos, sin);
        let (cos_p, sin_p) = shift_right_120(cos, sin);

        let d = ((self.a * sin) + (self.b * sin_m) + (self.b * sin_p)) * TWO_THIRDS;
        let q = ((self.a * cos) + (self.b * cos_m) + (self.b * cos_p)) * TWO_THIRDS;

        Dq { d, q }
    }

    pub fn to_dq0(&self, cos: Cos, sin: Sin) -> Dq0<T> {
        /* sin and cos with 120 degree offsets */
        let (cos_m, sin_m) = shift_left_120(cos, sin);
        let (cos_p, sin_p) = shift_right_120(cos, sin);

        let d = ((self.a * sin) + (self.b * sin_m) + (self.b * sin_p)) * TWO_THIRDS;
        let q = ((self.a * cos) + (self.b * cos_m) + (self.b * cos_p)) * TWO_THIRDS;
        let zero = (self.a + self.b + self.c) * ONE_THIRD;
        Dq0 { d, q, zero }
    }
}

impl<T: Add<Output = T> + Mul<f32, Output = T>> From<Abc<T>> for f32
where
    f32: From<T>,
{
    fn from(abc: Abc<T>) -> Self {
        ((abc.a + abc.b + abc.c) * ONE_THIRD).into()
    }
}

impl<T: From<f32>> From<Dq<T>> for Dq0<T> {
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

impl<
        T: Mul<Sin, Output = T>
            + Mul<Cos, Output = T>
            + Mul<f32, Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Neg<Output = T>
            + From<f32>
            + Copy,
    > Dq<T>
{
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

impl<
        T: Mul<Sin, Output = T>
            + Mul<Cos, Output = T>
            + Mul<f32, Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Neg<Output = T>
            + From<f32>
            + Copy,
    > Dq0<T>
{
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

    #[test]
    fn abc_to_alpha_beta_0() {
        let abc = Abc::<f32>::zero();
        let _alpha_beta_0 = AlphaBeta0::from(abc);
    }

    #[test]
    fn abc_to_dq0() {
        let abc = Abc::<f32>::zero();
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _dq0 = abc.to_dq0(cos, sin);
    }

    #[test]
    fn alpha_beta_0_to_dq0() {
        let alpha_beta_0 = AlphaBeta0::<f32>::zero();
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _dq0 = alpha_beta_0.to_dq0(cos, sin);
    }

    #[test]
    fn dq0_to_abc() {
        let dq0 = Dq0::<f32>::zero();
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _abc = dq0.to_abc(cos, sin);
    }

    #[test]
    fn dq0_to_alpha_beta_0() {
        let dq0 = Dq0::<f32>::zero();
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _abc = dq0.to_alpha_beta_0(cos, sin);
    }

    #[test]
    fn alpha_beta_to_abc() {
        let alpha_beta = AlphaBeta::<f32>::zero();
        let _abc = Abc::from(alpha_beta);
    }

    #[test]
    fn alpha_beta_to_alpha_beta_0() {
        let alpha_beta = AlphaBeta::<f32>::zero();
        let _alpha_beta0 = AlphaBeta0::<f32>::from(alpha_beta);
    }

    #[test]
    fn alpha_beta_to_dq0() {
        let alpha_beta = AlphaBeta::<f32>::zero();
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _dq0 = alpha_beta.to_dq0(cos, sin);
    }

    #[test]
    fn dq_to_abc() {
        let dq = Dq::<f32>::zero();
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _abc = dq.to_abc(cos, sin);
    }

    #[test]
    fn dq_to_alpha_beta_0() {
        let dq = Dq::<f32>::zero();
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _alpha_beta_0 = dq.to_alpha_beta_0(cos, sin);
    }

    #[test]
    fn dq_to_dq0() {
        let dq = Dq::<f32>::zero();
        let _dq0 = Dq0::from(dq);
    }

    #[test]
    fn abc_to_alpha_beta() {
        let abc = Abc {
            a: 0.0,
            b: 0.0,
            c: 0.0,
        };
        let _alpha_beta = AlphaBeta::from(abc);
    }

    #[test]
    fn abc_to_dq() {
        let abc = Abc::<f32>::zero();
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _dq = abc.to_dq(cos, sin);
    }

    #[test]
    fn alpha_beta_0_to_alpha_beta() {
        let alpha_beta_0 = AlphaBeta0::<f32>::zero();
        let _alpha_beta = AlphaBeta::from(alpha_beta_0);
    }

    #[test]
    fn alpha_beta_0_to_dq() {
        let alpha_beta_0 = AlphaBeta0::<f32>::zero();
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _dq = alpha_beta_0.to_dq(cos, sin);
    }

    #[test]
    fn dq0_to_alpha_beta() {
        let dq0 = Dq0::<f32>::zero();
        let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
        let _alpha_beta = dq0.to_alpha_beta(cos, sin);
    }

    #[test]
    fn dq0_to_dq() {
        let dq0 = Dq0::<f32>::zero();
        let _dq = Dq::from(dq0);
    }
}
