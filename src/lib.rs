#![cfg_attr(not(test), no_std)]

pub mod constants;
pub mod operations;
pub mod plls;
pub mod reference_frames;
pub mod transforms;
pub mod trig;

#[cfg(test)]
mod tests {

    // use fixed::types::extra::*;
    use fixed::FixedI32;

    #[test]
    fn play_with_fixed() {
        let a = FixedI32::<45>::from_num(1.5e-9);
        let b = FixedI32::<45>::from_num(100e-9);
        let c = a.wide_mul(b);

        println!("{:?}", a);
        println!("{:?}", b);
        println!("{:?}", c);
        println!("{:?}", a.to_bits());
        println!("{:?}", b.to_bits());
        println!("{:?}", c.to_bits());

        println!("{:?}", 1.5e-9);
        println!("{:?}", 100e-9);
        println!("{:?}", 1.5e-9 * 100e-9);
        println!("{:?}", a.to_bits());
        println!("{:?}", b.to_bits());
        println!("{:?}", c.to_bits());
    }
}
