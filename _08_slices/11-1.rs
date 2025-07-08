fn main() {
    let action_hero: String = String::from("Arnold Schwarzenegger");

    
    let mut first_name: &str = &action_hero[..6]; // "Arnold"

    first_name = "Sylvester";
    // Now first_name points to a different &str,
    // does not actually change the original string.

    println!("{}", first_name); // Prints "Sylvester"
    println!("{}", action_hero); // Prints "Arnold Schwarzenegger"
}