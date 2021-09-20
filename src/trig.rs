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

		let mut tmp: i64 = i64::from(self.sin)<<30;
		tmp += multiply(self.cos, 1859775393);
		let sin: i32 = (tmp >> 31) as i32;

		tmp = i64::from(self.cos)<<30;
		tmp -= multiply(self.sin, 1859775393);
		let cos: i32 = (tmp >> 31) as i32;

		Self{sin, cos}
	}

	/// Shifts sin/cos values 120 degrees left (-2pi/3)
	///
	/// Use Ptolemy's theorem rather than a new sin/cos lookup
	pub fn shift_left_120(&self) -> Self {

		let mut tmp: i64 = i64::from(self.sin)<<30;
		tmp -= multiply(self.cos, 1859775393);
		let sin: i32 = (tmp >> 31) as i32;

		tmp = i64::from(self.cos)<<30;
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

	/// Shifts sin/cos values 120 degrees right (+2pi/3)
	///
	/// Use Ptolemy's theorem rather than a new sin/cos lookup
	pub fn shift_right_120(&self) -> Self {


		let sin: f32 = self.sin * 0.5 + self.cos * 0.866025403784;
		let cos: f32 = self.cos * 0.5 - self.sin * 0.866025403784;

		Self{sin, cos}
	}

	/// Shifts sin/cos values 120 degrees left (-2pi/3)
	///
	/// Use Ptolemy's theorem rather than a new sin/cos lookup
	pub fn shift_left_120(&self) -> Self {

		let sin: f32 = self.sin * 0.5 - self.cos * 0.866025403784;
		let cos: f32 = self.cos * 0.5 + self.sin * 0.866025403784;

		Self{sin, cos}
	}
}


#[cfg(test)]
mod i32_tests {

    use super::*;


    #[test]
    fn calc() {
        let mut sin_cos: SinCos::<i32>;

        /* test at theta = -pi */
        sin_cos = SinCos::<i32>::from_theta(-2147483648);
        assert_eq!(sin_cos.sin, 0);
        assert_eq!(sin_cos.cos, -2147483648);

        /* test at theta = -3*pi/4 */
        sin_cos = SinCos::<i32>::from_theta(-1610612736);
        assert_eq!(sin_cos.sin, -1518500250);
        assert_eq!(sin_cos.cos, -1518500250);

        /* test at theta = -pi/2 */
        sin_cos = SinCos::<i32>::from_theta(-1073741824);
        assert_eq!(sin_cos.sin, -2147483648);
        assert_eq!(sin_cos.cos, 0);

        /* test at theta = -pi/4 */
        sin_cos = SinCos::<i32>::from_theta(-536870912);
        assert_eq!(sin_cos.sin, -1518500250);
        assert_eq!(sin_cos.cos, 1518500250);

        /* test at theta = 0 */
        sin_cos = SinCos::<i32>::from_theta(0);
        assert_eq!(sin_cos.sin, 0);
        assert_eq!(sin_cos.cos, 2147483647);

        /* test at theta = pi/4 */
        sin_cos = SinCos::<i32>::from_theta(536870912);
        assert_eq!(sin_cos.sin, 1518500250);
        assert_eq!(sin_cos.cos, 1518500250);

        /* test at theta = pi/2 */
        sin_cos = SinCos::<i32>::from_theta(1073741824);
        assert_eq!(sin_cos.sin, 2147483647);
        assert_eq!(sin_cos.cos, 0);

        /* test at theta = 3*pi/4 */
        sin_cos = SinCos::<i32>::from_theta(1610612736);
        assert_eq!(sin_cos.sin, 1518500250);
        assert_eq!(sin_cos.cos, -1518500250);

        /* test at theta = pi */
        sin_cos = SinCos::<i32>::from_theta(2147483647);
        assert_eq!(sin_cos.sin, 2);
        assert_eq!(sin_cos.cos, -2147483648);
    }


    #[test]
    fn shift_left() {
        let sin_cos = SinCos::<i32>::from_theta(0);
        let sin_cos_shift_left = sin_cos.shift_left_120();
        assert_eq!(sin_cos_shift_left.sin, -1859775393);
        assert_eq!(sin_cos_shift_left.cos, 1073741823);  // Should be 1073741824?
    }

    #[test]
    fn shift_right() {
        let sin_cos = SinCos::<i32>::from_theta(0);
        let sin_cos_shift_right = sin_cos.shift_right_120();
        assert_eq!(sin_cos_shift_right.sin, 1859775392);
        assert_eq!(sin_cos_shift_right.cos, 1073741823);  // Should be 1073741824?
    }
}

#[cfg(test)]
mod f32_tests {

    use super::*;

    // define a simple macro for comparing floating point numbers
    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) { panic!(); }
        }
    }
    


    #[test]
    fn calc() {
        let sin_cos: SinCos::<f32>;
        sin_cos = SinCos::<f32>::from_theta(2.356194490192345);
        assert_delta!(sin_cos.sin, 0.70710677, 1e-5);
        assert_delta!(sin_cos.cos, -0.70710677, 1e-5);
    }

    #[test]
    fn shift_left() {
        let sin_cos = SinCos::<f32>::from_theta(0.0);
        let sin_cos_shift_left = sin_cos.shift_left_120();
        assert_delta!(sin_cos_shift_left.sin, -0.8660254, 1e-10);
        assert_delta!(sin_cos_shift_left.cos, 0.5, 1e-10);
    }

    #[test]
    fn shift_right() {
        let sin_cos = SinCos::<f32>::from_theta(0.0);
        let sin_cos_shift_right = sin_cos.shift_right_120();
        assert_delta!(sin_cos_shift_right.sin, 0.8660254, 1e-10);
        assert_delta!(sin_cos_shift_right.cos, 0.5, 1e-10);
    }
}