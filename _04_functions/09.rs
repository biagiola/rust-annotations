fn main() {
	let season: &str = "winter";

	match season {
		"summer" => println!("School's out!"),
		"winter" => println!("Brr, so cold"),
		_ => println!("Lot's of rain"),
	}
}