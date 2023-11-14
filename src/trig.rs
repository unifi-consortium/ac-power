use crate::constants::{ONE_HALF, SQRT_3_OVER_2};
use fixed::types::{I0F32, I1F31};
use fixed::FixedI32;
use idsp::cossin;

pub fn cos_sin(theta: I0F32) -> (I1F31, I1F31) {
    // use the idsp library cos/sin function
    let (cos, sin) = cossin(theta.to_bits());

    // convert the result to fixed datatype
    (I1F31::from_bits(cos), I1F31::from_bits(sin))
}

// Use Ptolemy's theorem to rotate a vector
pub fn rotate<const FRAC: i32>(
    x: FixedI32<FRAC>,
    y: FixedI32<FRAC>,
    cos: I1F31,
    sin: I1F31,
) -> (FixedI32<FRAC>, FixedI32<FRAC>) {
    let mut xr = x;
    xr *= cos;
    xr.mul_acc(y, -sin);

    let mut yr = y;
    yr *= cos;
    yr.mul_acc(x, sin);

    (xr, yr)
}

/// Shifts sin/cos values 120 degrees right (+2pi/3)
pub fn shift_right_120(cos: I1F31, sin: I1F31) -> (I1F31, I1F31) {
    rotate(cos, sin, -ONE_HALF, SQRT_3_OVER_2)
}

/// Shifts sin/cos values 120 degrees left (-2pi/3)
pub fn shift_left_120(cos: I1F31, sin: I1F31) -> (I1F31, I1F31) {
    rotate(cos, sin, -ONE_HALF, -SQRT_3_OVER_2)
}

// use chebyshev method to calculate sin(Nx) and cos(Nx) from cos(x), sin((N-1)x), cos((N-1)x), sin((N-2)x), and cos((N-2)x)
// https://trans4mind.com/personal_development/mathematics/trigonometry/multipleAnglesRecursiveFormula.htm
pub fn chebyshev(cos: I1F31, sin1: I1F31, cos1: I1F31, sin2: I1F31, cos2: I1F31) -> (I1F31, I1F31) {
    let cosn = (cos * cos1).wrapping_mul_int(2).wrapping_sub(cos2);
    let sinn = (cos * sin1).wrapping_mul_int(2).wrapping_sub(sin2);
    (cosn, sinn)
}

#[cfg(test)]
mod tests {

    use super::*;
    use approx::assert_abs_diff_eq;

    use std::f64::consts::PI;

    #[test]
    fn test_shift_left() {
        let angle: f64 = 0.2;
        let (cos, sin) = cos_sin(I0F32::from_num(angle));
        let (cos_shifted, sin_shifted) = shift_left_120(cos, sin);

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
        let (cos, sin) = cos_sin(I0F32::from_num(angle));
        let (cos_shifted, sin_shifted) = shift_right_120(cos, sin);

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
        let (cos0, sin0) = (I1F31::MAX, I1F31::ZERO);
        let (cos1, sin1) = cos_sin(I0F32::from_num(angle));
        let (cos2, sin2) = chebyshev(cos1, sin1, cos1, sin0, cos0);
        let (cos3, sin3) = chebyshev(cos1, sin2, cos2, sin1, cos1);

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
