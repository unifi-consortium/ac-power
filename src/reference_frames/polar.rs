use fixed::types::I0F32;
use fixed::FixedI32;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Polar<const FRAC: i32> {
    pub amplitude: FixedI32<FRAC>,
    pub theta: I0F32,
}
