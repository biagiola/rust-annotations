fn main() {
    let action_hero: String = String::from("Arnold Schwarzeneger");

    let first_name: &str = &action_hero[..6]; // talking from the first byte
    println!("{first_name}");

    let last_name: &str = &action_hero[7..]; // talking until the last byte
    println!("{last_name}");
    
    // string slice - full sliced string
    let full_name: &str = &action_hero[..];

    // reference to a string - it would be the same
    let full_name: &String = &action_hero;

    // the advantage of a slice is when we want to take a portion of
    // the string like first_name and last_name examples.
    // the advantage of a reference is when we want to take a reference to the whole string.

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

