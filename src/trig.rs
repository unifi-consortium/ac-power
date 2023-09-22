use fixed::types::I1F31;
use idsp::cossin;

#[derive(Debug)]
pub struct SinCos {
    pub sin: I1F31,
    pub cos: I1F31,
}

const ONE_HALF: I1F31 = I1F31::from_bits(0x4000_0000);
const SQRT_3_OVER_2: I1F31 = I1F31::from_bits(0x6ed9_eba1);

impl SinCos {
    pub fn from_theta(theta: I1F31) -> Self {
        let (cos_i32, sin_i32) = cossin(theta.to_bits());
        let sin = I1F31::from_bits(sin_i32);
        let cos = I1F31::from_bits(cos_i32);

        Self { sin, cos }
    }

    /// Shifts sin/cos values 120 degrees right (+2pi/3)
    ///
    /// Use Ptolemy's theorem rather than a new sin/cos lookup
    pub fn shift_right_120(&self) -> Self {
        let mut sin = self.sin * (-ONE_HALF);
        sin.saturating_mul_acc(SQRT_3_OVER_2, self.cos);

        let mut cos = self.cos * (-ONE_HALF);
        cos.saturating_mul_acc(-SQRT_3_OVER_2, self.sin);

        Self { sin, cos }
    }

    /// Shifts sin/cos values 120 degrees left (-2pi/3)
    ///
    /// Use Ptolemy's theorem rather than a new sin/cos lookup
    pub fn shift_left_120(&self) -> Self {
        let mut sin = self.sin * (-ONE_HALF);
        sin.saturating_mul_acc(-SQRT_3_OVER_2, self.cos);

        let mut cos = self.cos * (-ONE_HALF);
        cos.saturating_mul_acc(SQRT_3_OVER_2, self.sin);

        Self { sin, cos }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn shift_left() {
        let angle = 0.2;
        let sin_cos = SinCos::from_theta(I1F31::from_num(angle));
        let sin_cos_shift_left = sin_cos.shift_left_120();
        assert_eq!(sin_cos_shift_left.sin, I1F31::from_num(-0.994521895));
        assert_eq!(sin_cos_shift_left.cos, I1F31::from_num(0.104528463));
    }

    #[test]
    fn shift_right() {
        let angle = 0.2;
        let sin_cos = SinCos::from_theta(I1F31::from_num(angle));
        let sin_cos_shift_right = sin_cos.shift_right_120();
        assert_eq!(sin_cos_shift_right.sin, I1F31::from_num(0.406736642));
        assert_eq!(sin_cos_shift_right.cos, I1F31::from_num(-0.9135454576));
    }
}
