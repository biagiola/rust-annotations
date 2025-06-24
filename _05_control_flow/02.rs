fn main() {
	let mut counter: i32 = 10;

	loop {
		if counter < 0 { println!("Blastoff!!"); break };
		println!("{counter} near to blastoff..!!");
		counter -= 1;
	}
}