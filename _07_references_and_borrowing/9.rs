fn main() {
    let mut languages: [String; 2] = [
        String::from("Rust"),
        String::from("Golang")
    ];

    // second option: use reference, borrowing ownership
    // where languages array is still the actual owner of 
    // all the elements of the array, including the first.
    let new_value: &mut String = &mut languages[0];

    // println!("{new_value} {languages:?}"); // we cannot print languages right now coz it's borrowed.
    new_value.push_str(" Programming Language"); // and the borrowed will affect to the original owner.

    println!("{new_value}");
    println!("{languages:?}");

    // also we can modify languages array directly
    languages[0].push_str(" is awesome");
    println!("{languages:?}");
}