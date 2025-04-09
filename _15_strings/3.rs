fn main() {
    let first_name: String = String::from("Sylvester");
    let last_name: String = String::from("Stallone");

    // let full_name: String = first_name + last_name; // after the plus sign  we need to provide a slice
    // let full_name: String = first_name + &last_name; // but this is fine
    let full_name: String = first_name + " " + &last_name; // also we can add an hardcode space. In background there are a more complex code using the add method, but this is a prettier syntax
    println!("{full_name}");

    println!("{first_name}"); // first_name loose its ownership
}

// quick note:
// The adds method is define in the Rust built-int String module.
// Just add the new slice into the current self that is the first String
// and then return the result.
//
// #[inline]
// fn add(mut self, other: &str) -> String {
//     self.push_str(other);
//     self
// }