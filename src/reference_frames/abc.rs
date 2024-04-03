use crate::constants::{ONE_HALF, SQRT_3_OVER_2};
use crate::trig::{cos_sin, Sin, Theta};
use core::ops::{Add, AddAssign, Mul, Sub, SubAssign};

/// Basic stationary reference frame (i.e. the instantaneous signals)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Abc<T> {
    pub a: T,
    pub b: T,
    pub c: T,
}

impl<T: Add<Output = T> + Copy> AddAssign<Abc<T>> for Abc<T> {
    fn add_assign(&mut self, rhs: Abc<T>) {
        *self = *self + rhs;
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign<Abc<T>> for Abc<T> {
    fn sub_assign(&mut self, rhs: Abc<T>) {
        *self = *self - rhs;
    }
}

impl<T: Add<Output = T>> Add<Abc<T>> for Abc<T> {
    fn add(self, other: Abc<T>) -> Abc<T> {
        let a = self.a + other.a;
        let b = self.b + other.b;
        let c = self.c + other.c;
        Self { a, b, c }
    }
    type Output = Abc<T>;
}

impl<T: Add<Output = T> + Copy> Add<T> for Abc<T> {
    fn add(self, other: T) -> Abc<T> {
        let a = self.a + other;
        let b = self.b + other;
        let c = self.c + other;
        Self { a, b, c }
    }
    type Output = Abc<T>;
}

impl<T: Sub<Output = T>> Sub<Abc<T>> for Abc<T> {
    fn sub(self, other: Abc<T>) -> Abc<T> {
        let a = self.a - other.a;
        let b = self.b - other.b;
        let c = self.c - other.c;
        Self { a, b, c }
    }
    type Output = Abc<T>;
}

impl<T: Sub<Output = T> + Copy> Sub<T> for Abc<T> {
    fn sub(self, other: T) -> Abc<T> {
        let a = self.a - other;
        let b = self.b - other;
        let c = self.c - other;
        Self { a, b, c }
    }
    type Output = Abc<T>;
}

impl<T: Mul<f32, Output = T> + Mul<Sin, Output = T> + Copy + From<f32>> Abc<T> {
    pub fn zero() -> Self {
        Self {
            a: 0.0.into(),
            b: 0.0.into(),
            c: 0.0.into(),
        }
    }

    pub fn from_polar(amplitude: T, theta: Theta) -> Self {
        let (cos, sin) = cos_sin(theta);
        let sin_m = -cos * SQRT_3_OVER_2 - sin * ONE_HALF;
        let sin_p = cos * SQRT_3_OVER_2 - sin * ONE_HALF;

        let a = amplitude * sin;
        let b = amplitude * sin_m;
        let c = amplitude * sin_p;

        Self { a, b, c }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn addition() {
        let abc_pos = Abc {
            a: 200.0,
            b: -100.0,
            c: -150.0,
        };
        let abc_neg = Abc {
            a: 100.0,
            b: -150.0,
            c: -75.0,
        };

        let abc = abc_pos + abc_neg;
        let expected = Abc {
            a: abc_pos.a + abc_neg.a,
            b: abc_pos.b + abc_neg.b,
            c: abc_pos.c + abc_neg.c,
        };

        assert_eq!(abc, expected);
    }

    #[test]
    fn subtraction() {
        let abc_pos = Abc {
            a: 200.0,
            b: -100.0,
            c: -150.0,
        };
        let abc_neg = Abc {
            a: 100.0,
            b: -150.0,
            c: -75.0,
        };

        let abc = abc_pos - abc_neg;
        let expected = Abc {
            a: abc_pos.a - abc_neg.a,
            b: abc_pos.b - abc_neg.b,
            c: abc_pos.c - abc_neg.c,
        };

        assert_eq!(abc, expected);
    }

    #[test]
    fn add_bias() {
        let abc_pos = Abc {
            a: 200.0,
            b: -100.0,
            c: -150.0,
        };

        let abc = abc_pos + 200.0;
        let expected = Abc {
            a: abc_pos.a + 200.0,
            b: abc_pos.b + 200.0,
            c: abc_pos.c + 200.0,
        };

        assert_eq!(abc, expected);
    }

    #[test]
    fn subtract_bias() {
        let abc_pos = Abc {
            a: 200.0,
            b: -100.0,
            c: -150.0,
        };

        let abc = abc_pos - 200.0;
        let expected = Abc {
            a: abc_pos.a - 200.0,
            b: abc_pos.b - 200.0,
            c: abc_pos.c - 200.0,
        };

        assert_eq!(abc, expected);
    }
}
