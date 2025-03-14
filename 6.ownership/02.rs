fn main() {
	let age: i32 = 33;
	let is_handsome: bool = true;

	println!("{age}");
	println!("{is_handsome}");
}

// both variables are sent to the stack due the fixed sizes.
// stack are LIFO style, so after finish the flow, is_handsome variable will be deallocated first.
 
// | OWNER       - VALUE|
// | is_handsome - true |
// | age         - 33   |
//  ------ STACK --------