use crate::trig::SinCos;

// alpha beta
#[derive(Debug)]
pub struct AlphaBeta{
	pub alpha: i32,
	pub beta: i32,
	pub gamma: i32
}


// abc
#[derive(Debug)]
pub struct Abc{
	pub a: i32,
	pub b: i32,
	pub c: i32
}

// dq0
#[derive(Debug)]
pub struct Dq0{
	pub d: i32,
	pub q: i32,
	pub z: i32
}


#[inline(always)]
fn multiply(a: i32, b: i32) -> i64 {
	(a as i64) * (b as i64)
}


impl Abc {
	// Clark Transform
	pub fn to_alpha_beta(self) -> AlphaBeta {
		let mut tmp: i64 = multiply(self.a, 0x55555555);
		tmp -= multiply(self.b, 0x2aaaaaab);
		tmp -= multiply(self.c, 0x2aaaaaab);
		let alpha: i32 = (tmp >> 32) as i32;

		tmp = multiply(self.b, 0x49e69d16);
		tmp -= multiply(self.c, 0x49e69d16);
		let beta: i32 = (tmp >> 32) as i32;

		tmp = multiply(self.a, 0x2aaaaaab);
		tmp += multiply(self.b, 0x2aaaaaab);
		tmp += multiply(self.c, 0x2aaaaaab);
		let gamma: i32 = (tmp >> 32) as i32;

		AlphaBeta{alpha, beta, gamma}
	}

	// DQ0 Transform
	pub fn to_dq0(self, sin_cos: SinCos) -> Dq0 {
		
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