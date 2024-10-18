mod from_abc;
mod from_alpha_beta;
mod from_dq;

use crate::constants::{FRAC_1_2SQRT3, ONE_HALF, ONE_SIXTH, ONE_THIRD, SQRT_3_OVER_2};
use crate::number::Num;
use crate::reference_frames::Dq;

/// Convert pos, neg, zero Dq's to line Dq's
pub fn seq_to_lines<T: Num>(pos: Dq<T>, neg: Dq<T>, zero: Dq<T>) -> (Dq<T>, Dq<T>, Dq<T>) {
    let a = Dq {
        d: pos.d - neg.d + zero.d,
        q: pos.q + neg.q + zero.q,
    };
    let b = Dq {
        d: -pos.d * ONE_HALF + pos.q * SQRT_3_OVER_2 + neg.d * ONE_HALF - neg.q * SQRT_3_OVER_2
            + zero.d,
        q: -pos.d * SQRT_3_OVER_2 - pos.q * ONE_HALF - neg.d * SQRT_3_OVER_2 - neg.q * ONE_HALF
            + zero.q,
    };
    let c = Dq {
        d: -pos.d * ONE_HALF - pos.q * SQRT_3_OVER_2
            + neg.d * ONE_HALF
            + neg.q * SQRT_3_OVER_2
            + zero.d,
        q: pos.d * SQRT_3_OVER_2 - pos.q * ONE_HALF + neg.d * SQRT_3_OVER_2 - neg.q * ONE_HALF
            + zero.q,
    };

    (a, b, c)
}

/// Convert line phasors to pos, neg, zero
pub fn lines_to_seq<T: Num>(a: Dq<T>, b: Dq<T>, c: Dq<T>) -> (Dq<T>, Dq<T>, Dq<T>) {
    let pos = Dq {
        d: a.d * ONE_THIRD - b.d * ONE_SIXTH - b.q * FRAC_1_2SQRT3 - c.d * ONE_SIXTH
            + c.q * FRAC_1_2SQRT3,
        q: a.q * ONE_THIRD + b.d * FRAC_1_2SQRT3
            - b.q * ONE_SIXTH
            - c.d * FRAC_1_2SQRT3
            - c.q * ONE_SIXTH,
    };
    let neg = Dq {
        d: -a.d * ONE_THIRD + b.d * ONE_SIXTH - b.q * FRAC_1_2SQRT3
            + c.d * ONE_SIXTH
            + c.q * FRAC_1_2SQRT3,
        q: a.q * ONE_THIRD - b.d * FRAC_1_2SQRT3 - b.q * ONE_SIXTH + c.d * FRAC_1_2SQRT3
            - c.q * ONE_SIXTH,
    };
    let zero = (a + b + c) * ONE_THIRD;

    (pos, neg, zero)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::trig::{cos_sin, Theta};
    use crate::{Abc, Sequence};
    use approx::assert_relative_eq;

    fn linspace(x0: f32, x1: f32, length: usize) -> Vec<f32> {
        let dx = (x1 - x0) / ((length - 1) as f32);
        let mut xs: Vec<f32> = vec![x0];
        for index in 1..length {
            xs.push(xs[index - 1] + dx);
        }
        xs
    }

    #[test]
    fn test_inverse() {
        let a = Dq { d: 1.0, q: 2.0 };
        let b = Dq { d: 3.0, q: 4.0 };
        let c = Dq { d: 5.0, q: 6.0 };

        let (pos, neg, zero) = lines_to_seq(a, b, c);
        let (a_, b_, c_) = seq_to_lines(pos, neg, zero);

        // verify
        assert_relative_eq!(a_.d, a.d, max_relative = 0.01);
        assert_relative_eq!(a_.q, a.q, max_relative = 0.01);
        assert_relative_eq!(b_.d, b.d, max_relative = 0.01);
        assert_relative_eq!(b_.q, b.q, max_relative = 0.01);
        assert_relative_eq!(c_.d, c.d, max_relative = 0.01);
        assert_relative_eq!(c_.q, c.q, max_relative = 0.01);
    }

    #[test]
    fn test_conversion_from_lines_to_seq() {
        let dq_a = Dq { d: 1.0, q: 2.0 };
        let dq_b = Dq { d: 3.0, q: 4.0 };
        let dq_c = Dq { d: 5.0, q: 6.0 };

        let (pos, neg, zero) = lines_to_seq(dq_a, dq_b, dq_c);

        // verify correct waveforms over line cycle
        let n = 2000;
        let angles = linspace(0.0, 360.0, n);
        for angle in angles.iter() {
            let theta = Theta::from_degrees(*angle);
            let (cos, sin) = cos_sin(theta);

            // calculate the instantaneous a, b, c from phasors
            let abc_phasor = Abc {
                a: dq_a.d * sin + dq_a.q * cos,
                b: dq_b.d * sin + dq_b.q * cos,
                c: dq_c.d * sin + dq_c.q * cos,
            };

            // calculate the instantaneous a, b, c from sequences
            let abc_sequence = pos.to_abc(cos, sin, Sequence::POSITIVE)
                + neg.to_abc(cos, sin, Sequence::NEGATIVE)
                + zero.to_abc(cos, sin, Sequence::ZERO);

            // verify
            assert_relative_eq!(abc_phasor.a, abc_sequence.a, max_relative = 0.01);
            assert_relative_eq!(abc_phasor.b, abc_sequence.b, max_relative = 0.01);
            assert_relative_eq!(abc_phasor.c, abc_sequence.c, max_relative = 0.01);
        }
    }

    #[test]
    fn test_conversion_from_seq_to_lines() {
        let pos = Dq { d: 1.0, q: 2.0 };
        let neg = Dq { d: 3.0, q: 4.0 };
        let zero = Dq { d: 5.0, q: 6.0 };

        let (dq_a, dq_b, dq_c) = seq_to_lines(pos, neg, zero);

        // Lets calculate numerically using park transforms and
        // averaging over one period
        let n = 2000;
        let angles = linspace(0.0, 360.0, n);
        let mut dq_pos_sum = Dq::zero();
        let mut dq_neg_sum = Dq::zero();
        for angle in angles.iter() {
            let theta = Theta::from_degrees(*angle);
            let (cos, sin) = cos_sin(theta);
            let a = dq_a.d * sin + dq_a.q * cos;
            let b = dq_b.d * sin + dq_b.q * cos;
            let c = dq_c.d * sin + dq_c.q * cos;
            let abc = Abc { a, b, c };
            let dq_pos = abc.to_dq(cos, sin, Sequence::POSITIVE);
            let dq_neg = abc.to_dq(cos, sin, Sequence::NEGATIVE);
            dq_pos_sum = dq_pos_sum + dq_pos;
            dq_neg_sum = dq_neg_sum + dq_neg;
        }

        // verify
        assert_relative_eq!(pos.d * (n as f32), dq_pos_sum.d, max_relative = 0.01);
        assert_relative_eq!(pos.q * (n as f32), dq_pos_sum.q, max_relative = 0.01);
        assert_relative_eq!(neg.d * (n as f32), dq_neg_sum.d, max_relative = 0.01);
        assert_relative_eq!(neg.q * (n as f32), dq_neg_sum.q, max_relative = 0.01);
    }
}
