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

	// DQ0 Transform
	pub fn to_dq0(self, sin_cos: SinCos<f32>) -> Dq0<f32> {
		
		/* sin and cos with 120 degree offsets */
		let sin_cos_shift_right = sin_cos.shift_right_120();
		let sin_cos_shift_left = sin_cos.shift_left_120();


		let d: f32 = 0.666666666666*(self.a*sin_cos.sin + self.b*sin_cos_shift_left.sin + self.c*sin_cos_shift_right.sin);
		let q: f32 = 0.666666666666*(self.a*sin_cos.cos + self.b*sin_cos_shift_left.cos + self.c*sin_cos_shift_right.cos);
		let z: f32 = 0.333333333333*self.a + 0.333333333333*self.b + 0.333333333333*self.c;

		Dq0{d, q, z}
	}
}




#[cfg(test)]
mod i32_tests {

    use crate::trig::SinCos;
    use crate::transforms::Abc;

    #[test]
    fn clark_transform() {
        // test fixed-point
        let abc: Abc<i32> = Abc{a:3827, b:6088, c:-9914};
        let alpha_beta = abc.to_alpha_beta();
        assert_eq!(alpha_beta.alpha, 3826);
        assert_eq!(alpha_beta.beta, 13065);
        assert_eq!(alpha_beta.gamma, 0);
    }

    #[test]
    fn dq0_transform() {
        let abc: Abc<i32> = Abc {a:0, b:-86603, c:86603};
        let sin_cos = SinCos::<i32>::from_theta(0);
        let dq0 = abc.to_dq0(sin_cos);
        assert_eq!(dq0.d, 100000);
        assert_eq!(dq0.q, 0);
        assert_eq!(dq0.z, 0);
    }
}

#[cfg(test)]
mod f32_tests {

    use crate::trig::SinCos;
    use crate::transforms::Abc;

    // define a simple macro for comparing floating point numbers
    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) { panic!(); }
        }
    }

    #[test]
    fn clark_transform() {
        let abc: Abc<f32> = Abc{a: 3827.0, b:6088.0, c:-9914.0};
        let alpha_beta = abc.to_alpha_beta();
        assert_delta!(alpha_beta.alpha, 3826.667, 1e-10);
        assert_delta!(alpha_beta.beta, 13065.579, 1e-10);
        assert_delta!(alpha_beta.gamma, 0.33325195, 1e-10);
    }

    #[test]
    fn dq0_transform() {
        let abc: Abc<f32> = Abc {a:0.0, b:-86603.0, c:86603.0};
        let sin_cos = SinCos::<f32>::from_theta(0.0);
        let dq0 = abc.to_dq0(sin_cos);
        assert_delta!(dq0.d, 100000.53, 1e-10);
        assert_delta!(dq0.q, 0.0, 1e-10);
        assert_delta!(dq0.z, 0.0, 1e-10);
    }
}