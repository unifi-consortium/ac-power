use crate::number::Num;
use crate::reference_frames::{Abc, Dq};
use crate::trig::{chebyshev, cos_sin, Cos, Sin, Theta};

pub struct Waveform<const N: usize, T> {
    pub positive: [Dq<T>; N],
    pub negative: [Dq<T>; N],
    pub zero: Dq<T>,
}

impl<const N: usize, T: Num> Waveform<N, T> {
    pub fn new() -> Self {
        Self {
            positive: [Dq::zero(); N],
            negative: [Dq::zero(); N],
            zero: Dq::zero(),
        }
    }

    pub fn calculate(&self, theta: Theta) -> Abc<T> {
        let (cos, sin) = cos_sin(theta);

        let mut abc = Abc::zero() + self.zero.d * sin + self.zero.q * cos;

        // add the harmonics
        let (mut cosn1, mut sinn1) = (Cos::from(1.0), Sin::from(0.0));
        let (mut cosn, mut sinn) = (cos, sin);
        for (pos, neg) in self.positive.iter().zip(self.negative.iter()) {
            abc += pos.to_abc(cosn, sinn);
            abc += neg.to_abc(cosn, -sinn);

            // use chebychev function to calculate cos, sin of next harmonic
            let cosn2 = cosn1;
            let sinn2 = sinn1;
            cosn1 = cosn;
            sinn1 = sinn;
            (cosn, sinn) = chebyshev(cos, cosn1, sinn1, cosn2, sinn2);
        }

        abc
    }
}
