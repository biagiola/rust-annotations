fn even_or_odd() {
	let result: &str = if number % 2 == 0 {
		"even"
	} else {
		"odd"
	}
	println!("The number is {result}");
}

fn main() {
	even_or_odd(17);
}