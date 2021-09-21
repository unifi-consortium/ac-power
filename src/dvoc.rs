use crate::trig::SinCos;

pub struct Dvoc<F, V, I> {
	// state variables
	pub v: i32,
	pub theta: i32,

	// secondary control biases
	pub omega_bias: <F>,
	pub v_bias: <V>,

	// tertiary control biases
	pub ia_bias: <I>,
	pub ir_bias: <I>
}



impl Dvoc {
	pub fn new(eps: f32,
		       k_v: f32,
		       k_i: f32,
		       v_nom: f32,
		       omega_nom:f32,
		       l:f32,
		       c:f32,
		       dt:f32) -> Self {

		// calculated constants
		// 683_565_275.5764316 = 1<< 31 / pi
		let k0: i32 = (omega_nom * dt * 683_565_275.5764316) as i32;





	}


	// step the the oscillator by step dts
	pub fn step(&self, step:u16) {

		// update theta
		let mut d_theta: i32 = multiply(self.k0, step);

	}

	pub 
}