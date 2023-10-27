use crate::trig::sin_cos;
use fixed::types::I0F32;
use fixed::FixedI32;

use crate::reference_frames::AlphaBeta;

use fixed::types::I1F31;

#[derive(Debug, Copy, Clone)]
pub struct Term<const FRAC: i32> {
    pub alpha_beta: AlphaBeta<FRAC>,

    // oscillation constants
    sin: I1F31,
    cos: I1F31,

    // feedback gains
    pub k_alpha: I1F31,
    pub k_beta: I1F31,
}

impl<const FRAC: i32> Term<FRAC> {
    pub fn new(fref: f32, ts: f32, k_alpha: f32, k_beta: f32) -> Self {
        let (sin, cos) = sin_cos(I0F32::from_num(fref * ts));
        Self {
            alpha_beta: AlphaBeta {
                alpha: FixedI32::<FRAC>::ZERO,
                beta: FixedI32::<FRAC>::ZERO,
            },
            sin,
            cos,
            k_alpha: I1F31::from_num(k_alpha),
            k_beta: I1F31::from_num(k_beta),
        }
    }
    pub fn update(&mut self, error: FixedI32<FRAC>) -> FixedI32<FRAC> {
        let mut alpha = self.alpha_beta.alpha;
        alpha *= self.cos;
        alpha.mul_acc(-self.sin, self.alpha_beta.beta);
        alpha.mul_acc(self.k_alpha, error);

        let mut beta = self.alpha_beta.beta;
        beta *= self.cos;
        beta.mul_acc(self.sin, self.alpha_beta.alpha);
        beta.mul_acc(self.k_beta, error);

        self.alpha_beta.alpha = alpha;
        self.alpha_beta.beta = beta;
        alpha
    }
}

pub struct Kalman<const FRAC: i32> {
    pub term: Term<FRAC>,
    acc: FixedI32<FRAC>,
    error: FixedI32<FRAC>,
}

impl<const FRAC: i32> Kalman<FRAC> {
    pub fn new(fref: f32, ts: f32) -> Self {
        let term = Term::new(fref, ts, 0.08, -0.04);
        Self {
            term,
            acc: FixedI32::<FRAC>::ZERO,
            error: FixedI32::<FRAC>::ZERO,
        }
    }
    pub fn update(&mut self, v: FixedI32<FRAC>) {
        let error = v - self.acc;

        let mut acc = FixedI32::<FRAC>::ZERO;
        acc += self.term.update(error);

        self.acc = acc;
        self.error = error;
    }
}
