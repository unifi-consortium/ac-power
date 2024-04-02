use crate::newtypes::{Current, Voltage};
use crate::reference_frames::{Abc, AlphaBeta, Pq};
use core::ops::Mul;

impl Mul<AlphaBeta<Current>> for AlphaBeta<Voltage> {
    fn mul(self, rhs: AlphaBeta<Current>) -> Pq {
        let p = self.alpha * rhs.alpha + self.beta * rhs.beta;
        let q = self.beta * rhs.alpha - self.alpha * rhs.beta;
        Pq { p, q }
    }
    type Output = Pq;
}
impl Mul<AlphaBeta<Voltage>> for AlphaBeta<Current> {
    fn mul(self, rhs: AlphaBeta<Voltage>) -> Pq {
        let p = rhs.alpha * self.alpha + rhs.beta * self.beta;
        let q = rhs.beta * self.alpha - rhs.alpha * self.beta;
        Pq { p, q }
    }
    type Output = Pq;
}

impl Mul<Abc<Current>> for AlphaBeta<Voltage> {
    fn mul(self, rhs: Abc<Current>) -> Pq {
        self * AlphaBeta::from(rhs)
    }
    type Output = Pq;
}

impl Mul<AlphaBeta<Current>> for Abc<Voltage> {
    fn mul(self, rhs: AlphaBeta<Current>) -> Pq {
        AlphaBeta::from(self) * rhs
    }
    type Output = Pq;
}

impl Mul<Abc<Current>> for Abc<Voltage> {
    fn mul(self, rhs: Abc<Current>) -> Pq {
        AlphaBeta::from(self) * AlphaBeta::from(rhs)
    }
    type Output = Pq;
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     use crate::trig::Theta;
//     use approx::assert_abs_diff_eq;

//     #[test]
//     fn multiply() {
//         // let v_mag = Voltage::from(339.4112549695428);
//         // let i_mag = Current::from(1.4142135623730951);
//         let v_mag = Voltage::from(1.414 * 240.0);
//         let i_mag = Current::from(1.414 * 0.58);
//         let v = Abc::from_polar(v_mag, Theta::from_degrees(0.0));
//         let i = Abc::from_polar(i_mag, Theta::from_degrees(0.0));
//         let pq = v * i;

//         assert_abs_diff_eq!(f32::from(pq.p), 415.2, epsilon = 0.01);
//         // assert_abs_diff_eq!(f32::from(pq.q), 207.59, epsilon = 0.01);
//     }
// }
