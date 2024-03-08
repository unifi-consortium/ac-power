use core::ops::{Add, Sub};

// Unbalanced reference frames
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Abc {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl Add<Abc> for Abc {
    fn add(self, other: Abc) -> Abc {
        let a = self.a + other.a;
        let b = self.b + other.b;
        let c = self.c + other.c;
        Self { a, b, c }
    }
    type Output = Abc;
}

impl Add<f32> for Abc {
    fn add(self, other: f32) -> Abc {
        let a = self.a + other;
        let b = self.b + other;
        let c = self.c + other;
        Self { a, b, c }
    }
    type Output = Abc;
}

impl Sub<Abc> for Abc {
    fn sub(self, other: Abc) -> Abc {
        let a = self.a - other.a;
        let b = self.b - other.b;
        let c = self.c - other.c;
        Self { a, b, c }
    }
    type Output = Abc;
}

impl Sub<f32> for Abc {
    fn sub(self, other: f32) -> Abc {
        let a = self.a - other;
        let b = self.b - other;
        let c = self.c - other;
        Self { a, b, c }
    }
    type Output = Abc;
}

impl Abc {
    pub const ZERO: Abc = Abc {
        a: 0.0,
        b: 0.0,
        c: 0.0,
    };
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
