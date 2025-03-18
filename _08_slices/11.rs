fn main() {
    let action_hero: String = String::from("Arnold Schwarzeneger");

    let first_name: &str = &action_hero[..6]; // talking from the first byte
    println!("{first_name}");

    let last_name: &str = &action_hero[7..]; // talking until the last byte
    println!("{last_name}");
    
    let full_name: &str = &action_hero[..];
    // let full_name: &String = &ction_hero; it would be the same    
    // be with a different type.
    // the advantage of a slice is where we want to take portion of
    // the string like first_name and last_name examples.

    println!("{full_name}");
}

// What happens if we whant to change first_name later?

// Option 1:
// let mut first_name: String = action_hero[..6].to_string();
// first_name.push_str("y"); // ✅ Now you can modify it and it will not affect to action_hero

// Option 2:
// let action_hero: String = String::from("Arnold Schwarzenegger");
// let mut first_name: &str = &action_hero[..6]; // "Arnold"

// first_name = "Sylvester"; // Now first_name points to a different &str
// println!("{}", first_name); // Prints "Sylvester"

