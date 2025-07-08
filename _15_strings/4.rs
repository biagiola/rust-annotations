// format! helps us to create a new string, without taking ownership of the original variables.

fn main() {
    let first_name: String = String::from("Sylvester");
    let last_name: String = String::from("Stallone");

    // let icon: String = format!("{} {}", first_name, last_name); 
    let icon: String = format!("{first_name} {last_name}"); // we're concatenating the two strings through borrowing
    // print our new heap allocated string
    println!("{icon}");

    // they don't lose ownership
    println!("{first_name}");
    println!("{last_name}");

    //TODO: what happens if we use data types like structs or enums?
}