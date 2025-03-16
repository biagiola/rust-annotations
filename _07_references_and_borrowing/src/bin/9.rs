fn main() {
    let mut languages: [String; 2] = [
        String::from("Rust"),
        String::from("Golang")
    ];

    // second option: use reference, borrowing ownership
    // where languages array is still the actual owner of 
    // all the elements of the array, including the first.
    let new_value: &String = &languages[0];
    println!("{new_value} {languages:?}");

    // this push_str doesn not make sense even if it
    // a mutable variable coz new_value is not a raw
    // string but a reference to a raw string or String in this case.
    new_value.push_str(" Programming Language");

    // but we can modify languages array
    languages[0].push_str(" Programming Language");
    println!("{languages:?}");
}