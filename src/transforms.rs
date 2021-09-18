
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

	// pub fn to_dq0(self, sin_cos: SinCos) -> Dq0 {
		
	// 	/* calculate SinCos s */

	// }
}