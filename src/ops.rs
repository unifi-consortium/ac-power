use crate::newtypes::{Current, Voltage};
use crate::reference_frames::{Abc, AlphaBeta, Pq};
use core::ops::Mul;

impl Mul<AlphaBeta<Current>> for AlphaBeta<Voltage> {
    fn mul(self, rhs: AlphaBeta<Current>) -> Pq {
        let p = 0.5 * (self.alpha * rhs.alpha + self.beta * rhs.beta);
        let q = 0.5 * (self.beta * rhs.alpha - self.alpha * rhs.beta);
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
