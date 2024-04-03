mod cos;
mod sin;
mod theta;

pub use cos::Cos;
pub use sin::Sin;
pub use theta::Theta;

// helper macros for implementing arithmetic on sin and cos
#[macro_export]
macro_rules! impl_f32_ops {
    ($t:ty) => {
        impl Mul<f32> for $t {
            fn mul(self, other: f32) -> f32 {
                self.0 * other
            }
            type Output = f32;
        }

        impl Mul<$t> for f32 {
            fn mul(self, other: $t) -> f32 {
                self * other.0
            }
            type Output = f32;
        }
    };
}
