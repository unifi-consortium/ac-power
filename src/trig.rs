use crate::common_tables::SIN_TABLE;

#[derive(Debug)]
pub struct SinCos<T>{
	pub sin: T,
	pub cos:T
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
	let mut temp: i64 = i64::from(DN)*(i64::from(d1) + i64::from(d2));
	temp = temp - (i64::from(df) << 32);
	temp = i64::from(fract)*(temp >> 31);
	temp = temp + ((3*i64::from(df) << 31) - (i64::from(d2) + (i64::from(d1) << 1))*i64::from(DN));
	temp = i64::from(fract)*(temp >> 31);
	temp = temp + i64::from(d1)*i64::from(DN);
	temp = i64::from(fract)*(temp >> 31);
	return clip_i64_to_i32((temp >> 31) + i64::from(f1));
}

#[inline(always)]
fn multiply(a: i32, b: i32) -> i64 {
	i64::from(a) * i64::from(b)
}

impl SinCos <i32>{
	pub fn from_theta(theta: i32) -> Self {
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

		let mut tmp: i64 = multiply(self.sin, -1073741824);
		tmp += multiply(self.cos, 1859775393);
		let sin: i32 = (tmp >> 31) as i32;

		tmp = multiply(self.cos, -1073741824);
		tmp -= multiply(self.sin, 1859775393);
		let cos: i32 = (tmp >> 31) as i32;

		Self{sin, cos}
	}

	/// Shifts sin/cos values 120 degrees left (-2pi/3)
	///
	/// Use Ptolemy's theorem rather than a new sin/cos lookup
	pub fn shift_left_120(&self) -> Self {

		let mut tmp: i64 = multiply(self.sin, -1073741824);
		tmp -= multiply(self.cos, 1859775393);
		let sin: i32 = (tmp >> 31) as i32;

		tmp = multiply(self.cos, -1073741824);
		tmp += multiply(self.sin, 1859775393);
		let cos: i32 = (tmp >> 31) as i32;

		Self{sin, cos}
	}
}

impl SinCos <f32>{
	pub fn from_theta(theta: f32) -> Self {
		let sin_cos = theta.sin_cos();
		Self{sin:sin_cos.0, cos:sin_cos.1}
	}
}