fn countdown(seconds: i32) {
	if seconds == 0 {
		println!("Blastoff..!!");
	} else {
		println!("{seconds} seconds to blastoff..!!");
		countdown(seconds - 1);
	}
}

fn main() {
	countdown(5);
}