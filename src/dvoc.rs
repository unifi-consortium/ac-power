use crate::trig::SinCos;
use fixed::{FixedI32, FixedI64};
use fixed::types::I1F31;

pub struct Impedance <X: LeEqU32, R: LeEqU32> {
	x: FixedI32<X>,
	r: FixedI32<R>
}

pub struct Dvoc<V: LeEqU32, F: LeEqU32, I: LeEqU32, X: LeEqU32, R: LeEqU32> {
	// state variables
	pub v: FixedI32<V>,
	pub theta: I1F31,

	// gains
	pub k_v: FixedI32<V>,
	pub k_i: FixedI32<I>,

	// nominal values
	pub v_nom: FixedI32<V>,
	pub hz_nom: FixedI32<F>,

	// secondary control biases
	pub hz_bias: FixedI32<F>,
	pub v_bias: FixedI32<V>,

	// tertiary control biases
	pub ia_bias: FixedI32<I>,
	pub ir_bias: FixedI32<I>,

	// cross coupling
	pub x: SinCos,
	pub z: Impedance<X, R>,

	// private calculated constants
	k0: I1F31,
	k1: FixedI64<Sum<V, V>>,
	k2: I1F31,
	k3: I1F31

}


impl From<Config> for Dvoc {
  fn from(c: Config) -> Self {
  	// calculated constants


    Self {
    	
      k0, k1, k2, k3    
  	}
  }
}



impl <V, F, I>Dvoc<V, F, I> {
	pub fn new(eps: f32,
		       k_v: f32,
		       k_i: f32,
		       v_nom: f32,
		       hz_nom:f32,
		       l:f32,
		       c:f32,
		       dt:f32) -> Self {


		// simple algorithm for selected fixed point formats

		// calculated constants
		let k0 = I1F31::from_num(eps/(k_v*k_v));
		let k1 = I1F31::from_num(2 * V_nom * V_nom);
		let k2 = I1F31::from_num(k_v * k_i / (3*c));
		let k3 = I1F31::from_num(hz_nom * dt);

		Self{v, theta, v_nom, hz_nom, v_bias, hz_bias, ia_bias, ir_bias, k0, k1, k2}


	}

	pub fn update_secondary(&mut self, hz_bias: FixedI32<F>, v_bias: FixedI32<V>) {
		self.hz_bias = hz_bias;
		self.v_bias = v_bias;
	}

	pub fn update_tertiary(&mut self, ia_bias: FixedI32<I>, ir_bias: FixedI32<I>) {
		self.ia_bias = ia_bias;
		self.ir_bias = ir_bias;
	}

	// oscillator code
	pub fn step(&mut self, step:u16, ia: FixedI32<I>, ir: FixedI32<I>) {

		// calculate the active and reactive current errors
		let ia_err = self.ia_bias - ia;
		let ir_err = self.ir_bias - ir;

		// calculate the current feedback term for v
		let mut i_v = ir_error;
		i_v *= self.x.sin;
		i_v.saturating_mul_acc(ia_err, self.x.cos)
		i_v = i_v.saturating_mul(self.k2)

		
		// calculate the current feedback term for theta
		let mut i_theta = ia_error;
		i_theta *= self.x.sin;
		i_theta.saturating_mul_acc(ir_err, -self.x.cos)
		i_theta = i_theta.saturating_mul(self.k2).saturating_div(self.v)

		// calculate the voltage delta
		let two_v_squared = self.v.wide_mul(self.v).saturating_mul_int(2);
		let k1_minus_two_v_squared = self.k1 - two_v_squared;
		let mut damping: i64 = k1_minus_two_v_squared.to_bits() >> 32;
		damping *= K0;


		let mut theta_delta = self.k3 * step;
		theta_delta -= i_theta;

		// update the state parameters
		self.v.wrapping_add(theta_delta);
		self.theta.wrapping_add(v_delta);


		// step x[0] * (K1*K0 - 2*K0*x[0]**2) - step*K2 * (Ir_err*SIN + Ia_err*COS),
  	// step*K3 - step*K2/x[0] * (Ia_err*SIN - Ir_err*COS),


		// step*K0 * x[0] * (K1 - 2*x[0]**2) - step*K2 * (Ir_err*SIN + Ia_err*COS),
  	// step*K3 - step*K2/x[0] * (Ia_err*SIN - Ir_err*COS),
	}
}



#[cfg(test)]
mod tests {

	const CPU_CLK: f32 = 100_000_000;

    #[test]
    fn simulate() {

    	use core::f32::consts::PI;
    	let dvoc = Dvoc::new(eps 		= 15.,
    		                 k_v 		= 80.,
    		                 k_i 		= 0.2,
    		                 v_nom 		= 80.,
    		                 hz_nom		= 60.,
    		                 l          = 26.268e-6,
    		                 c          = 0.2679,
    		                 z          = Impedance::new(1.5e-3, 0.4),  // filter or virtual impedance
    		                 varphi     = PI / 2
    		                 dt         = 1./CPU_CLK      	  // set processor clock period
    		                 smax       = 1_000               // used to determine Qfmts
    		                 )

    	hz_bias = I3F29::from_num(-0.1); // Fixed point number with 3 integer bits and 29 fraction bits
    	v_bias = I5F27::from_num(5.);    // Fixed point number with 5 integer bits and 27 fraction bits
    	dvoc.update_secondary(hz_bias, v_bias);

    	ia_bias = I5F27::from_num(5.);    // Fixed point number with 5 integer bits and 27 fraction bits
    	ir_bias = I5F27::from_num(-0.1);  // Fixed point number with 5 integer bits and 27 fraction bits
    	dvoc.update_tertiary(ia_bias, ir_bias);

    	// loop to simulate routine running in an interrupt contect
    	ts = [0..1000];
    	let mut abc: Abc;
    	for t in ts:
    		dvoc.step(10000);  // if we are running at 10kHz, then we step 100MHz/10kHZ = 10000 each interrupt
    		abc = Abc::from_polar(dvoc.v, dvoc.theta);


    		pwma.set(abc.a);
    		pwmb.set(abc.b);
    		pwmc.set(abc.c);


}