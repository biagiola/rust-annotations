fn factorial_recursive(current: i32) -> i32 {
	if current <= 1 {
		1
	} else {
		current * factorial_recursive(current - 1)
	}
}

fn main() {
	let result = factorial_recursive(4);
	println!("Factorial of four: {result}");
}

// factorial (4) -> 4 * 3 * 2 * 1
// factorial (4) -> 4 * factorial(3)
// factorial (3) -> 3 * factorial(2)