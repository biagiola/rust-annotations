fn main() {
	let multiplaier: i32 = 3;

	// open a block section and assign the return value
	// to calculation variable
	let calculation: i32 = {
		let value: i32 = 5 + 4;
		value * multiplier // if we not put semicolon we're return a unit
	};
}