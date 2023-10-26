use crate::plls::filter::{LowpassFilter, PiFilter};
use crate::plls::Pll;
use crate::reference_frames::Abc;
use crate::reference_frames::AlphaBeta;
use crate::trig::{cheyshev, sin_cos};
use fixed::types::I1F31;

pub struct Dsf<const FRAC: i32> {
    pub fref: I1F31,
    filter: PiFilter,
    pub theta: I1F31,
    pub sin: I1F31,
    pub cos: I1F31,

    // decoupling block filters
    pub d_pos_bar: LowpassFilter<FRAC>,
    pub q_pos_bar: LowpassFilter<FRAC>,
    pub d_neg_bar: LowpassFilter<FRAC>,
    pub q_neg_bar: LowpassFilter<FRAC>,
}

impl<const FRAC: i32> Pll<FRAC> for Dsf<FRAC> {
    fn new(fref: f32, kp: f32, ki: f32, max_integral: f32, ts: f32) -> Dsf<FRAC> {
        // Normalize fref, kp, and ki to I1F31 numbers
        //  fref --> Hz       to   (%/cycle)
        //  kp   --> Hz/V     to   (%/cycle)/V
        //  ki   --> Hz/V-s   to   (%/cycle)/V/cycle
        // Where % represents the % of 360 degree
        let fref_norm = I1F31::from_num(2.0 * fref * ts);
        let kp_norm = I1F31::from_num(2.0 * kp * ts);
        let ki_norm = I1F31::from_num(2.0 * ki * ts * ts);
        let max_integral_norm = I1F31::from_num(max_integral * ts);

        let filter = PiFilter::new(kp_norm, ki_norm, max_integral_norm);
        Self {
            fref: fref_norm,
            filter,
            theta: I1F31::ZERO,
            sin: I1F31::ZERO,
            cos: I1F31::MAX,
            d_pos_bar: LowpassFilter::<FRAC>::new(I1F31::from_num(0.01)),
            q_pos_bar: LowpassFilter::<FRAC>::new(I1F31::from_num(0.01)),
            d_neg_bar: LowpassFilter::<FRAC>::new(I1F31::from_num(0.01)),
            q_neg_bar: LowpassFilter::<FRAC>::new(I1F31::from_num(0.01)),
        }
    }

    fn update(&mut self, abc: Abc<FRAC>) {
        // clarke transform
        let alpha_beta = AlphaBeta::from(abc);

        // park transforms
        let dq_pos = alpha_beta.to_dq(self.sin, self.cos);
        let dq_neg = alpha_beta.to_dq(-self.sin, self.cos);

        // De-coupling block
        let (sin2, cos2) = cheyshev(self.cos, self.sin, self.cos, I1F31::ZERO, I1F31::MAX);
        let mut d_pos_hat = dq_pos.d;
        d_pos_hat.mul_acc(-cos2, self.d_neg_bar.value);
        d_pos_hat.mul_acc(-sin2, self.q_neg_bar.value);
        let mut q_pos_hat = dq_pos.q;
        q_pos_hat.mul_acc(sin2, self.d_neg_bar.value);
        q_pos_hat.mul_acc(-cos2, self.q_neg_bar.value);
        let mut d_neg_hat = dq_neg.d;
        d_neg_hat.mul_acc(-cos2, self.d_pos_bar.value);
        d_neg_hat.mul_acc(sin2, self.q_pos_bar.value);
        let mut q_neg_hat = dq_neg.q;
        q_neg_hat.mul_acc(-sin2, self.d_pos_bar.value);
        q_neg_hat.mul_acc(-cos2, self.q_pos_bar.value);
        self.d_pos_bar.update(d_pos_hat);
        self.q_pos_bar.update(q_pos_hat);
        self.d_neg_bar.update(d_neg_hat);
        self.q_neg_bar.update(q_neg_hat);

        // PI control loop
        let f = self.fref + self.filter.update(q_pos_hat);

        // update the phase info
        self.theta = self.theta.wrapping_add(f);
        (self.sin, self.cos) = sin_cos(self.theta);
    }
}
