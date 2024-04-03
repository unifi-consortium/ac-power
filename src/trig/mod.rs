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

/*!
This module contains trigometric data-types, methods, and functions useful for ac power signal processing.  The sin, cos, and arctan functions are wrappers around the [idsp](https://crates.io/crates/idsp) crate implementations, which are implemented with optimized fixed-point arithmetic for resource constrained platforms (i.e. microcontrollers).

# Newtypes

This module defines three new data types which are wrappers around i32 and f32 prmitives

1. [struct Theta(i32)](crate::trig::Theta) - Representation of a phase between -180 and + 180 degres (-pi to + pi)
2. [struct Sin(f32)](crate::trig::Sin)- Representation of a sin(theta)
3. [struct Cos(f32)](crate::trig::Cos) - Representation of a cos(theta)

Each newtype contains constructors for instantiating on instance from a variable

```rust
use ac_power::trig::{Theta, Sin, Cos};

let theta = Theta::from_degrees(90.0);
let theta = Theta::from_radians(core::f32::consts::PI/2.0);
let sin = Sin::from_degrees(90.0);
let cos = Cos::from_radians(core::f32::consts::PI/2.0);
```

They also include conversions for easily converting to and from the native data types they wrap.

```rust
use ac_power::trig::{Theta, Sin, Cos};

let theta: Theta = 536870912.into();
let sin = Sin::from(0.6);
let sin_as_float: f32 = sin.into();
```

`Sin` and `Cos` types support multiplication to other numeric types, always returning the other type.

```rust
use ac_power::trig::Sin;
use ac_power::newtypes::Voltage;
use approx::assert_abs_diff_eq;

let sin = Sin::from_degrees(45.0);
let v: Voltage = Voltage::from(1.0) * sin;
assert_abs_diff_eq!(f32::from(v), 0.707, epsilon = 0.0001);
```

The `Theta` data-type supports a wrapping add assign.

```rust
use ac_power::trig::Theta;
use approx::assert_abs_diff_eq;

let mut theta = Theta::from_degrees(179.0);
theta += Theta::from_degrees(2.0);
assert_abs_diff_eq!(theta.to_degrees(), -179.0, epsilon = 0.0001);
```

# Functions

The trig modules contains 5 functions which are useful for ac power processing

1. [cos_sin] - Calculate cos and sin from theta simultaneously.
2. [rotate] - Function for rotating a vector using Ptolemy's theorem
3. [shift_right_120] - Function for rotating a vector clockwise by 120 degrees
4. [shift_left_120] - Function for rotating a vector counter-clockwise by 120 degrees
5. [chebyshev] - calculate sin(Nx) and cos(Nx) using chebyshev method

*/

mod types;
use core::ops::{Add, Mul, Sub};
pub use types::{Cos, Sin, Theta};

use crate::constants::{ONE_HALF, SQRT_3_OVER_2};

use idsp;

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
pub fn cos_sin(theta: Theta) -> (Cos, Sin) {
    // use the idsp library cos/sin function
    let (cos, sin) = idsp::cossin(theta.into());

    // convert to the Cos and Sin newtypes
    (cos.into(), sin.into())
}

/// Use Ptolemy's theorem to rotate a vector
///
/// # Examples
///
/// ```
/// use ac_power::trig::{rotate, Theta, cos_sin};
///
/// let (x, y) = (1.0, 0.0);
/// let theta = Theta::from_degrees(90.0);
/// let (cos, sin) = cos_sin(theta);
/// let (xr, yr) = rotate(x, y, cos, sin);
/// ```
pub fn rotate<
    T: Copy + Mul<Sin, Output = T> + Mul<Cos, Output = T> + Sub<Output = T> + Add<Output = T>,
>(
    x: T,
    y: T,
    cos: Cos,
    sin: Sin,
) -> (T, T) {
    let xr = x * cos - y * sin;
    let yr = x * sin + y * cos;

    (xr, yr)
}

/// Shifts a cos/sin pair 120 degrees right (+2pi/3)
///
/// # Examples
///
/// ```
/// use ac_power::trig::{shift_right_120, Theta, cos_sin};
///
/// let (x, y) = (1.0, 0.0);
/// let theta = Theta::from_degrees(90.0);
/// let (cos, sin) = cos_sin(theta);
/// let (cosr, sinr) = shift_right_120(cos, sin);
/// ```
pub fn shift_right_120(cos: Cos, sin: Sin) -> (Cos, Sin) {
    let (cosr, sinr) = rotate(
        f32::from(cos),
        f32::from(sin),
        (-ONE_HALF).into(),
        SQRT_3_OVER_2.into(),
    );
    (cosr.into(), sinr.into())
}

