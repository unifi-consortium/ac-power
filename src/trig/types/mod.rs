mod cos;
mod sin;
mod theta;

use core::ops::Mul;
pub use cos::Cos;
pub use sin::Sin;
pub use theta::Theta;

// impliment the trig multiplies for f32, our base primitive
impl Mul<f32> for Sin {
    fn mul(self, rhs: f32) -> f32 {
        f32::from(self) * rhs
    }
    type Output = f32;
}

impl Mul<Sin> for f32 {
    fn mul(self, rhs: Sin) -> f32 {
        self * f32::from(rhs)
    }
    type Output = f32;
}

impl Mul<f32> for Cos {
    fn mul(self, rhs: f32) -> f32 {
        f32::from(self) * rhs
    }
    type Output = f32;
}

impl Mul<Cos> for f32 {
    fn mul(self, rhs: Cos) -> f32 {
        self * f32::from(rhs)
    }
    type Output = f32;
}
