
// alpha beta
pub struct AlphaBeta{
	pub alpha: i32,
	pub beta: i32,
	pub gamma: i32
}


// abc
pub struct Abc{
	pub a: i32,
	pub b: i32,
	pub c: i32
}

// dq0
pub struct Dq0{
	pub d: i32,
	pub q: i32,
	pub z: i32
}

fn dq<A, B, C, T, D, Q, Z>(abc: Abc<A, B, C>, theta: T) -> Dq0<D, Q, Z>{

}