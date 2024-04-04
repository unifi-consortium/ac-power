use crate::constants::FRAC_1_SQRT_3;
use crate::newtypes::{Current, Voltage};
use crate::pq::Pq;
use crate::reference_frames::{Abc, AlphaBeta, AlphaBeta0, Dq, Dq0};
use core::ops::Mul;

impl Mul<Abc<Current>> for Abc<Voltage> {
    fn mul(self, rhs: Abc<Current>) -> Pq {
        let p = self.a * rhs.a + self.b * rhs.b + self.c * rhs.c;
        let q = FRAC_1_SQRT_3
            * ((self.a - self.b) * rhs.c + (self.b - self.c) * rhs.a + (self.c - self.a) * rhs.b);
        Pq { p, q }
    }
    type Output = Pq;
}
impl Mul<Abc<Voltage>> for Abc<Current> {
    fn mul(self, rhs: Abc<Voltage>) -> Pq {
        rhs * self
    }
    type Output = Pq;
}

impl Mul<AlphaBeta<Current>> for AlphaBeta<Voltage> {
    fn mul(self, rhs: AlphaBeta<Current>) -> Pq {
        let p = 1.5 * (self.alpha * rhs.alpha + self.beta * rhs.beta);
        let q = 1.5 * (self.beta * rhs.alpha - self.alpha * rhs.beta);
        Pq { p, q }
    }
    type Output = Pq;
}

impl Mul<AlphaBeta<Voltage>> for AlphaBeta<Current> {
    fn mul(self, rhs: AlphaBeta<Voltage>) -> Pq {
        rhs * self
    }
    type Output = Pq;
}

impl Mul<AlphaBeta0<Current>> for AlphaBeta0<Voltage> {
    fn mul(self, rhs: AlphaBeta0<Current>) -> Pq {
        let p = 1.5 * (self.alpha * rhs.alpha + self.beta * rhs.beta + 2.0 * self.zero * rhs.zero);
        let q = 1.5 * (self.beta * rhs.alpha - self.alpha * rhs.beta);
        Pq { p, q }
    }
    type Output = Pq;
}

impl Mul<AlphaBeta0<Voltage>> for AlphaBeta0<Current> {
    fn mul(self, rhs: AlphaBeta0<Voltage>) -> Pq {
        rhs * self
    }
    type Output = Pq;
}

impl Mul<Dq0<Current>> for Dq0<Voltage> {
    fn mul(self, rhs: Dq0<Current>) -> Pq {
        let p = 1.5 * (self.d * rhs.d + self.q * rhs.q + 2.0 * self.zero * rhs.zero);
        let q = 1.5 * (self.q * rhs.d - self.d * rhs.q);
        Pq { p, q }
    }
    type Output = Pq;
}

impl Mul<Dq0<Voltage>> for Dq0<Current> {
    fn mul(self, rhs: Dq0<Voltage>) -> Pq {
        rhs * self
    }
    type Output = Pq;
}

impl Mul<Dq<Current>> for Dq<Voltage> {
    fn mul(self, rhs: Dq<Current>) -> Pq {
        let p = 1.5 * (self.d * rhs.d + self.q * rhs.q);
        let q = 1.5 * (self.q * rhs.d - self.d * rhs.q);
        Pq { p, q }
    }
    type Output = Pq;
}

impl Mul<Dq<Voltage>> for Dq<Current> {
    fn mul(self, rhs: Dq<Voltage>) -> Pq {
        rhs * self
    }
    type Output = Pq;
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::trig::cos_sin;
    use crate::trig::Theta;
    use approx::assert_abs_diff_eq;

    // helper function to assert pq approximate equality
    fn check_pqs(pq0: Pq, pq1: Pq) {
        assert_abs_diff_eq!(f32::from(pq0.p), f32::from(pq1.p), epsilon = 0.01,);
        assert_abs_diff_eq!(f32::from(pq0.q), f32::from(pq1.q), epsilon = 0.01,);
    }

    #[test]
    fn multiply() {
        let v_theta = Theta::from_degrees(20.0);
        let i_theta = Theta::from_degrees(45.0);
        let (cos, sin) = cos_sin(v_theta);

        let v_mag = Voltage::from(240.0);
        let i_mag = Current::from(1.0);
        let v_zero = Voltage::from(10.0);
        let i_zero = Current::from(-2.0);
        let v_abc = Abc::from_polar(v_mag, v_theta) + v_zero;
        let i_abc = Abc::from_polar(i_mag, i_theta) + i_zero;
        let pq_abc = v_abc * i_abc;

        let v_alpha_beta0 = AlphaBeta0::from(v_abc);
        let i_alpha_beta0 = AlphaBeta0::from(i_abc);
        let pq_alpha_beta0 = v_alpha_beta0 * i_alpha_beta0;

        let v_dq0 = v_abc.to_dq0(cos, sin);
        let i_dq0 = i_abc.to_dq0(cos, sin);
        let pq_dq0 = v_dq0 * i_dq0;

        check_pqs(pq_abc, pq_alpha_beta0);
        check_pqs(pq_abc, pq_dq0);
    }
}
