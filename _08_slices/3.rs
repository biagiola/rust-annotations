fn main() {
    let action_hero: String = String::from("Arnold Schwarzenegger");
    let string_reference: &String = &action_hero;
    // this code is what we're learning in previous lecture.
    // In that previous way we can only borrow the whole thing.
    println!("{string_reference}");

    // but now we take a slice of the String
    let first_name: &str = &action_hero[0..6]; // first 6 bytes
    // Here, Rust creates a slice reference (&str) that contains:
    // 1. A pointer to the starting position in the heap (where "Arnold" begins)
    // 2. A length (6 bytes)
    // The slice doesn't copy the data - it just references part of the original string

    let last_name: &str = &action_hero[7..21];
    println!("{first_name} {last_name}");

    // same example but with numbers
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let slice_num: &[i32] = &numbers[0..3];
    println!("{slice_num:?}");
}
