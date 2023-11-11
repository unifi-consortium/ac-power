use core::ops::{Add, Sub};
use fixed::FixedI32;

// Unbalanced reference frames
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Abc<const FRAC: i32> {
    pub a: FixedI32<FRAC>,
    pub b: FixedI32<FRAC>,
    pub c: FixedI32<FRAC>,
}

impl<const FRAC: i32> Add<Abc<FRAC>> for Abc<FRAC> {
    fn add(self, other: Abc<FRAC>) -> Abc<FRAC> {
        let a = self.a + other.a;
        let b = self.b + other.b;
        let c = self.c + other.c;
        Self { a, b, c }
    }
    type Output = Abc<FRAC>;
}

impl<const FRAC: i32> Add<FixedI32<FRAC>> for Abc<FRAC> {
    fn add(self, other: FixedI32<FRAC>) -> Abc<FRAC> {
        let a = self.a + other;
        let b = self.b + other;
        let c = self.c + other;
        Self { a, b, c }
    }
    type Output = Abc<FRAC>;
}

impl<const FRAC: i32> Sub<Abc<FRAC>> for Abc<FRAC> {
    fn sub(self, other: Abc<FRAC>) -> Abc<FRAC> {
        let a = self.a - other.a;
        let b = self.b - other.b;
        let c = self.c - other.c;
        Self { a, b, c }
    }
    type Output = Abc<FRAC>;
}

impl<const FRAC: i32> Sub<FixedI32<FRAC>> for Abc<FRAC> {
    fn sub(self, other: FixedI32<FRAC>) -> Abc<FRAC> {
        let a = self.a - other;
        let b = self.b - other;
        let c = self.c - other;
        Self { a, b, c }
    }
    type Output = Abc<FRAC>;
}

#[cfg(test)]
mod tests {

    use super::*;
    use fixed::types::I11F21;

    #[test]
    fn addition() {
        let abc_pos = Abc {
            a: I11F21::from_num(200),
            b: I11F21::from_num(-100),
            c: I11F21::from_num(-150),
        };
        let abc_neg = Abc {
            a: I11F21::from_num(100),
            b: I11F21::from_num(-150),
            c: I11F21::from_num(-75),
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
            a: I11F21::from_num(200),
            b: I11F21::from_num(-100),
            c: I11F21::from_num(-150),
        };
        let abc_neg = Abc {
            a: I11F21::from_num(100),
            b: I11F21::from_num(-150),
            c: I11F21::from_num(-75),
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
            a: I11F21::from_num(200),
            b: I11F21::from_num(-100),
            c: I11F21::from_num(-150),
        };

        let abc = abc_pos + I11F21::from_num(200);
        let expected = Abc {
            a: abc_pos.a + I11F21::from_num(200),
            b: abc_pos.b + I11F21::from_num(200),
            c: abc_pos.c + I11F21::from_num(200),
        };

        assert_eq!(abc, expected);
    }

    #[test]
    fn subtract_bias() {
        let abc_pos = Abc {
            a: I11F21::from_num(200),
            b: I11F21::from_num(-100),
            c: I11F21::from_num(-150),
        };

        let abc = abc_pos - I11F21::from_num(200);
        let expected = Abc {
            a: abc_pos.a - I11F21::from_num(200),
            b: abc_pos.b - I11F21::from_num(200),
            c: abc_pos.c - I11F21::from_num(200),
        };

        assert_eq!(abc, expected);
    }
}
