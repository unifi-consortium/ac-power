use crate::reference_frames::Abc;

pub mod dsf;
pub mod filter;

pub trait PhaseLockedLoop<const FRAC: i32> {
    fn new(fref: f32, kp: f32, ki: f32, max_integral: f32, ts: f32) -> Self;
    fn update(&mut self, abc: Abc<FRAC>);
}
