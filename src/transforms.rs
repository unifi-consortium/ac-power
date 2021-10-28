use crate::trig::SinCos;
use core::convert::From;
use fixed::types::extra::LeEqU32;
use fixed::types::I1F31;
use fixed::FixedI32;

// define constants
const ONE_THIRD: I1F31 = I1F31::from_bits(0x2aaa_aaab);
const TWO_THIRDS: I1F31 = I1F31::from_bits(0x5555_5555);
const SQRT_3_OVER_3: I1F31 = I1F31::from_bits(0x49e6_9d16);

// alpha beta
#[derive(Debug)]
pub struct AlphaBeta<Frac: LeEqU32> {
    pub alpha: FixedI32<Frac>,
    pub beta: FixedI32<Frac>,
    pub gamma: FixedI32<Frac>,
}

// abc
#[derive(Debug)]
pub struct Abc<Frac: LeEqU32> {
    pub a: FixedI32<Frac>,
    pub b: FixedI32<Frac>,
    pub c: FixedI32<Frac>,
}

// dq0
#[derive(Debug)]
pub struct Dq0<Frac: LeEqU32> {
    pub d: FixedI32<Frac>,
    pub q: FixedI32<Frac>,
    pub z: FixedI32<Frac>,
}

// polar
#[derive(Debug)]
pub struct Polar<Frac: LeEqU32> {
    pub amplitude: FixedI32<Frac>,
    pub theta: I1F31,
}

// polar to Abc transformation
impl<Frac> From<Polar<Frac>> for Abc<Frac>
where
    Frac: LeEqU32,
{
    fn from(polar: Polar<Frac>) -> Self {
        let sin_cos = SinCos::from_theta(polar.theta);
        let minus = sin_cos.shift_left_120();
        let plus = sin_cos.shift_right_120();

        let mut a = polar.amplitude;
        let mut b = polar.amplitude;
        let mut c = polar.amplitude;
        a *= sin_cos.sin;
        b *= minus.sin;
        c *= plus.sin;
        Self { a, b, c }
    }
}

// abc to alpha beta (clark) transform
impl<Frac> From<Abc<Frac>> for AlphaBeta<Frac>
where
    Frac: LeEqU32,
{
    fn from(abc: Abc<Frac>) -> Self {
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

impl<Frac> Abc<Frac>
where
    Frac: LeEqU32,
{
    // DQ0 Transform
    pub fn to_dq0(&self, sin_cos: SinCos) -> Dq0<Frac> {
        /* sin and cos with 120 degree offsets */
        let sin_cos_shift_right = sin_cos.shift_right_120();
        let sin_cos_shift_left = sin_cos.shift_left_120();

        let mut d = self.a;
        d *= sin_cos.sin;
        d.saturating_mul_acc(self.b, sin_cos_shift_left.sin);
        d.saturating_mul_acc(self.c, sin_cos_shift_right.sin);
        d *= TWO_THIRDS;

        let mut q = self.a;
        q *= sin_cos.cos;
        q.saturating_mul_acc(self.b, sin_cos_shift_left.cos);
        q.saturating_mul_acc(self.c, sin_cos_shift_right.cos);
        q *= TWO_THIRDS;

        let mut z = self.a;
        z *= ONE_THIRD;
        z.saturating_mul_acc(self.b, ONE_THIRD);
        z.saturating_mul_acc(self.c, ONE_THIRD);

        Dq0 { d, q, z }
    }
}

#[cfg(test)]
mod tests {

    use crate::transforms::{Abc, AlphaBeta, Polar};
    use crate::trig::SinCos;

    use fixed::types::extra::*;
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
        assert_eq!(alpha_beta.alpha, I11F21::from_num(83.3511243));
        assert_eq!(alpha_beta.beta, I11F21::from_num(-472.7077217));
        assert_eq!(alpha_beta.gamma, I11F21::from_num(-0.000001));
    }

    #[test]
    fn dq0_transform() {
        let theta = I1F31::from_num(20. / 360.);
        let amplitude = FixedI32::<U5>::from_num(12e3);
        let polar = Polar { theta, amplitude };
        let abc = Abc::from(polar);

        let sin_cos = SinCos::from_theta(theta);
        let dq0 = abc.to_dq0(sin_cos);

        println!("{:?}", dq0);
        // // we loose a little precision in the transform
        // // I think most of this is in the sin/cos shifts
        // // TODO:  Can we make this better?
        // assert_eq!(dq0.d, I11F21::from_num(479.999999));
        // assert_eq!(dq0.q, I11F21::from_num(0.0));
        // assert_eq!(dq0.z, I11F21::from_num(-0.000001));
    }
}