/// Shifts a cos/sin pair 120 degrees left (-2pi/3)
///
/// # Examples
///
/// ```
/// use ac_power::trig::{shift_left_120, Theta, cos_sin};
///
/// let (x, y) = (1.0, 0.0);
/// let theta = Theta::from_degrees(90.0);
/// let (cos, sin) = cos_sin(theta);
/// let (cosr, sinr) = shift_left_120(cos, sin);
/// ```
pub fn shift_left_120(cos: Cos, sin: Sin) -> (Cos, Sin) {
    let (cosr, sinr) = rotate(
        f32::from(cos),
        f32::from(sin),
        (-ONE_HALF).into(),
        (-SQRT_3_OVER_2).into(),
    );
    (cosr.into(), sinr.into())
}

/// Use [chebyshev method](https://trans4mind.com/personal_development/mathematics/trigonometry/multipleAnglesRecursiveFormula.htm) to calculate sin(Nx) and cos(Nx) from cos(x), sin((N-1)x), cos((N-1)x), sin((N-2)x), and cos((N-2)x)
///
/// # Examples
///
/// ```
/// use ac_power::trig::{chebyshev, Theta, cos_sin, Sin, Cos};
///
/// let radians: f32 = 1.2;
/// let theta = Theta::from_radians(radians);
/// let (cos0, sin0) = (Cos::from(1.0), Sin::from(0.0));
/// let (cos1, sin1) = cos_sin(theta);
/// let (cos2, sin2) = chebyshev(cos1, sin1, cos1, sin0, cos0);
/// let (cos3, sin3) = chebyshev(cos1, sin2, cos2, sin1, cos1);
/// ```
pub fn chebyshev(cos: Cos, sin1: Sin, cos1: Cos, sin2: Sin, cos2: Cos) -> (Cos, Sin) {
    let cosn = 2.0 * (cos * cos1) - f32::from(cos2);
    let sinn = 2.0 * (cos * sin1) - f32::from(sin2);
    (cosn.into(), sinn.into())
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

        assert_abs_diff_eq!(f32::from(sin), radians.sin(), epsilon = 0.0001);
        assert_abs_diff_eq!(f32::from(cos), radians.cos(), epsilon = 0.0001);
    }

    #[test]
    fn shift_left() {
        let radians: f32 = 1.2;
        let theta = Theta::from_radians(radians);
        let (cos, sin) = cos_sin(theta);
        let (cos_shifted, sin_shifted) = shift_left_120(cos, sin);

        assert_abs_diff_eq!(
            f32::from(sin_shifted),
            (radians - 2.0 * PI / 3.0).sin(),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            f32::from(cos_shifted),
            (radians - 2.0 * PI / 3.0).cos(),
            epsilon = 0.0001
        );
    }

    #[test]
    fn shift_right() {
        let radians: f32 = 1.2;
        let theta = Theta::from_radians(radians);
        let (cos, sin) = cos_sin(theta);
        let (cos_shifted, sin_shifted) = shift_right_120(cos, sin);

        assert_abs_diff_eq!(
            f32::from(sin_shifted),
            (radians + 2.0 * PI / 3.0).sin(),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            f32::from(cos_shifted),
            (radians + 2.0 * PI / 3.0).cos(),
            epsilon = 0.0001
        );
    }

    #[test]
    fn chebyshev_works() {
        let radians: f32 = 1.2;
        let theta = Theta::from_radians(radians);
        let (cos0, sin0) = (Cos::from(1.0), Sin::from(0.0));
        let (cos1, sin1) = cos_sin(theta);
        let (cos2, sin2) = chebyshev(cos1, sin1, cos1, sin0, cos0);
        let (cos3, sin3) = chebyshev(cos1, sin2, cos2, sin1, cos1);

        assert_abs_diff_eq!(f32::from(sin2), (2.0 * radians).sin(), epsilon = 0.0001);
        assert_abs_diff_eq!(f32::from(cos2), (2.0 * radians).cos(), epsilon = 0.0001);
        assert_abs_diff_eq!(f32::from(sin3), (3.0 * radians).sin(), epsilon = 0.0001);
        assert_abs_diff_eq!(f32::from(cos3), (3.0 * radians).cos(), epsilon = 0.0001);
    }
}
