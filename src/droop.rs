
pub struct Impedance {
	pub r: i32,
	pub l: i32
}

pub struct Droop {
	// virtual impedance
	z: Impedance,

	// governing coefficients
	G00: i32,
	G01: i32,
	G10: i32,
	G11: i32,
	Y00: i32,
	Y01: i32,
	Y10: i32,
	Y11: i32,

	// secondary and tertiary biases
	v_bias: i32,
	omega_bias: i32
	ia_bias: i32,
	ir_bias: i32

}

impl Droop {
	pub fn set_rotation_angle(&self, angle:i32) {

	}

	pub fn set_gain(&self, kv: i32, kf: i32) {

	}

	pub fn update_secondary(&self, v_bias: i32, omega_bias: i32) {
		self.v_bias = v_bias;
		self.omega_bias = omega_bias;
	}

	pub fn update_tertiary(&self, ia_bias: i32, ir_bias: i32) {
		self.ia_bias = v_bias;
		self.ir_bias = hz_bias;
	}



	pub fn update(&self, sin_cos: SinCos, v: i32, omega: i32) -> i32 {
		// calculations of errors
		let v_error = v - self.v_ref + self.v_bias;
		let omega_error = omega - self.omega_ref + self.omega_bias;

		// primary control response
		let mut ia: i64 = multiply(G00, v_error);
		ia += multiply(G01, omega_error);
		let mut ir: i64 = multiply(G10, v_error);
		ir += multiply(G11, omega_error);

		// tertiary
		ia += self.ia_bias;
		ir += self.ir_bias;

		// impose limits

		// calculate virtual source voltage
		ia_p = 1;
		ir_p = 1;

		let mut a: i64 = multiply(ir_p, self.z);
		a += (i64:from(v)<<32);
		a = multiply(sin_cos.sin, i32::from(a >> 32));

		let mut b: i64 = multiply(ia_p, self.z);
		b = multiply(sin_cos.cis, i32::from(b >> 32));

		i32::from((a + b) >> 32)
	}
}