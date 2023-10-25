use fixed::types::I1F31;
use fixed::FixedI32;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Abc<const FRAC: i32> {
    pub a: FixedI32<FRAC>,
    pub b: FixedI32<FRAC>,
    pub c: FixedI32<FRAC>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AlphaBeta<const FRAC: i32> {
    pub alpha: FixedI32<FRAC>,
    pub beta: FixedI32<FRAC>,
}

// polar
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Polar<const FRAC: i32> {
    pub amplitude: FixedI32<FRAC>,
    pub theta: I1F31,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dq<const FRAC: i32> {
    pub d: FixedI32<FRAC>,
    pub q: FixedI32<FRAC>,
}

// dq0
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dq0<const FRAC: i32> {
    pub d: FixedI32<FRAC>,
    pub q: FixedI32<FRAC>,
    pub zero: FixedI32<FRAC>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AlphaBetaZero<const FRAC: i32> {
    pub alpha: FixedI32<FRAC>,
    pub beta: FixedI32<FRAC>,
    pub zero: FixedI32<FRAC>,
}
