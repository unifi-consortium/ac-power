// Balanced reference frames
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AlphaBeta {
    pub alpha: f32,
    pub beta: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AlphaBeta0 {
    pub alpha: f32,
    pub beta: f32,
    pub zero: f32,
}

impl AlphaBeta {
    pub const ZERO: AlphaBeta = AlphaBeta {
        alpha: 0.0,
        beta: 0.0,
    };
}

impl AlphaBeta0 {
    pub const ZERO: AlphaBeta0 = AlphaBeta0 {
        alpha: 0.0,
        beta: 0.0,
        zero: 0.0,
    };
}
