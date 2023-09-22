use fixed::types::I1F31;
use idsp::cossin;

const ONE_HALF: I1F31 = I1F31::from_bits(0x4000_0000);
const SQRT_3_OVER_2: I1F31 = I1F31::from_bits(0x6ed9_eba1);

pub fn sin_cos(theta: I1F31) -> (I1F31, I1F31) {
    // use the idsp library cos/sin function
    let (cos_i32, sin_i32) = cossin(theta.to_bits());

    // convert the result to fixed datatype
    let sin_val = I1F31::from_bits(sin_i32);
    let cos_val = I1F31::from_bits(cos_i32);

    (sin_val, cos_val)
}

/// Shifts sin/cos values 120 degrees right (+2pi/3)
///
/// Use Ptolemy's theorem rather than a new sin/cos lookup
pub fn shift_right_120(sin: I1F31, cos: I1F31) -> (I1F31, I1F31) {
    let mut sin_shifted = sin * (-ONE_HALF);
    sin_shifted.saturating_mul_acc(SQRT_3_OVER_2, cos);

    let mut cos_shifted = cos * (-ONE_HALF);
    cos_shifted.saturating_mul_acc(-SQRT_3_OVER_2, sin);

    (sin_shifted, cos_shifted)
}

/// Shifts sin/cos values 120 degrees left (-2pi/3)
///
/// Use Ptolemy's theorem rather than a new sin/cos lookup
pub fn shift_left_120(sin: I1F31, cos: I1F31) -> (I1F31, I1F31) {
    let mut sin_shifted = sin * (-ONE_HALF);
    sin_shifted.saturating_mul_acc(-SQRT_3_OVER_2, cos);

    let mut cos_shifted = cos * (-ONE_HALF);
    cos_shifted.saturating_mul_acc(SQRT_3_OVER_2, sin);

    (sin_shifted, cos_shifted)
}

#[cfg(test)]
mod tests {

    use super::*;
    use approx::assert_abs_diff_eq;

    use std::f64::consts::PI;

    #[test]
    fn shift_left() {
        let angle: f64 = 0.2;
        let (sin, cos) = sin_cos(I1F31::from_num(angle));
        let (sin_shifted, cos_shifted) = shift_left_120(sin, cos);

        assert_abs_diff_eq!(
            f64::from(sin_shifted),
            (PI * angle - 2.0 * PI / 3.0).sin(),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            f64::from(cos_shifted),
            (PI * angle - 2.0 * PI / 3.0).cos(),
            epsilon = 0.0001
        );
    }

    #[test]
    fn shift_right() {
        let angle: f64 = 0.2;
        let (sin, cos) = sin_cos(I1F31::from_num(angle));
        let (sin_shifted, cos_shifted) = shift_right_120(sin, cos);

        assert_abs_diff_eq!(
            f64::from(sin_shifted),
            (PI * angle + 2.0 * PI / 3.0).sin(),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            f64::from(cos_shifted),
            (PI * angle + 2.0 * PI / 3.0).cos(),
            epsilon = 0.0001
        );
    }
}
