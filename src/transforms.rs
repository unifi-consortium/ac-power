use crate::constants::{ONE_THIRD, SQRT_3_OVER_3, TWO_THIRDS};
use crate::reference_frames::{Abc, AlphaBeta, Dq0, Polar};
use crate::trig::{shift_left_120, shift_right_120, sin_cos};
use core::convert::From;
use fixed::types::I1F31;

// polar to Abc transformation
impl<const FRAC: i32> From<Polar<FRAC>> for Abc<FRAC> {
    fn from(polar: Polar<FRAC>) -> Self {
        let (sin, cos) = sin_cos(polar.theta);
        let (sin_m, _) = shift_left_120(sin, cos);
        let (sin_p, _) = shift_right_120(sin, cos);

        let mut a = polar.amplitude;
        let mut b = polar.amplitude;
        let mut c = polar.amplitude;
        a *= sin;
        b *= sin_m;
        c *= sin_p;
        Self { a, b, c }
    }
}

// abc to alpha beta (clarke) transform
impl<const FRAC: i32> From<Abc<FRAC>> for AlphaBeta<FRAC> {
    fn from(abc: Abc<FRAC>) -> Self {
        let mut alpha = abc.a;
        alpha *= TWO_THIRDS;
        alpha.saturating_mul_acc(abc.b, -ONE_THIRD);
        alpha.saturating_mul_acc(abc.c, -ONE_THIRD);

        let mut beta = abc.b;
        beta *= SQRT_3_OVER_3;
        beta.saturating_mul_acc(abc.c, -SQRT_3_OVER_3);

        let mut gamma = abc.a;
        gamma *= ONE_THIRD;
        gamma.saturating_mul_acc(abc.b, ONE_THIRD);
        gamma.saturating_mul_acc(abc.c, ONE_THIRD);

        Self { alpha, beta, gamma }
    }
}

impl<const FRAC: i32> Abc<FRAC> {
    // DQ0 Transform
    pub fn to_dq0(&self, sin: I1F31, cos: I1F31) -> Dq0<FRAC> {
        /* sin and cos with 120 degree offsets */
        let (sin_m, cos_m) = shift_left_120(sin, cos);
        let (sin_p, cos_p) = shift_right_120(sin, cos);

        let mut d = self.a;
        d *= sin;
        d.saturating_mul_acc(self.b, sin_m);
        d.saturating_mul_acc(self.c, sin_p);
        d *= TWO_THIRDS;

        let mut q = self.a;
        q *= cos;
        q.saturating_mul_acc(self.b, cos_m);
        q.saturating_mul_acc(self.c, cos_p);
        q *= TWO_THIRDS;

        let mut z = self.a;
        z *= ONE_THIRD;
        z.saturating_mul_acc(self.b, ONE_THIRD);
        z.saturating_mul_acc(self.c, ONE_THIRD);

        Dq0 { d, q, z }
    }
}

impl<const FRAC: i32> Dq0<FRAC> {
    // DQ0 Transform
    pub fn to_abc(&self, sin: I1F31, cos: I1F31) -> Abc<FRAC> {
        /* sin and cos with 120 degree offsets */
        let (sin_m, cos_m) = shift_left_120(sin, cos);
        let (sin_p, cos_p) = shift_right_120(sin, cos);

        let mut a = self.d;
        a *= cos;
        a.saturating_mul_acc(self.q, -sin);
        a = a.saturating_add(self.z);

        let mut b = self.d;
        b *= cos_m;
        b.saturating_mul_acc(self.q, -sin_m);
        b = b.saturating_add(self.z);

        let mut c = self.d;
        c *= cos_p;
        c.saturating_mul_acc(self.q, -sin_p);
        c = c.saturating_add(self.z);

        Abc { a, b, c }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use approx::assert_relative_eq;

    use fixed::types::{
        I11F21, // 1 sine bit and 10 integer bits allows up to 1kV
        I1F31,
    };
    use fixed::FixedI32;

    #[test]
    fn clark_transform() {
        let theta = I1F31::from_num(20. / 360.);
        let amplitude = I11F21::from_num(480.0);
        let polar = Polar { theta, amplitude };
        let abc = Abc::from(polar);

        let alpha_beta = AlphaBeta::from(abc);

        // we loose a little precision in the transform
        // I think most of this is in the sin/cos shifts
        // TODO:  Can we make this better?
        assert_relative_eq!(f64::from(alpha_beta.alpha), 83.34947681427002);
        assert_relative_eq!(f64::from(alpha_beta.beta), -472.70061111450195);
        assert_relative_eq!(f64::from(alpha_beta.gamma), -1.430511474609375e-6);
    }

    #[test]
    fn dq0_transform() {
        let theta = I1F31::from_num(20. / 360.);
        let amplitude = FixedI32::<5>::from_num(12e3);
        let polar = Polar { theta, amplitude };
        let abc = Abc::from(polar);

        let (sin, cos) = sin_cos(theta);
        let dq0 = abc.to_dq0(sin, cos);

        println!("{:?}", dq0);
        // // we loose a little precision in the transform
        // // I think most of this is in the sin/cos shifts
        // // TODO:  Can we make this better?
        // assert_eq!(dq0.d, I11F21::from_num(479.999999));
        // assert_eq!(dq0.q, I11F21::from_num(0.0));
        // assert_eq!(dq0.z, I11F21::from_num(-0.000001));
    }
}
