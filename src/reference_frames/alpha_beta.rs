use fixed::FixedI32;

// Balanced reference frames
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AlphaBeta<const FRAC: i32> {
    pub alpha: FixedI32<FRAC>,
    pub beta: FixedI32<FRAC>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AlphaBeta0<const FRAC: i32> {
    pub alpha: FixedI32<FRAC>,
    pub beta: FixedI32<FRAC>,
    pub zero: FixedI32<FRAC>,
}
