use crate::reference_frames::{Abc, AlphaBeta, Pq};
use core::ops::Mul;

impl Mul<AlphaBeta> for AlphaBeta {
    fn mul(self, rhs: AlphaBeta) -> Pq {
        let p = 0.5 * (self.alpha * rhs.alpha + self.beta * rhs.beta);
        let q = 0.5 * (self.beta * rhs.alpha - self.alpha * rhs.beta);
        Pq { p, q }
    }
    type Output = Pq;
}

impl Mul<Abc> for AlphaBeta {
    fn mul(self, rhs: Abc) -> Pq {
        self * AlphaBeta::from(rhs)
    }
    type Output = Pq;
}

impl Mul<AlphaBeta> for Abc {
    fn mul(self, rhs: AlphaBeta) -> Pq {
        AlphaBeta::from(self) * rhs
    }
    type Output = Pq;
}

impl Mul<Abc> for Abc {
    fn mul(self, rhs: Abc) -> Pq {
        AlphaBeta::from(self) * AlphaBeta::from(rhs)
    }
    type Output = Pq;
}
