fn main() {
    let action_hero: String = String::from("Arnold Schwarzenegger");
    let string_reference: &String = &action_hero;
    // this code is what we're learning in previous lecture.
    // In that previous way we can only borrow the whole thing.
    println!("{string_reference}");

    // but now we take a slice of the String
    let first_name: &str = &action_hero[0..6]; // six first bytes
    // here, Rust is going to dereference the address
    // to get to the original value in memory but now
    // we are storing the address, not of the complete string text on the heap
    // and basically, an address to just the first 6 bytes which
    // comprises the content Arnold (we're using english alphabet)

    let last_name: &str = &action_hero[7..21];
    println!("{first_name} {last_name}");

    // same example but with numbers
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let slice_num:  &[i32] = &numbers[0..3];
    println!("{slice_num:?}");
}