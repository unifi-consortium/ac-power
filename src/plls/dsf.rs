use crate::plls::filter::PiFilter;
use crate::reference_frames::Abc;
use crate::reference_frames::AlphaBeta;
use crate::trig::sin_cos;
use fixed::types::I1F31;

pub struct Dsf<const FRAC: i32> {
    pub fref: I1F31,
    filter: PiFilter,
    pub theta: I1F31,
    pub sin: I1F31,
    pub cos: I1F31,
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

    pub fn update(&mut self, abc: Abc<FRAC>) {
        // clarke transform
        let alpha_beta = AlphaBeta::from(abc);

        // park transforms
        let dq0_pos = alpha_beta.to_dq0(self.sin, self.cos);
        let _dq0_neg = alpha_beta.to_dq0(-self.sin, self.cos);

        // TODO: Decoupling block

        // PI control loop
        let f = self.fref + self.filter.update(dq0_pos.q);

        // update the phase info
        self.theta = self.theta.wrapping_add(f);
        (self.sin, self.cos) = sin_cos(self.theta);
    }
}

// models.pll.output.theta = pll.theta.to_bits();
// models.pll.output.sin = pll.sin.to_bits();
// models.pll.output.cos = pll.cos.to_bits();

// models.pll.output.f = pll.f.to_bits();

// models.pll.output.alpha = pll.alpha.to_bits();
// models.pll.output.beta = pll.beta.to_bits();
// models.pll.output.zero = pll.zero.to_bits();

// models.pll.output.d_pos = pll.d_pos.to_bits();
// models.pll.output.q_pos = pll.q_pos.to_bits();
// models.pll.output.d_neg = pll.d_neg.to_bits();
// models.pll.output.q_neg = pll.q_neg.to_bits();
