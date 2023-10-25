// use fixed::types::extra::LeEqU32;
use fixed::types::I1F31;
use fixed::FixedI32;

pub struct PiFilter {
    kp: I1F31,
    ki: I1F31,
    max_integral: I1F31,
    integral_term: I1F31,
}

impl PiFilter {
    pub fn new(kp: i32, ki: i32, max_integral: i32) -> Self {
        let kp = I1F31::from_bits(kp);
        let ki = I1F31::from_bits(ki);
        let max_integral = I1F31::from_bits(max_integral);
        let integral_term = I1F31::from_bits(0);
        Self {
            kp,
            ki,
            max_integral,
            integral_term,
        }
    }

    pub fn update<const FRAC: i32>(&mut self, error: FixedI32<FRAC>) -> I1F31 {
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
