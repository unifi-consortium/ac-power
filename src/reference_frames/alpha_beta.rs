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
