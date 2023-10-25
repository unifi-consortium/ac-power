use crate::plls::filter::PiFilter;
use crate::reference_frames::Abc;
use crate::reference_frames::AlphaBeta;
use crate::trig::sin_cos;
use fixed::types::I1F31;
use fixed::FixedI32;

pub struct Dsf<const FRAC: i32> {
    pub fref: I1F31,
    filter: PiFilter,
    pub theta: I1F31,
    pub sin: I1F31,
    pub cos: I1F31,
}

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
}

impl<const FRAC: i32> Dsf<FRAC> {
    pub fn new(fref: i32, kp: i32, ki: i32, max_integral: i32) -> Dsf<FRAC> {
        let fref = I1F31::from_bits(fref);
        let filter = PiFilter::new(kp, ki, max_integral);
        Self {
            fref,
            filter,
            theta: I1F31::from_bits(0),
            sin: I1F31::from_bits(0),
            cos: I1F31::MAX,
        }
    }

    pub fn update(&mut self, abc: Abc<FRAC>) -> Telemetry {
        // clarke transform
        let alpha_beta = AlphaBeta::from(abc);

        // park transforms
        let dq_pos = alpha_beta.to_dq(self.sin, self.cos);
        let dq_neg = alpha_beta.to_dq(-self.sin, self.cos);
        let zero = FixedI32::<FRAC>::from(abc);

        // TODO: Decoupling block

        // PI control loop
        let f = self.fref + self.filter.update(dq_pos.q);

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
        }
    }
}
