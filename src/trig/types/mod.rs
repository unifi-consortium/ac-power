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

        impl MulAssign<$t> for f32 {
            fn mul_assign(&mut self, other: $t) {
                *self *= other.0;
            }
        }

        impl From<$t> for f32 {
            fn from(item: $t) -> Self {
                item.0
            }
        }

        impl From<f32> for $t {
            fn from(item: f32) -> Self {
                // if item > 1.0 || item < -1.0 {
                //     panic!("A $t type must be between -1.0 and +1.0");
                // }
                Self(item)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_trig_ops {
    ($tr: ty, $nt: ty) => {
        impl Mul<$nt> for $tr {
            fn mul(self, rhs: $nt) -> $nt {
                rhs * self
            }
            type Output = $nt;
        }

        impl Mul<$tr> for $nt {
            fn mul(self, other: $tr) -> $nt {
                self * f32::from(other)
            }
            type Output = $nt;
        }

        impl MulAssign<$tr> for $nt {
            fn mul_assign(&mut self, other: $tr) {
                self.0 *= f32::from(other);
            }
        }
    };
}
