use crate::trig::Theta;

/// Polar reference frame (just amplitude and angle)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Polar<T> {
    pub amplitude: T,
    pub theta: Theta,
}
