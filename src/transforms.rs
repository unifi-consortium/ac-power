use crate::trig::SinCos;
use fixed::FixedI32;
use fixed::types::I1F31;
use fixed::types::extra::LeEqU32;


// define constants
const ONE_THIRD:I1F31 = I1F31::from_bits(0x2aaa_aaab);
const TWO_THIRDS:I1F31 = I1F31::from_bits(0x5555_5555);
const SQRT_3_OVER_3:I1F31 = I1F31::from_bits(0x49e6_9d16);

// alpha beta
#[derive(Debug)]
pub struct AlphaBeta<Frac: LeEqU32>{
	pub alpha: FixedI32<Frac>,
	pub beta: FixedI32<Frac>,
	pub gamma: FixedI32<Frac>
}


// abc
#[derive(Debug)]
pub struct Abc<Frac: LeEqU32>{
	pub a: FixedI32<Frac>,
	pub b: FixedI32<Frac>,
	pub c: FixedI32<Frac>
}

// dq0
#[derive(Debug)]
pub struct Dq0<Frac: LeEqU32> {
	pub d: FixedI32<Frac>,
	pub q: FixedI32<Frac>,
	pub z: FixedI32<Frac>
}



impl <Frac> Abc<Frac>
where Frac: LeEqU32 {
	pub fn from_polar(amplitude: FixedI32<Frac>, theta: I1F31) -> Self{

		let sin_cos = SinCos::from_theta(theta);
		let minus = sin_cos.shift_left_120();
		let plus = sin_cos.shift_right_120();
		let mut a = amplitude;
		let mut b = amplitude;
		let mut c = amplitude;
		a *= sin_cos.sin;
		b *= minus.sin;
		c *= plus.sin;
		Self{a, b, c}

	}
	// clark transform
	pub fn to_alpha_beta(&self) -> AlphaBeta<Frac> {
		let mut alpha = self.a;
		alpha *= TWO_THIRDS;
		alpha.saturating_mul_acc(self.b, -ONE_THIRD);
		alpha.saturating_mul_acc(self.c, -ONE_THIRD);

		let mut beta = self.b;
		beta *= SQRT_3_OVER_3;
		beta.saturating_mul_acc(self.c, -SQRT_3_OVER_3);

		let mut gamma = self.a;
		gamma *= ONE_THIRD;
		gamma.saturating_mul_acc(self.b, ONE_THIRD);
		gamma.saturating_mul_acc(self.c, ONE_THIRD);

		AlphaBeta{alpha, beta, gamma}
	}

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
		z *= TWO_THIRDS;

		Dq0{d, q, z}
	}
}


#[cfg(test)]
mod tests {

    use crate::trig::SinCos;
    use crate::transforms::Abc;


    use fixed::types::{I1F31,
                       I11F21	// 1 sine bit and 10 integer bits allows up to 1kV
                       };

    #[test]
    fn clark_transform() {
        let theta = I1F31::from_num(20./360.);
    	let amplitude = I11F21::from_num(480.0);
        let abc = Abc::from_polar(amplitude, theta);

        let alpha_beta = abc.to_alpha_beta();

        // we loose a little precision in the transform
        // I think most of this is in the sin/cos shifts
        // TODO:  Can we make this better?
        assert_eq!(alpha_beta.alpha, I11F21::from_num(83.3511243));
        assert_eq!(alpha_beta.beta, I11F21::from_num(-472.7077217));
        assert_eq!(alpha_beta.gamma, I11F21::from_num(-0.000001));
    }

    #[test]
    fn dq0_transform() {
    	let theta = I1F31::from_num(20./360.);
    	let amplitude = I11F21::from_num(480.0);
        let abc = Abc::from_polar(amplitude, theta);

        let sin_cos = SinCos::from_theta(theta);
        let dq0 = abc.to_dq0(sin_cos);

        // we loose a little precision in the transform
        // I think most of this is in the sin/cos shifts
        // TODO:  Can we make this better?
        assert_eq!(dq0.d, I11F21::from_num(479.999999));
        assert_eq!(dq0.q, I11F21::from_num(0.0));
        assert_eq!(dq0.z, I11F21::from_num(-0.000001));
    }
}