use crate::constants::{ONE_THIRD, SQRT_3_OVER_3, TWO_THIRDS};
use crate::reference_frames::{Abc, AlphaBeta, AlphaBeta0, Dq, Dq0, Polar};
use crate::trig::{cos_sin, shift_left_120, shift_right_120};

impl From<Polar> for Abc {
    fn from(polar: Polar) -> Self {
        let (cos, sin) = cos_sin(polar.theta);
        let (_, sin_m) = shift_left_120(cos, sin);
        let (_, sin_p) = shift_right_120(cos, sin);

        let mut a = polar.amplitude;
        let mut b = polar.amplitude;
        let mut c = polar.amplitude;
        a *= sin;
        b *= sin_m;
        c *= sin_p;
        Self { a, b, c }
    }
}

impl From<Abc> for AlphaBeta {
    fn from(abc: Abc) -> Self {
        let alpha = (abc.a * TWO_THIRDS) - (abc.b * ONE_THIRD) - (abc.c * ONE_THIRD);
        let beta = (abc.b * SQRT_3_OVER_3) - (abc.c * SQRT_3_OVER_3);

        Self { alpha, beta }
    }
}

impl From<Abc> for AlphaBeta0 {
    fn from(abc: Abc) -> Self {
        let alpha = (abc.a * TWO_THIRDS) - (abc.b * ONE_THIRD) - (abc.c * ONE_THIRD);
        let beta = (abc.b * SQRT_3_OVER_3) - (abc.c * SQRT_3_OVER_3);
        let zero = ONE_THIRD * (abc.a + abc.b + abc.c);

        Self { alpha, beta, zero }
    }
}

impl AlphaBeta {
    // DQ0 Transform
    pub fn to_dq0(&self, cos: f32, sin: f32) -> Dq0 {
        let d = (self.alpha * sin) - (self.beta * cos);
        let q = (self.alpha * cos) + (self.beta * sin);

        Dq0 { d, q, zero: 0.0 }
    }

    pub fn to_dq(&self, cos: f32, sin: f32) -> Dq {
        let d = (self.alpha * sin) - (self.beta * cos);
        let q = (self.alpha * cos) + (self.beta * sin);

        Dq { d, q }
    }
}

impl Abc {
    pub fn to_dq(&self, cos: f32, sin: f32) -> Dq {
        /* sin and cos with 120 degree offsets */
        let (cos_m, sin_m) = shift_left_120(cos, sin);
        let (cos_p, sin_p) = shift_right_120(cos, sin);

        let d = TWO_THIRDS * ((self.a * sin) + (self.b * sin_m) + (self.b * sin_p));
        let q = TWO_THIRDS * ((self.a * cos) + (self.b * cos_m) + (self.b * cos_p));

        Dq { d, q }
    }

    pub fn to_dq0(&self, cos: f32, sin: f32) -> Dq0 {
        /* sin and cos with 120 degree offsets */
        let (cos_m, sin_m) = shift_left_120(cos, sin);
        let (cos_p, sin_p) = shift_right_120(cos, sin);

        let d = TWO_THIRDS * ((self.a * sin) + (self.b * sin_m) + (self.b * sin_p));
        let q = TWO_THIRDS * ((self.a * cos) + (self.b * cos_m) + (self.b * cos_p));

        Dq0 {
            d,
            q,
            zero: (*self).into(),
        }
    }
}

impl From<Abc> for f32 {
    fn from(abc: Abc) -> Self {
        ONE_THIRD * (abc.a + abc.b + abc.c)
    }
}

impl Dq {
    pub fn to_abc(&self, cos: f32, sin: f32) -> Abc {
        /* sin and cos with 120 degree offsets */
        let (cos_m, sin_m) = shift_left_120(cos, sin);
        let (cos_p, sin_p) = shift_right_120(cos, sin);

        let a = (self.d * sin) + (self.q * cos);
        let b = (self.d * sin_m) + (self.q * cos_m);
        let c = (self.d * sin_p) + (self.q * cos_p);

        Abc { a, b, c }
    }
}

impl Dq0 {
    pub fn to_abc(&self, cos: f32, sin: f32) -> Abc {
        Dq {
            d: self.d,
            q: self.q,
        }
        .to_abc(cos, sin)
            + self.zero
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::trig::Theta;

    #[test]
    fn clarke_transform() {
        let degrees = 20. / 360.;
        let theta = Theta::from_degrees(degrees);
        let polar = Polar {
            theta,
            amplitude: 480.0,
        };
        let abc = Abc::from(polar);

        let _alpha_beta_zero = AlphaBeta0::from(abc);

        // FIXME:  Need a check here
    }

    #[test]
    fn dq0_transform() {
        let degrees = 20. / 360.;
        let theta = Theta::from_degrees(degrees);
        let polar = Polar {
            theta,
            amplitude: 480.0,
        };
        let abc = Abc::from(polar);

        let (cos, sin) = cos_sin(theta);
        let dq0 = abc.to_dq0(cos, sin);

        println!("{:?}", dq0);
        // // we loose a little precision in the transform
        // // I think most of this is in the sin/cos shifts
        // // TODO:  Can we make this better?
        // assert_eq!(dq0.d, I11F21::from_num(479.999999));
        // assert_eq!(dq0.q, I11F21::from_num(0.0));
        // assert_eq!(dq0.z, I11F21::from_num(-0.000001));
    }
}
