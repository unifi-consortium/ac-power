pub struct Dvoc {
	pub v_alpha: i32,
	pub v_beta: i32,

	// private constants
	K0: i32,
	K1: i32,

}



impl Dvoc {
	pub fn new() -> Self {

	}

	pub fn design() -> Self {

	}
	pub fn step(&self) {

		let v_alpha_squared: i32 = (multiply(self.v_alpha, self.v_alpha) >> 32) as i32;
		let v_beta_squared: i32 = (multiply(self.v_beta, self.v_beta) >> 32) as i32;

		/* non-linear differential equations */
		let mut tmp = K0;
		tmp -= multiply(K1, v_alpha_squared);
		tmp -= multiply(K1, v_beta_squared);
		tmp = v_alpha_dot >> 32;


		let mut v_alpha_dot: i32 = tmp as i32;
		v_alpha_dot = multiply(v_alpha_dot, v_alpha);
		v_alpha_dot -= multiply(K3, v_beta);

		let mut v_beta_dot: i32 = tmp as i32;
		v_beta_dot = multiply(v_beta_dot, v_beta);
		v_beta_dot -= multiply(K3, v_alpha);

		/* step */
		self.v_alpha += v_alpha_dot;
		self.v_beta += v_beta_dot;
	}
}