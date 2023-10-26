// use fixed::types::extra::LeEqU32;
use fixed::types::{I0F32, I1F31};
use fixed::FixedI32;

pub struct PiFilter {
    kp: I0F32,
    ki: I0F32,
    max_integral: I0F32,
    integral_term: I0F32,
}

impl PiFilter {
    pub fn new(kp: I0F32, ki: I0F32, max_integral: I0F32) -> Self {
        Self {
            kp,
            ki,
            max_integral,
            integral_term: I0F32::ZERO,
        }
    }

    pub fn update<const FRAC: i32>(&mut self, error: FixedI32<FRAC>) -> I0F32 {
        let mut proportional_term = self.kp;
        proportional_term *= error;
        self.integral_term.saturating_mul_acc(self.ki, error);

        // anti-windup
        if self.integral_term > self.max_integral {
            self.integral_term = self.max_integral;
        }
        if self.integral_term < -self.max_integral {
            self.integral_term = -self.max_integral;
        }

        proportional_term + self.integral_term
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
