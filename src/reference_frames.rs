use fixed::types::extra::LeEqU32;
use fixed::types::I1F31;
use fixed::FixedI32;

// alpha beta
#[derive(Debug)]
pub struct AlphaBeta<Frac: LeEqU32> {
    pub alpha: FixedI32<Frac>,
    pub beta: FixedI32<Frac>,
    pub gamma: FixedI32<Frac>,
}

// abc
#[derive(Debug)]
pub struct Abc<Frac: LeEqU32> {
    pub a: FixedI32<Frac>,
    pub b: FixedI32<Frac>,
    pub c: FixedI32<Frac>,
}

// dq0
#[derive(Debug)]
pub struct Dq0<Frac: LeEqU32> {
    pub d: FixedI32<Frac>,
    pub q: FixedI32<Frac>,
    pub z: FixedI32<Frac>,
}

// polar
#[derive(Debug)]
pub struct Polar<Frac: LeEqU32> {
    pub amplitude: FixedI32<Frac>,
    pub theta: I1F31,
}
