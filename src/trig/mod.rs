// Copyright 2023 Enphase Energy, Inc and Universal Interoperability for
// Grid-Forming Inverters (UNIFI) Consortium.
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

mod types;
pub use types::Theta;

use crate::constants::{ONE_HALF, SQRT_3_OVER_2};

use idsp::cossin;

/// Calculates sin and cos from theta
///
/// # Examples
///
/// ```
/// use ac_power::trig::{Theta, cos_sin};
///
/// let theta = Theta::from_degrees(180.0);
/// let (cos, sin) = cos_sin(theta);
/// ```
pub fn cos_sin(theta: Theta) -> (f32, f32) {
    // use the idsp library cos/sin function
    let (cos, sin) = cossin(theta.0);

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
        let radians: f32 = 1.2;
        let theta = Theta::from_radians(radians);
        let (cos, sin) = cos_sin(theta);

        assert_abs_diff_eq!(sin, radians.sin(), epsilon = 0.0001);
        assert_abs_diff_eq!(cos, radians.cos(), epsilon = 0.0001);
    }

    #[test]
    fn test_shift_left() {
        let radians: f32 = 1.2;
        let theta = Theta::from_radians(radians);
        let (cos, sin) = cos_sin(theta);
        let (cos_shifted, sin_shifted) = shift_left_120(cos, sin);

        assert_abs_diff_eq!(
            sin_shifted,
            (radians - 2.0 * PI / 3.0).sin(),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            cos_shifted,
            (radians - 2.0 * PI / 3.0).cos(),
            epsilon = 0.0001
        );
    }

    #[test]
    fn test_shift_right() {
        let radians: f32 = 1.2;
        let theta = Theta::from_radians(radians);
        let (cos, sin) = cos_sin(theta);
        let (cos_shifted, sin_shifted) = shift_right_120(cos, sin);

        assert_abs_diff_eq!(
            sin_shifted,
            (radians + 2.0 * PI / 3.0).sin(),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            cos_shifted,
            (radians + 2.0 * PI / 3.0).cos(),
            epsilon = 0.0001
        );
    }

    #[test]
    fn test_chebyshev() {
        let radians: f32 = 1.2;
        let theta = Theta::from_radians(radians);
        let (cos0, sin0) = (1.0, 0.0);
        let (cos1, sin1) = cos_sin(theta);
        let (cos2, sin2) = chebyshev(cos1, sin1, cos1, sin0, cos0);
        let (cos3, sin3) = chebyshev(cos1, sin2, cos2, sin1, cos1);

        assert_abs_diff_eq!(sin2, (2.0 * radians).sin(), epsilon = 0.0001);
        assert_abs_diff_eq!(cos2, (2.0 * radians).cos(), epsilon = 0.0001);
        assert_abs_diff_eq!(sin3, (3.0 * radians).sin(), epsilon = 0.0001);
        assert_abs_diff_eq!(cos3, (3.0 * radians).cos(), epsilon = 0.0001);
    }
}
