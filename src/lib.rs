#![cfg_attr(not(test), no_std)]

pub mod constants;
pub mod reference_frames;
pub mod transforms;
pub mod trig;

#[cfg(test)]
mod tests {}
