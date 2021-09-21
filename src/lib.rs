pub mod trig;
pub mod transforms;
pub mod common_tables;


#[cfg(test)]
mod tests {

	// use fixed::types::extra::*;
	// use fixed::FixedI16;
	use fixed::types::{I1F31, I11F21};

	#[test]
    fn play_with_fixed() {

    	let a = I1F31::from_num(0.768);
    	let b = I11F21::from_num(400.0);


    	println!("{:?}", a);
    	println!("{:?}", b);



	}
}