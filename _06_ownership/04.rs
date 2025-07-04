// String literals are the actual text values like "hello", "pasta", "eggs"
// They are not stored on either the stack or the heap.
// Rather, it's embedded directly into the binary executable that the Rust compiler produces,
// coz, the value is already known at compile time.
// They are immutable by default.
// This type is ok when we know the value at compile time.

// Now a reference to a string literal can change its value.

fn main() {
	let mut food: &str = "pasta"; // "pasta" is a string literal and food is a reference to it
	println!("{food}");

	// we can change the value of the reference to a string literal
	// but the string literal ("pasta") itself is immutable
	food = "eggs";
	println!("{food}");

	// &str doesn't have mutation methods like push, push_str, etc.

	// Also, dynamic strings from input user, reading content from a file etc, these variables are saved into the heap
	let text: String = String::new();

	// we can initialize its value with ::from
	let candy: String = String::from("kitkat");
}