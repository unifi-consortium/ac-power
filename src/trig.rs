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
    use approx::assert_abs_diff_eq;

    use std::f64::consts::PI;

    #[test]
    fn shift_left() {
        let angle: f64 = 0.2;
        let sin_cos = SinCos::from_theta(I1F31::from_num(angle));
        let sin_cos_shift_left = sin_cos.shift_left_120();

        assert_abs_diff_eq!(
            f64::from(sin_cos_shift_left.sin),
            (PI * angle - 2.0 * PI / 3.0).sin(),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            f64::from(sin_cos_shift_left.cos),
            (PI * angle - 2.0 * PI / 3.0).cos(),
            epsilon = 0.0001
        );
    }

    #[test]
    fn shift_right() {
        let angle = 0.2;
        let sin_cos = SinCos::from_theta(I1F31::from_num(angle));
        let sin_cos_shift_right = sin_cos.shift_right_120();

        assert_abs_diff_eq!(
            f64::from(sin_cos_shift_right.sin),
            (PI * angle + 2.0 * PI / 3.0).sin(),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            f64::from(sin_cos_shift_right.cos),
            (PI * angle + 2.0 * PI / 3.0).cos(),
            epsilon = 0.0001
        );
    }
}
