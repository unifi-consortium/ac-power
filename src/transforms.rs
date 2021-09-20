use crate::trig::SinCos;

// alpha beta
#[derive(Debug)]
pub struct AlphaBeta<T>{
	pub alpha: T,
	pub beta: T,
	pub gamma: T
}


// abc
#[derive(Debug)]
pub struct Abc<T>{
	pub a: T,
	pub b: T,
	pub c: T
}

// dq0
#[derive(Debug)]
pub struct Dq0<T>{
	pub d: T,
	pub q: T,
	pub z: T
}

#[inline(always)]
fn multiply(a: i32, b: i32) -> i64 {
	(a as i64) * (b as i64)
}



impl Abc <i32>{
	// Fixed-point clark transform
	pub fn to_alpha_beta(self) -> AlphaBeta<i32>
	{
		let mut tmp: i64 = multiply(self.a, 0x55555555);
		tmp -= multiply(self.b, 0x2aaaaaab);
		tmp -= multiply(self.c, 0x2aaaaaab);
		let alpha: i32 = (tmp >> 31) as i32;

		tmp = multiply(self.b, 0x6882f5c0);
		tmp -= multiply(self.c, 0x6882f5c0);
		let beta: i32 = (tmp >> 31) as i32;

		tmp = multiply(self.a, 0x2aaaaaab);
		tmp += multiply(self.b, 0x2aaaaaab);
		tmp += multiply(self.c, 0x2aaaaaab);
		let gamma: i32 = (tmp >> 31) as i32;

		AlphaBeta{alpha, beta, gamma}
	}

	// DQ0 Transform
	pub fn to_dq0(self, sin_cos: SinCos<i32>) -> Dq0<i32> {
		
		/* sin and cos with 120 degree offsets */
		let sin_cos_shift_right = sin_cos.shift_right_120();
		let sin_cos_shift_left = sin_cos.shift_left_120();

		let mut tmp: i64 = multiply(self.a, sin_cos.sin);
		tmp += multiply(self.b, sin_cos_shift_left.sin);
		tmp += multiply(self.c, sin_cos_shift_right.sin);
		tmp = tmp >> 30;
		let d: i32 = (multiply(tmp as i32, 0x55555555) >> 32) as i32;


		tmp = multiply(self.a, sin_cos.cos);
		tmp += multiply(self.b, sin_cos_shift_left.cos);
		tmp += multiply(self.c, sin_cos_shift_right.cos);
		tmp = tmp >> 30;
		let q: i32 = (multiply(tmp as i32, 0x55555555) >> 32) as i32;

		tmp = multiply(self.a, 0x55555555);
		tmp += multiply(self.b, 0x55555555);
		tmp += multiply(self.c, 0x55555555);
		let z = (tmp >> 32) as i32;

		Dq0{d, q, z}
	}
}


impl Abc <f32>{
	// Fixed-point clark transform
	pub fn to_alpha_beta(self) -> AlphaBeta<f32>
	{

		let alpha: f32 = 0.666666666666*self.a - 0.333333333333*self.b - 0.333333333333*self.c;
		let beta: f32 = 0.816496580928*self.b - 0.816496580928*self.c;
		let gamma: f32 = 0.333333333333*self.a + 0.333333333333*self.b + 0.333333333333*self.c;
		
		AlphaBeta{alpha, beta, gamma}
	}
}