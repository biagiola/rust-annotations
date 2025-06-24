fn main() {
	let result: i32 = square(5);
	println!("{result}");
}

fn square(number: i32) -> i32 {
	number * number // we don't need to put return coz it's a unit trait
}