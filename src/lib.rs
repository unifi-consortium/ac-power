#![cfg_attr(not(test), no_std)]

pub mod common_tables;
pub mod transforms;
pub mod trig;

#[cfg(test)]
mod tests {

    // use fixed::types::extra::*;
    // use fixed::FixedI16;
    use fixed::types::{I11F21, I1F31};

    #[test]
    fn play_with_fixed() {
        let a = I1F31::from_num(0.768);
        let b = I11F21::from_num(400.0);

        println!("{:?}", a);
        println!("{:?}", b);
    }
}
