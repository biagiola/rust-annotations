fn main() {
	// example of a normal tuple
	let student: (i32, bool, &str) = (99, true, "David Biagiola");
	println!("{student:?}");

	// also we can extract value from the tuple
	let (age, is_active, name) = student;
	println!("{}, is studying right now?: {}", name, is_active);

	// or access those value by index
	let (age, is_active, name) = student;
	println!("{}, is studying right now?: {}", student.2, student.1);

	// example of a unit, most simple tuple
	let emptyTuple: () = ();

	// an empty function return a unit
	let unit: () = mystery();
}

fn mystery() {
	println!("Hello"); // mystery also can make operations
}


// the student tuple could be an struct, and structs are more heavy meanwhile tuples
// are focus for quick and lightwieght data. With strcuts we can achieve more comples
// goals and also OOP.