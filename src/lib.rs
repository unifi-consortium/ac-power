pub mod trig;
// pub mod transforms;
pub mod common_tables;


#[cfg(test)]
mod tests {

	// use fixed::types::extra::*;
	// use fixed::FixedI16;
	use fixed::types::I1F31;

	#[test]
    fn play_with_fixed() {

    	let a = I1F31::from_num(0.768);
    	let b = I1F31::from_num(0.56);
    	let c = a.wide_mul(b);

    	let y = I1F31::saturating_from_num(c);

    	println!("{:?}", a);
    	println!("{:?}", b);
    	println!("{:?}", c);
    	println!("{:?}", y);


	}
}