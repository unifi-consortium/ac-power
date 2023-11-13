use fixed::types::{I0F32, I1F31};
use idsp::cossin;

const ONE_HALF: I1F31 = I1F31::from_bits(0x4000_0000);
const SQRT_3_OVER_2: I1F31 = I1F31::from_bits(0x6ed9_eba1);

pub fn sin_cos(theta: I0F32) -> (I1F31, I1F31) {
    // use the idsp library cos/sin function
    let (cos_i32, sin_i32) = cossin(theta.to_bits());

    // convert the result to fixed datatype
    let sin_val = I1F31::from_bits(sin_i32);
    let cos_val = I1F31::from_bits(cos_i32);

    (sin_val, cos_val)
}

// Use Ptolemy's theorem to rotate a sin/cos pair
pub fn rotate(sina: I1F31, cosa: I1F31, sinb: I1F31, cosb: I1F31) -> (I1F31, I1F31) {
    let sin = sina * cosb + cosa * sinb;
    let cos = cosa * cosb - sina * sinb;
    (sin, cos)
}

/// Shifts sin/cos values 120 degrees right (+2pi/3)
pub fn shift_right_120(sin: I1F31, cos: I1F31) -> (I1F31, I1F31) {
    rotate(sin, cos, SQRT_3_OVER_2, -ONE_HALF)
}

/// Shifts sin/cos values 120 degrees left (-2pi/3)
pub fn shift_left_120(sin: I1F31, cos: I1F31) -> (I1F31, I1F31) {
    rotate(sin, cos, -SQRT_3_OVER_2, -ONE_HALF)
}

// use chebyshev method to calculate sin(Nx) and cos(Nx) from cos(x), sin((N-1)x), cos((N-1)x), sin((N-2)x), and cos((N-2)x)
// https://trans4mind.com/personal_development/mathematics/trigonometry/multipleAnglesRecursiveFormula.htm
pub fn chebyshev(cos: I1F31, sin1: I1F31, cos1: I1F31, sin2: I1F31, cos2: I1F31) -> (I1F31, I1F31) {
    let cosn = (cos * cos1).wrapping_mul_int(2).wrapping_sub(cos2);
    let sinn = (cos * sin1).wrapping_mul_int(2).wrapping_sub(sin2);
    (sinn, cosn)
}

#[cfg(test)]
mod tests {

    use super::*;
    use approx::assert_abs_diff_eq;

    use std::f64::consts::PI;

    #[test]
    fn test_shift_left() {
        let angle: f64 = 0.2;
        let (sin, cos) = sin_cos(I0F32::from_num(angle));
        let (sin_shifted, cos_shifted) = shift_left_120(sin, cos);

        assert_abs_diff_eq!(
            f64::from(sin_shifted),
            (2.0 * PI * angle - 2.0 * PI / 3.0).sin(),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            f64::from(cos_shifted),
            (2.0 * PI * angle - 2.0 * PI / 3.0).cos(),
            epsilon = 0.0001
        );
    }

    #[test]
    fn test_shift_right() {
        let angle: f64 = 0.2;
        let (sin, cos) = sin_cos(I0F32::from_num(angle));
        let (sin_shifted, cos_shifted) = shift_right_120(sin, cos);

        assert_abs_diff_eq!(
            f64::from(sin_shifted),
            (2.0 * PI * angle + 2.0 * PI / 3.0).sin(),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            f64::from(cos_shifted),
            (2.0 * PI * angle + 2.0 * PI / 3.0).cos(),
            epsilon = 0.0001
        );
    }

    #[test]
    fn test_chebyshev() {
        let angle: f64 = 0.2;
        let (sin0, cos0) = (I1F31::ZERO, I1F31::MAX);
        let (sin1, cos1) = sin_cos(I0F32::from_num(angle));
        let (sin2, cos2) = chebyshev(cos1, sin1, cos1, sin0, cos0);
        let (sin3, cos3) = chebyshev(cos1, sin2, cos2, sin1, cos1);

        assert_abs_diff_eq!(
            f64::from(sin2),
            (2.0 * 2.0 * PI * angle).sin(),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            f64::from(cos2),
            (2.0 * 2.0 * PI * angle).cos(),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            f64::from(sin3),
            (3.0 * 2.0 * PI * angle).sin(),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            f64::from(cos3),
            (3.0 * 2.0 * PI * angle).cos(),
            epsilon = 0.0001
        );
    }
}
