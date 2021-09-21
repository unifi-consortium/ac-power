use crate::common_tables::SIN_TABLE;
use fixed::types::I1F31;

#[derive(Debug)]
pub struct SinCos{
	pub sin: I1F31,
	pub cos:I1F31
}

const DN: i32 = 0x1921FB5; // delta between the two points (fixed), in this case 2*pi/FAST_MATH_TABLE_SIZE
const CONTROLLER_Q31_SHIFT: u32 = 32 - 9;

const ZP500:I1F31 = I1F31::from_bits(0x4000_0000);
const ZP866:I1F31 = I1F31::from_bits(0x6ed9_eba1);

#[inline(always)]
fn clip_i64_to_i32(x: i64) -> i32{
	if x > (i32::MAX as i64) {return i32::MAX;}
	if x < (i32::MIN as i64) {return i32::MIN;}
	x as i32
}

#[inline(always)]
fn interpolate(f1: i32, f2: i32, d1: i32, d2: i32, fract: i32) -> I1F31{
	let df = f2 - f1;
	let mut temp: i64 = i64::from(DN)*(i64::from(d1) + i64::from(d2));
	temp = temp - (i64::from(df) << 32);
	temp = i64::from(fract)*(temp >> 31);
	temp = temp + ((3*i64::from(df) << 31) - (i64::from(d2) + (i64::from(d1) << 1))*i64::from(DN));
	temp = i64::from(fract)*(temp >> 31);
	temp = temp + i64::from(d1)*i64::from(DN);
	temp = i64::from(fract)*(temp >> 31);
	I1F31::from_bits(clip_i64_to_i32((temp >> 31) + i64::from(f1)))
}

impl SinCos {
	pub fn from_theta(theta: I1F31) -> Self {
		/* Calculate the nearest index */
	  	let index_s:u16 = ((theta.to_bits() as u32) >> CONTROLLER_Q31_SHIFT) as u16;
	  	let index_c:u16 = (index_s + 128) & 0x1ff;

		/* Calculation of fractional value */
	  	let fract: i32 = (theta.to_bits() - ((i32::from(index_s)) << CONTROLLER_Q31_SHIFT)) << 8;

		/* Calculation of cosine value */
		let mut f1: i32 = SIN_TABLE[(index_c+0) as usize] as i32;
		let mut f2: i32 = SIN_TABLE[(index_c+1) as usize] as i32;
		let mut d1: i32 = (SIN_TABLE[(index_s+0) as usize] as i32).wrapping_neg();
		let mut d2: i32 = (SIN_TABLE[(index_s+1) as usize] as i32).wrapping_neg();
		let cos = interpolate(f1, f2, d1, d2, fract);

		/* Calculation of sine value */
		f1 = SIN_TABLE[(index_s+0) as usize] as i32;
		f2 = SIN_TABLE[(index_s+1) as usize] as i32;
		d1 = SIN_TABLE[(index_c+0) as usize] as i32;
		d2 = SIN_TABLE[(index_c+1) as usize] as i32;
		let sin = interpolate(f1, f2, d1, d2, fract);

		Self{sin, cos}
	}

	/// Shifts sin/cos values 120 degrees right (+2pi/3)
	///
	/// Use Ptolemy's theorem rather than a new sin/cos lookup
	pub fn shift_right_120(&self) -> Self {

		let mut sin = self.sin * (-ZP500);
		sin.saturating_mul_acc(ZP866, self.cos);

		let mut cos = self.cos * (-ZP500);
		cos.saturating_mul_acc(-ZP866, self.sin);

		Self{sin, cos}
	}

	/// Shifts sin/cos values 120 degrees left (-2pi/3)
	///
	/// Use Ptolemy's theorem rather than a new sin/cos lookup
	pub fn shift_left_120(&self) -> Self {

		let mut sin = self.sin * (-ZP500);
		sin.saturating_mul_acc(-ZP866, self.cos);

		let mut cos = self.cos * (-ZP500);
		cos.saturating_mul_acc(ZP866, self.sin);

		Self{sin, cos}
	}
}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn calc() {
        let mut sin_cos: SinCos;

        // test at theta = -pi
        sin_cos = SinCos::from_theta(I1F31::from_num(-1.));
        assert_eq!(sin_cos.sin, I1F31::from_num(0.));
        assert_eq!(sin_cos.cos, I1F31::from_num(-1.));

        // test at theta = -3*pi/4
        sin_cos = SinCos::from_theta(I1F31::from_num(-3./4.));
        assert_eq!(sin_cos.sin, I1F31::from_num(-0.707106781));
        assert_eq!(sin_cos.cos, I1F31::from_num(-0.707106781));

        // test at theta = -pi/2
        sin_cos = SinCos::from_theta(I1F31::from_num(-1./2.));
        assert_eq!(sin_cos.sin, I1F31::from_num(-1.));
        assert_eq!(sin_cos.cos, I1F31::from_num(0.));

        // test at theta = -pi/4
        sin_cos = SinCos::from_theta(I1F31::from_num(-1./4.));
        assert_eq!(sin_cos.sin, I1F31::from_num(-0.707106781));
        assert_eq!(sin_cos.cos, I1F31::from_num(0.707106781));

        // test at theta = 0
        sin_cos = SinCos::from_theta(I1F31::from_num(0.));
        assert_eq!(sin_cos.sin, I1F31::from_num(0.));
        assert_eq!(sin_cos.cos, I1F31::saturating_from_num(1.));

        // test at theta = pi/4
        sin_cos = SinCos::from_theta(I1F31::from_num(1./4.));
        assert_eq!(sin_cos.sin, I1F31::from_num(0.707106781));
        assert_eq!(sin_cos.cos, I1F31::from_num(0.707106781));

        // test at theta = pi/2
        sin_cos = SinCos::from_theta(I1F31::from_num(1./2.));
        assert_eq!(sin_cos.sin, I1F31::saturating_from_num(1.));
        assert_eq!(sin_cos.cos, I1F31::from_num(0.));

        // test at theta = 3*pi/4
        sin_cos = SinCos::from_theta(I1F31::from_num(3./4.));
        assert_eq!(sin_cos.sin, I1F31::from_num(0.707106781));
        assert_eq!(sin_cos.cos, I1F31::from_num(-0.707106781));

        // test at theta = pi
        sin_cos = SinCos::from_theta(I1F31::wrapping_from_num(1.));
        assert_eq!(sin_cos.sin, I1F31::from_num(0.));
        assert_eq!(sin_cos.cos, I1F31::from_num(-1.));
    }


    #[test]
    fn shift_left() {
    	let angle = 0.2;
        let sin_cos = SinCos::from_theta(I1F31::from_num(angle));
        let sin_cos_shift_left = sin_cos.shift_left_120();
        assert_eq!(sin_cos_shift_left.sin, I1F31::from_num(-0.994521895));
        assert_eq!(sin_cos_shift_left.cos, I1F31::from_num(0.104528463));
    }

    #[test]
    fn shift_right() {
    	let angle = 0.2;
        let sin_cos = SinCos::from_theta(I1F31::from_num(angle));
        let sin_cos_shift_right = sin_cos.shift_right_120();
        assert_eq!(sin_cos_shift_right.sin, I1F31::from_num(0.406736642));
        assert_eq!(sin_cos_shift_right.cos, I1F31::from_num(-0.9135454576));
    }
}