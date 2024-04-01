use crate::reference_frames::{Abc, Dq};
use crate::trig::{chebyshev, cos_sin, Cos, Sin, Theta};

pub struct Waveform<const N: usize> {
    pub positive: [Dq; N],
    pub negative: [Dq; N],
    pub zero: Dq,
}

impl<const N: usize> Waveform<N> {
    pub fn new() -> Self {
        Self {
            positive: [Dq::ZERO; N],
            negative: [Dq::ZERO; N],
            zero: Dq::ZERO,
        }
    }

    pub fn calculate(&self, theta: Theta) -> Abc {
        let (cos, sin) = cos_sin(theta);

        let mut abc = Abc::ZERO + self.zero.d * sin + self.zero.q * cos;

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
            (cosn, sinn) = chebyshev(cos, sinn1, cosn1, sinn2, cosn2);
        }

        abc
    }
}
