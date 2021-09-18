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
fn multiply(a: i32, b: i32) -> i32 {
	(((a as i64) * (b as i64)) >> 32) as i32
}

pub fn clark(abc: Abc)->AlphaBeta{
	let mut tmp: i32 = multiply(abc.a, 0x55555555);
	tmp -= multiply(abc.b, 0x2aaaaaab);
	tmp -= multiply(abc.c, 0x2aaaaaab);
	let alpha: i32 = tmp;

	tmp = multiply(abc.b, 0x49e69d16);
	tmp -= multiply(abc.c, 0x49e69d16);
	let beta: i32 = tmp;

	tmp = multiply(abc.a, 0x2aaaaaab);
	tmp += multiply(abc.b, 0x2aaaaaab);
	tmp += multiply(abc.c, 0x2aaaaaab);
	let gamma: i32 = tmp;

	AlphaBeta{alpha, beta, gamma}
}