use crate::plls::filter::{LowpassFilter, PiFilter};
use crate::reference_frames::Abc;
use crate::reference_frames::AlphaBeta;
use crate::trig::{cheyshev, sin_cos};
use fixed::types::I1F31;
use fixed::FixedI32;

pub struct Dsf<const FRAC: i32> {
    fref: I1F31,
    filter: PiFilter,
    theta: I1F31,
    sin: I1F31,
    cos: I1F31,

    // decoupling block filters
    d_pos_bar: LowpassFilter<FRAC>,
    q_pos_bar: LowpassFilter<FRAC>,
    d_neg_bar: LowpassFilter<FRAC>,
    q_neg_bar: LowpassFilter<FRAC>,
}

// 32-bit telemetry struct for ECDC data model
pub struct Telemetry {
    pub theta: i32,
    pub sin: i32,
    pub cos: i32,
    pub f: i32,

    pub alpha: i32,
    pub beta: i32,
    pub zero: i32,

    pub d_pos: i32,
    pub q_pos: i32,
    pub d_neg: i32,
    pub q_neg: i32,

    pub d_pos_hat: i32,
    pub q_pos_hat: i32,
    pub d_neg_hat: i32,
    pub q_neg_hat: i32,

    pub d_pos_bar: i32,
    pub q_pos_bar: i32,
    pub d_neg_bar: i32,
    pub q_neg_bar: i32,
}

impl<const FRAC: i32> Dsf<FRAC> {
    pub fn new(fref: f32, kp: f32, ki: f32, max_integral: f32, ts: f32) -> Dsf<FRAC> {
        // Normalize fref, kp, and ki to I1F31 numbers
        //  fref --> Hz       to   (%/cycle)
        //  kp   --> Hz/V     to   (%/cycle)/V
        //  ki   --> Hz/V-s   to   (%/cycle)/V/cycle
        // Where % represents the % of 360 degree
        let fref_norm = I1F31::from_num(fref * ts);
        let kp_norm = I1F31::from_num(kp * ts);
        let ki_norm = I1F31::from_num(ki * ts * ts);
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

    pub fn update(&mut self, abc: Abc<FRAC>) -> Telemetry {
        // clarke transform
        let alpha_beta = AlphaBeta::from(abc);

        // park transforms
        let dq_pos = alpha_beta.to_dq(self.sin, self.cos);
        let dq_neg = alpha_beta.to_dq(-self.sin, self.cos);
        let zero = FixedI32::<FRAC>::from(abc);

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

        // return telemetry
        Telemetry {
            theta: self.theta.to_bits(),
            sin: self.sin.to_bits(),
            cos: self.cos.to_bits(),
            f: f.to_bits(),
            alpha: alpha_beta.alpha.to_bits(),
            beta: alpha_beta.beta.to_bits(),
            zero: zero.to_bits(),
            d_pos: dq_pos.d.to_bits(),
            q_pos: dq_pos.q.to_bits(),
            d_neg: dq_neg.d.to_bits(),
            q_neg: dq_neg.q.to_bits(),

            d_pos_hat: d_pos_hat.to_bits(),
            q_pos_hat: q_pos_hat.to_bits(),
            d_neg_hat: d_neg_hat.to_bits(),
            q_neg_hat: q_neg_hat.to_bits(),

            d_pos_bar: self.d_pos_bar.value.to_bits(),
            q_pos_bar: self.q_pos_bar.value.to_bits(),
            d_neg_bar: self.d_neg_bar.value.to_bits(),
            q_neg_bar: self.q_neg_bar.value.to_bits(),
        }
    }
}
