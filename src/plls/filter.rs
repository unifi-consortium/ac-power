use fixed::types::{I0F32, I0F64, I1F31};
use fixed::FixedI32;

pub struct PiFilter {
    kp: I0F32,
    ki: I0F32,
    max_integral: i64,
    integral_term: i64,
}

impl PiFilter {
    pub fn new(kp: I0F32, ki: I0F32, max_integral: I0F64) -> Self {
        Self {
            kp,
            ki,
            max_integral: max_integral.to_bits(),
            integral_term: 0,
        }
    }

    pub fn update<const FRAC: i32>(&mut self, error: FixedI32<FRAC>) -> I0F32 {
        let mut proportional_term = self.kp;
        proportional_term *= error;

        self.integral_term += (self.ki.to_bits() as i64) * (error.to_bits() as i64);

        // anti-windup
        self.integral_term = self
            .integral_term
            .clamp(-self.max_integral, self.max_integral);

        proportional_term + I0F32::from_bits((self.integral_term >> 32) as i32)
    }
}

pub struct LowpassFilter<const FRAC: i32> {
    k: I1F31,
    pub value: FixedI32<FRAC>,
}

impl<const FRAC: i32> LowpassFilter<FRAC> {
    pub fn new(k: I1F31) -> Self {
        Self {
            k,
            value: FixedI32::<FRAC>::ZERO,
        }
    }

    pub fn update(&mut self, x: FixedI32<FRAC>) -> FixedI32<FRAC> {
        let mut delta = x - self.value;
        delta *= self.k;
        self.value += delta;
        self.value
    }
}
