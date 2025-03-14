fn main() {
	// example of a normal tuple
	let result: (i32, bool, &str) = (5, true, "hello");

	// example of a unit
	let emptyTuple: () = ();

	// an empty function return a unit
	let unit: () = mystery();
}

fn mystery() {
	println!("Hello"); // mystery also can make operations
}