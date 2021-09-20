use crate::trig::SinCos;

pub struct Dvoc {
	pub v_alpha: i32,
	pub v_beta: i32,
	pub theta: i32,
	pub v: i32,
	pub r: SinCos,

	// private constants
	K0: i32,
	K1: i32,

}



impl Dvoc {
	pub fn new() -> Self {

	}


	pub fn step(&self, ia:i32, ir:i32) {

		let ia_error = self.ia - ia;
		let ir_error = self.ir - ir;
		let mut d: i64 = multiply(self.r.sin, ir_error);
		d += multiply(self.r.cos, ia_error);
		d = tmp >> 31;
		d = multiply(tmp, K2);


		let x0_squared: i32 = multiply(self.x0, self.x0);
		let p = (K1 - (x0_squared<<1))>>32;
		p = multiply(p, K0) >> 32;
		p = multiply(p, self.x0) >> 32;

		dx0dt = p - d;
	}
}