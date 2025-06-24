// Strings are not stored on either the stack or the heap. Rather, it's embedded directly into the binary executable that the Rust compiler produces, coz, the value is already known at compile time. (it is not mutable btw)

fn main() {
	// this type is ok when we know the value at compile time
	let mut food: &str = "pasta";

	println!("{food}");

	food = "eggs";
	println!("{food}");

	// dynamic strings from input user, reading content from a file etc
	// these variables are saved into the heap
	let text: String = String::new();

	// we ca initialize its value with ::from
	let candy: String = String::from("kitkat");
}