use crate::common_tables::SIN_TABLE;

#[derive(Debug)]
pub struct SinCos{
	pub sin: i32,
	pub cos:i32
}


const DN: i32 = 0x1921FB5; // delta between the two points (fixed), in this case 2*pi/FAST_MATH_TABLE_SIZE
const CONTROLLER_Q31_SHIFT: u32 = 32 - 9;

#[inline(always)]
fn clip_i64_to_i32(x: i64) -> i32{
	if x > (i32::MAX as i64) {return i32::MAX;}
	if x < (i32::MIN as i64) {return i32::MIN;}
	x as i32
}

#[inline(always)]
fn interpolate(f1: i32, f2: i32, d1: i32, d2: i32, fract: i32) -> i32{
	let df = f2 - f1;
	let mut temp: i64 = (DN as i64)*((d1 as i64) + (d2 as i64));
	temp = temp - ((df as i64) << 32);
	temp = (fract as i64)*(temp >> 31);
	temp = temp + ((3*(df as i64) << 31) - ((d2 as i64) + ((d1 as i64) << 1))*(DN as i64));
	temp = (fract as i64)*(temp >> 31);
	temp = temp + (d1 as i64)*(DN as i64);
	temp = (fract as i64)*(temp >> 31);
	return clip_i64_to_i32((temp >> 31) + (f1 as i64));
}

impl SinCos{
	pub fn from_theta(theta: i32) -> Self{
		/* Calculate the nearest index */
	  	let index_s:u16 = ((theta as u32) >> CONTROLLER_Q31_SHIFT) as u16;
	  	let index_c:u16 = (index_s + 128) & 0x1ff;

		/* Calculation of fractional value */
	  	let fract: i32 = (theta - ((index_s as i32) << CONTROLLER_Q31_SHIFT)) << 8;

		/* Calculation of cosine value */
		let mut f1: i32 = SIN_TABLE[(index_c+0) as usize] as i32;
		let mut f2: i32 = SIN_TABLE[(index_c+1) as usize] as i32;
		let mut d1: i32 = (SIN_TABLE[(index_s+0) as usize] as i32).wrapping_neg();
		let mut d2: i32 = (SIN_TABLE[(index_s+1) as usize] as i32).wrapping_neg();
		let cos_val = interpolate(f1, f2, d1, d2, fract);

		/* Calculation of sine value */
		f1 = SIN_TABLE[(index_s+0) as usize] as i32;
		f2 = SIN_TABLE[(index_s+1) as usize] as i32;
		d1 = (SIN_TABLE[(index_c+0) as usize] as i32).wrapping_neg();
		d2 = (SIN_TABLE[(index_c+1) as usize] as i32).wrapping_neg();
		let sin_val = interpolate(f1, f2, d1, d2, fract);

		Self{sin:sin_val, cos:cos_val}
	}
}