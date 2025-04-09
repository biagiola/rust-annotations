fn main() {
    // Concatenation: only the Strings can use this coz
    // they can grow in sizes due
    let mut full_name: String = String::from("Sylvester");
    let last_name: &str = "Stallone";

    full_name.push(' '); // push a single character
    full_name.push_str(last_name);
    println!("{full_name}");
    println!("{last_name}");

    // Quick site note: push_str expects a &str, but if last_name was a String,
    // we can pass to it as a &String, and rust through a feature called deref coercion,
    // rust can convert this ref string to the expected parameter.
    // So &String -> &str. Rust know that there's no danger to doing this because
    // it just create a string slice from a ref string. It creates a borrow of some portion of text stored
    // stored somewhere else. It just so happens that new that text is stored on the heap.
}