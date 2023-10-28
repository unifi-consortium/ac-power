use crate::plls::filter::PiFilter;
use crate::reference_frames::AlphaBeta;
use crate::trig::sin_cos;
use az::Cast;
use fixed::types::I0F32;
use fixed::types::I0F64;
use fixed::types::I1F31;
use fixed::FixedI32;
use heapless::Vec;

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
    // kalman blocks
    pub terms: Vec<Term<FRAC>, 12>,
    acc: FixedI32<FRAC>,
    fref: I0F32,
    pub f: I0F32,
    pub theta: I0F32,
    sin: I1F31,
    cos: I1F31,
    phase_filter: PiFilter,
    clock_filter: PiFilter,
}

impl<const FRAC: i32> Kalman<FRAC> {
    pub fn new(fref: f32, vref: f32, tset: f32, gamma: f32, max_integral: f32, ts: f32) -> Self {
        // create kalman oscillators
        let mut terms: Vec<Term<FRAC>, 12> = Vec::new();
        terms
            .push(Term::new(fref, ts, 0.08700214, -0.01708236))
            .unwrap();
        terms
            .push(Term::new(3.0 * fref, ts, 0.05441161, -0.02309186))
            .unwrap();
        terms
            .push(Term::new(5.0 * fref, ts, 0.05151601, -0.02898203))
            .unwrap();
        terms
            .push(Term::new(7.0 * fref, ts, 0.03887693, -0.04452462))
            .unwrap();

        // calculate the gains of the phase filter
        let kp = 8.0 / (tset * vref);
        let ki = 16.0 / (gamma * gamma * tset * tset * vref);

        // calculate the gains of clock filter
        let ka = 50.0 * 2e-3;
        let k = 50.0;

        // normalize terms to the sampling frequency and conver to fixed-point
        let fref_norm = I0F32::from_num(fref * ts);
        let kp_norm = I0F32::from_num(kp * ts);
        let ki_norm = I0F32::from_num(ki * ts * ts);
        let ka_norm = I0F32::from_num(ka * ts);
        let k_norm = I0F32::from_num(k * ts * ts);
        let max_integral_norm = I0F64::from_num(max_integral * ts);
        let phase_filter = PiFilter::new(kp_norm, ki_norm, max_integral_norm);
        let clock_filter = PiFilter::new(ka_norm, k_norm, max_integral_norm);

        Self {
            terms,
            acc: FixedI32::<FRAC>::ZERO,
            theta: I0F32::ZERO,
            sin: I1F31::ZERO,
            cos: I1F31::MAX,
            fref: fref_norm,
            f: fref_norm,
            phase_filter,
            clock_filter,
        }
    }
    pub fn update(&mut self, v: FixedI32<FRAC>) -> u32 {
        // kalman feedback section
        let v_error = v - self.acc;
        let mut acc = FixedI32::<FRAC>::ZERO;
        for term in &mut self.terms {
            acc += term.update(v_error);
        }
        self.acc = acc;

        // park transform
        let dq = self.terms[0].alpha_beta.to_dq(self.sin, self.cos);

        // PI control loop
        let f_error = self.phase_filter.update(dq.q);
        self.f = self.fref + f_error;

        // update the phase info
        self.theta = self.theta.wrapping_add(self.f);
        (self.sin, self.cos) = sin_cos(self.theta);

        // update the sample rate
        let f_eff = self.fref + self.clock_filter.update(f_error);

        // calculate the next sample time
        // This division is expensive, but critical for accurate frequency measurement
        // TODO: Consider optimizing
        let ratio: FixedI32<30> = f_eff.wide_div(self.f).cast();
        let mut lmt = FixedI32::<16>::from_bits(10_000 << 16);
        lmt *= ratio;
        lmt.to_bits() as u32
    }
}
