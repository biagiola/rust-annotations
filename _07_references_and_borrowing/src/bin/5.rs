fn main() {
    let mut coffe: String = String::from("Mocha");

    let a: &mut String = &mut coffe;
    println!("{a}");

    let b: &mut String = a;
    println!("{b}");

    // In this way is okay, coz we're printing its
    // values before change of ownership
}