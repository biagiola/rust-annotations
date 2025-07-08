fn main() {
    // Concatenation: only Strings can use this coz
    // they can grow in size due to the heap allocation.
    let mut full_name: String = String::from("Sylvester");
    let last_name: &str = "Stallone";

    full_name.push(' '); // push a single character
    full_name.push_str(last_name);
    println!("{full_name}");
    println!("{last_name}");

    // Quick side note: push_str expects a &str as parameter, but if last_name was a String,
    // we can pass it as a &String, and Rust, through a feature called deref coercion,
    // can convert this &String to the expected parameter, &str.
    // &String -> &str. Rust knows that there's no danger to doing this because
    // it just creates a string slice that points to the existing String data on the heap.
    // No new text is created - it's just a different view of the same data.
}