mod cos;
mod sin;
mod theta;

pub use cos::Cos;
pub use sin::Sin;
pub use theta::Theta;

// helper macros for implementing arithmetic on sin and cos
#[macro_export]
macro_rules! impl_ops {
    ($trig:ty, $type:ty) => {
        impl Mul<$type> for $trig {
            fn mul(self, other: $type) -> $type {
                self.0 * other
            }
            type Output = $type;
        }

        impl Mul<$trig> for $type {
            fn mul(self, other: $trig) -> $type {
                self * other.0
            }
            type Output = $type;
        }

        impl Sub<$trig> for $type {
            fn sub(self, other: $trig) -> $type {
                self - other.0
            }
            type Output = $type;
        }

        impl MulAssign<$trig> for $type {
            fn mul_assign(&mut self, other: $trig) {
                *self *= other.0;
            }
        }

        impl Neg for $trig {
            fn neg(self) -> Self {
                Self(-self.0)
            }
            type Output = Self;
        }

        impl From<$trig> for $type {
            fn from(item: $trig) -> Self {
                item.0
            }
        }

        impl From<$type> for $trig {
            fn from(item: $type) -> Self {
                // if item > 1.0 || item < -1.0 {
                //     panic!("A $trig type must be between -1.0 and +1.0");
                // }
                Self(item)
            }
        }
    };
}
