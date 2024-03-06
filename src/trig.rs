use crate::constants::{ONE_HALF, SQRT_3_OVER_2};
use idsp::cossin;

pub fn cos_sin(theta: i32) -> (f32, f32) {
    // use the idsp library cos/sin function
    let (cos, sin) = cossin(theta);

    // convert the result to floating point
    ((cos as f32) / 2147483648., (sin as f32) / 2147483648.)
}

// Use Ptolemy's theorem to rotate a vector
pub fn rotate(x: f32, y: f32, cos: f32, sin: f32) -> (f32, f32) {
    let xr = x * cos - y * sin;
    let yr = x * sin + y * cos;

    (xr, yr)
}

/// Shifts sin/cos values 120 degrees right (+2pi/3)
pub fn shift_right_120(cos: f32, sin: f32) -> (f32, f32) {
    rotate(cos, sin, -ONE_HALF, SQRT_3_OVER_2)
}

/// Shifts sin/cos values 120 degrees left (-2pi/3)
pub fn shift_left_120(cos: f32, sin: f32) -> (f32, f32) {
    rotate(cos, sin, -ONE_HALF, -SQRT_3_OVER_2)
}

// use chebyshev method to calculate sin(Nx) and cos(Nx) from cos(x), sin((N-1)x), cos((N-1)x), sin((N-2)x), and cos((N-2)x)
// https://trans4mind.com/personal_development/mathematics/trigonometry/multipleAnglesRecursiveFormula.htm
pub fn chebyshev(cos: f32, sin1: f32, cos1: f32, sin2: f32, cos2: f32) -> (f32, f32) {
    let cosn = 2.0 * (cos * cos1) - cos2;
    let sinn = 2.0 * (cos * sin1) - sin2;
    (cosn, sinn)
}

#[cfg(test)]
mod tests {

    use super::*;
    use approx::assert_abs_diff_eq;

    use std::f32::consts::PI;

    #[test]
    fn test_cos_sin() {
        let angle = 0.2;
        let (cos, sin) = cos_sin((angle * 4294967296.) as i32);

        assert_abs_diff_eq!(sin, (2.0 * PI * angle).sin(), epsilon = 0.0001);
        assert_abs_diff_eq!(cos, (2.0 * PI * angle).cos(), epsilon = 0.0001);
    }

    #[test]
    fn test_shift_left() {
        let angle = 0.2;
        let (cos, sin) = cos_sin((angle * 4294967296.) as i32);
        let (cos_shifted, sin_shifted) = shift_left_120(cos, sin);

        assert_abs_diff_eq!(
            sin_shifted,
            (2.0 * PI * angle - 2.0 * PI / 3.0).sin(),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            cos_shifted,
            (2.0 * PI * angle - 2.0 * PI / 3.0).cos(),
            epsilon = 0.0001
        );
    }

    #[test]
    fn test_shift_right() {
        let angle = 0.2;
        let (cos, sin) = cos_sin((angle * 4294967296.) as i32);
        let (cos_shifted, sin_shifted) = shift_right_120(cos, sin);

        assert_abs_diff_eq!(
            sin_shifted,
            (2.0 * PI * angle + 2.0 * PI / 3.0).sin(),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            cos_shifted,
            (2.0 * PI * angle + 2.0 * PI / 3.0).cos(),
            epsilon = 0.0001
        );
    }

    #[test]
    fn test_chebyshev() {
        let angle = 0.2;
        let (cos0, sin0) = (1.0, 0.0);
        let (cos1, sin1) = cos_sin((angle * 4294967296.) as i32);
        let (cos2, sin2) = chebyshev(cos1, sin1, cos1, sin0, cos0);
        let (cos3, sin3) = chebyshev(cos1, sin2, cos2, sin1, cos1);

        assert_abs_diff_eq!(sin2, (2.0 * 2.0 * PI * angle).sin(), epsilon = 0.0001);
        assert_abs_diff_eq!(cos2, (2.0 * 2.0 * PI * angle).cos(), epsilon = 0.0001);
        assert_abs_diff_eq!(sin3, (3.0 * 2.0 * PI * angle).sin(), epsilon = 0.0001);
        assert_abs_diff_eq!(cos3, (3.0 * 2.0 * PI * angle).cos(), epsilon = 0.0001);
    }
}
