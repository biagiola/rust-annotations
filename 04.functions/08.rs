fn main() {
	let evaluation: bool = true;

	match evaluation {
		true => { println!("The value is true"); }
		false => { println!("The value is false"); }
	}

	let result = match evaluation {
		true => 20,
		false => 40,
	}

	println!("Result: {result}");
}