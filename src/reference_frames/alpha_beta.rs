// Balanced reference frames
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AlphaBeta<T> {
    pub alpha: T,
    pub beta: T,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AlphaBeta0<T> {
    pub alpha: T,
    pub beta: T,
    pub zero: T,
}

impl<T: From<f32>> AlphaBeta<T> {
    pub fn zero() -> Self {
        Self {
            alpha: 0.0.into(),
            beta: 0.0.into(),
        }
    }
}

impl<T: From<f32>> AlphaBeta0<T> {
    pub fn zero() -> Self {
        Self {
            alpha: 0.0.into(),
            beta: 0.0.into(),
            zero: 0.0.into(),
        }
    }
}
