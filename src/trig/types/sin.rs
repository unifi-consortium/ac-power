use crate::impl_f32_ops;
use crate::trig::cos_sin;
use crate::trig::Cos;
use crate::trig::Theta;
use core::convert::From;
use core::ops::{Mul, Neg};
use derive_more::{From, Into};

#[derive(Debug, Copy, Clone, PartialEq, From, Into)]
pub struct Sin(f32);

impl Sin {
    /// Calculates sin from theta
    ///
    /// # Examples
    ///
    /// ```
    /// use ac_power::trig::{Theta, Sin};
    ///
    /// let theta = Theta::from_degrees(180.0);
    /// let sin = Sin::from_theta(theta);
    /// ```
    pub fn from_theta(theta: Theta) -> Self {
        let (_, sin) = cos_sin(theta);
        sin
    }

    /// Calculates sin from degrees
    ///
    /// # Examples
    ///
    /// ```
    /// use ac_power::trig::Sin;
    ///
    /// let sin = Sin::from_degrees(45.0);
    /// ```
    pub fn from_degrees(degrees: f32) -> Self {
        let theta = Theta::from_degrees(degrees);
        Self::from_theta(theta)
    }

    /// Calculates sin from radians
    ///
    /// # Examples
    ///
    /// ```
    /// use ac_power::trig::Sin;
    ///
    /// let sin = Sin::from_radians(core::f32::consts::PI/4.0);
    /// ```
    pub fn from_radians(radians: f32) -> Self {
        let theta = Theta::from_radians(radians);
        Self::from_theta(theta)
    }
}

// use macro to support most of the arithmetic
impl_f32_ops!(Sin);

impl Neg for Sin {
    fn neg(self) -> Self {
        Self(-self.0)
    }
    type Output = Self;
}

impl From<i32> for Sin {
    fn from(item: i32) -> Self {
        Self((item as f32) / 2147483648.)
    }
}

impl Mul<Sin> for Sin {
    fn mul(self, other: Sin) -> f32 {
        self.0 * other.0
    }
    type Output = f32;
}

impl Mul<Cos> for Sin {
    fn mul(self, rhs: Cos) -> f32 {
        self.0 * rhs
    }
    type Output = f32;
}

#[cfg(test)]
mod tests {

    use super::*;
    use approx::assert_abs_diff_eq;

    use std::f32::consts::PI;

    fn linspace(x0: f32, x1: f32, length: usize) -> Vec<f32> {
        let dx = (x1 - x0) / ((length - 1) as f32);
        let mut xs: Vec<f32> = vec![x0];
        for index in 1..length {
            xs.push(xs[index - 1] + dx);
        }
        xs
    }

    fn degrees_to_radians(degrees: f32) -> f32 {
        2.0 * PI * (degrees / 360.0)
    }

    fn check_from_degrees(degrees: f32) {
        let sin = Sin::from_degrees(degrees);
        let radians = degrees_to_radians(degrees);
        assert_abs_diff_eq!(f32::from(sin), radians.sin(), epsilon = 0.0001);
    }

    fn check_from_radians(radians: f32) {
        let sin = Sin::from_radians(radians);
        assert_abs_diff_eq!(f32::from(sin), radians.sin(), epsilon = 0.0001);
    }

    #[test]
    fn constructors() {
        let angles = linspace(-720.0, 720.0, 500);
        for angle in angles.iter() {
            check_from_degrees(*angle);
        }

        let angles = linspace(-4.0 * PI, 4.0 * PI, 100);
        for angle in angles.iter() {
            check_from_radians(*angle);
        }
    }

    #[test]
    fn conversions() {
        let x = 0.707;
        let sin = Sin::from(x);
        assert_abs_diff_eq!(x, f32::from(sin), epsilon = 0.0001);
    }

    #[test]
    fn arithmetic() {
        let radians = 0.26;
        let sin = Sin::from_radians(radians);
        let x = 1.0;
        assert_abs_diff_eq!(x * sin, x * radians.sin(), epsilon = 0.0001);
        assert_abs_diff_eq!(sin * x, x * radians.sin(), epsilon = 0.0001);
        assert_abs_diff_eq!(sin * sin, radians.sin() * radians.sin(), epsilon = 0.0001);
    }
}
