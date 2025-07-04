// Rust's primitive data types, scalar, the fixed size one that we store on the stack, implements copy trait.
// This means full copies of those values will be created automatically in situations where duplicates are needed.

fn main() {
	let mut time: i32 = 2025;
	let year: i32 = time;

	println!("Time: {time}");
	println!("Year: {year}");

	time = 2026;

	println!("Time: {time}");
	println!("Year: {year}");

    // time and year variables are in differents allocations
    // they don't share thier references
    // they reprensent two duplicates, separate and independent variables
    // primitive data types implements copy trait, so they are independent.
    
    // ----------------------
    // | OWNER        VALUE|
    // | time          2025 |
    // | year          2025 |
    // ------ STACK --------
}