use crate::trig::Cos;
use idsp;

// function to normalize p and q, which are floats, to fixed-point i32 while preverving ratio
fn normalize(x: f32, y: f32) -> (i32, i32) {
    let norm = 2147483648. * f32::max(x, y).recip();
    let xn = (norm * x) as i32;
    let yn = (norm * y) as i32;
    (xn, yn)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pq {
    pub p: f32,
    pub q: f32,
}

impl Pq {
    pub fn power_factor(&self) -> Cos {
        // convert p and q into fixed-point format for efficient trig
        let (x, y) = normalize(self.p, self.q);

        // calculate the fixed-point power factor (PF = cos(arctan(Q/P)))
        let (pf, _) = idsp::cossin(idsp::atan2(y, x));

        pf.into()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn power_factor() {
        let pq = Pq { p: 100.0, q: 0.0 };
        let pf = pq.power_factor();
        assert_abs_diff_eq!(f32::from(pf), 1.0, epsilon = 0.0001);

        let pq = Pq { p: 0.0, q: 1.0 };
        let pf = pq.power_factor();
        assert_abs_diff_eq!(f32::from(pf), 0.0, epsilon = 0.0001);

        let pq = Pq { p: 1.0, q: 1.0 };
        let pf = pq.power_factor();
        assert_abs_diff_eq!(f32::from(pf), 0.707, epsilon = 0.0001);
    }
}
