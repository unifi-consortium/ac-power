use crate::trig::Theta;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Polar<T> {
    pub amplitude: T,
    pub theta: Theta,
}
