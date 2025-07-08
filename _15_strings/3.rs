fn main() {
    let first_name: String = String::from("Sylvester");
    let last_name: String = String::from("Stallone");

    // let full_name: String = first_name + last_name; // after the plus sign we need to provide a slice
    // let full_name: String = first_name + &last_name; // if that is a String, we need to specify the reference
    let full_name: String = first_name + " " + &last_name; // also we can add a hardcoded space. In background there is more complex code using the add method, but this is prettier syntax
    println!("{full_name}");

    // println!("{first_name}"); // ❌ COMPILATION ERROR! first_name was moved on line 7
    // first_name loses its ownership when used in the + operation above
}

// Quick note:
// The add method is defined in the Rust built-in String module.
// It takes ownership of self (the first String), appends the slice,
// and returns the result as a new String.
//
// #[inline]
// fn add(mut self, other: &str) -> String {
//     self.push_str(other);
//     self
// }