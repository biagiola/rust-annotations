fn main() {
	println!("{:?}", alphabets("zebra"));
}

fn alphabets(text: &str) -> (bool, bool) {
	// let a: bool = text.contains("a");
    // let b: bool = text.contains("z");
    
    // (a,b)
    
    // short version
	(text.contains("a"), text.contains("z"))
}