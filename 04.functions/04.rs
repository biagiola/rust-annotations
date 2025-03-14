#[allow(unused_variables)]
fn main() {
	apply_to_jobs(4, "Software development");

	let result: bool = is_even(-2);
	println!("{result}");
}

fn apply_to_jobs(number: i32, title: &str) {
	println!("I'm applying to {number} {title} jobs");
}

fn is_even(number: i32) -> bool {
	number % 2 == 0
}

fn find_letter(word: &str, letter: char) {
	for c in word {
		print!("{c}");
	}
}