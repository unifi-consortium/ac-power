use crate::constants::{ONE_THIRD, SQRT_3_OVER_3, TWO_THIRDS};
use crate::reference_frames::{Abc, AlphaBeta, AlphaBeta0, Dq, Dq0, Polar};
use crate::trig::{cos_sin, shift_left_120, shift_right_120};
use core::convert::From;
use fixed::types::I1F31;
use fixed::FixedI32;

impl<const FRAC: i32> From<Polar<FRAC>> for Abc<FRAC> {
    fn from(polar: Polar<FRAC>) -> Self {
        let (cos, sin) = cos_sin(polar.theta);
        let (_, sin_m) = shift_left_120(cos, sin);
        let (_, sin_p) = shift_right_120(cos, sin);

        let mut a = polar.amplitude;
        let mut b = polar.amplitude;
        let mut c = polar.amplitude;
        a *= sin;
        b *= sin_m;
        c *= sin_p;
        Self { a, b, c }
    }
}

impl<const FRAC: i32> From<Abc<FRAC>> for AlphaBeta<FRAC> {
    fn from(abc: Abc<FRAC>) -> Self {
        let mut alpha = abc.a;
        alpha *= TWO_THIRDS;
        alpha.mul_acc(abc.b, -ONE_THIRD);
        alpha.mul_acc(abc.c, -ONE_THIRD);

        let mut beta = abc.b;
        beta *= SQRT_3_OVER_3;
        beta.mul_acc(abc.c, -SQRT_3_OVER_3);

        Self { alpha, beta }
    }
}

impl<const FRAC: i32> From<Abc<FRAC>> for AlphaBeta0<FRAC> {
    fn from(abc: Abc<FRAC>) -> Self {
        let mut alpha = abc.a;
        alpha *= TWO_THIRDS;
        alpha.mul_acc(abc.b, -ONE_THIRD);
        alpha.mul_acc(abc.c, -ONE_THIRD);

        let mut beta = abc.b;
        beta *= SQRT_3_OVER_3;
        beta.mul_acc(abc.c, -SQRT_3_OVER_3);

        let mut zero = abc.a;
        zero *= ONE_THIRD;
        zero.mul_acc(abc.b, ONE_THIRD);
        zero.mul_acc(abc.c, ONE_THIRD);

        Self { alpha, beta, zero }
    }
}

impl<const FRAC: i32> AlphaBeta<FRAC> {
    // DQ0 Transform
    pub fn to_dq0(&self, cos: I1F31, sin: I1F31) -> Dq0<FRAC> {
        let mut d = self.alpha;
        d *= sin;
        d.mul_acc(-self.beta, cos);

        let mut q = self.alpha;
        q *= cos;
        q.mul_acc(self.beta, sin);

        Dq0 {
            d,
            q,
            zero: FixedI32::<FRAC>::ZERO,
        }
    }

    pub fn to_dq(&self, cos: I1F31, sin: I1F31) -> Dq<FRAC> {
        let mut d = self.alpha;
        d *= sin;
        d.mul_acc(-self.beta, cos);

        let mut q = self.alpha;
        q *= cos;
        q.mul_acc(self.beta, sin);

        Dq { d, q }
    }
}

impl<const FRAC: i32> Abc<FRAC> {
    pub fn to_dq(&self, cos: I1F31, sin: I1F31) -> Dq<FRAC> {
        /* sin and cos with 120 degree offsets */
        let (cos_m, sin_m) = shift_left_120(cos, sin);
        let (cos_p, sin_p) = shift_right_120(cos, sin);

        let mut d = self.a;
        d *= sin;
        d.mul_acc(self.b, sin_m);
        d.mul_acc(self.c, sin_p);
        d *= TWO_THIRDS;

        let mut q = self.a;
        q *= cos;
        q.mul_acc(self.b, cos_m);
        q.mul_acc(self.c, cos_p);
        q *= TWO_THIRDS;

        Dq { d, q }
    }

    pub fn to_dq0(&self, cos: I1F31, sin: I1F31) -> Dq0<FRAC> {
        let dq = self.to_dq(cos, sin);

        Dq0 {
            d: dq.d,
            q: dq.q,
            zero: (*self).into(),
        }
    }
}

impl<const FRAC: i32> From<Abc<FRAC>> for FixedI32<FRAC> {
    fn from(abc: Abc<FRAC>) -> Self {
        let mut zero = abc.a;
        zero *= ONE_THIRD;
        zero.mul_acc(abc.b, ONE_THIRD);
        zero.mul_acc(abc.c, ONE_THIRD);
        zero
    }
}

impl<const FRAC: i32> Dq<FRAC> {
    pub fn to_abc(&self, sin: I1F31, cos: I1F31) -> Abc<FRAC> {
        /* sin and cos with 120 degree offsets */
        let (cos_m, sin_m) = shift_left_120(cos, sin);
        let (cos_p, sin_p) = shift_right_120(cos, sin);

        let mut a = self.d;
        a *= sin;
        a.mul_acc(self.q, cos);

        let mut b = self.d;
        b *= sin_m;
        b.mul_acc(self.q, cos_m);

        let mut c = self.d;
        c *= sin_p;
        c.mul_acc(self.q, cos_p);

        Abc { a, b, c }
    }
}

impl<const FRAC: i32> Dq0<FRAC> {
    pub fn to_abc(&self, cos: I1F31, sin: I1F31) -> Abc<FRAC> {
        Dq::<FRAC> {
            d: self.d,
            q: self.q,
        }
        .to_abc(cos, sin)
            + self.zero
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use fixed::types::{I0F32, I11F21};
    use fixed::FixedI32;

    #[test]
    fn clarke_transform() {
        let theta = I0F32::from_num(20. / 360.);
        let amplitude = I11F21::from_num(480.0);
        let polar = Polar { theta, amplitude };
        let abc = Abc::from(polar);

        let _alpha_beta_zero = AlphaBeta0::from(abc);

        // FIXME:  Need a check here
    }

    #[test]
    fn dq0_transform() {
        let theta = I0F32::from_num(20. / 360.);
        let amplitude = FixedI32::<5>::from_num(12e3);
        let polar = Polar { theta, amplitude };
        let abc = Abc::from(polar);

        let (cos, sin) = cos_sin(theta);
        let dq0 = abc.to_dq0(cos, sin);

        println!("{:?}", dq0);
        // // we loose a little precision in the transform
        // // I think most of this is in the sin/cos shifts
        // // TODO:  Can we make this better?
        // assert_eq!(dq0.d, I11F21::from_num(479.999999));
        // assert_eq!(dq0.q, I11F21::from_num(0.0));
        // assert_eq!(dq0.z, I11F21::from_num(-0.000001));
    }
}
