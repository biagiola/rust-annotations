fn main() {
    // same example but now using tuples instead of arrays
    let registrations: (bool, bool, bool) = (true, false, true);
    let first: bool = registrations.0;
    println!("{first} and {registrations:?}");
    
    let languages: (String, String) = (
        String::from("Rust"),
        String::from("Javascript")
    );
    // same error as before so we need to use clone or reference it
    // let first: String = languages.0;
    
    // duplicating the heap data
    let first: String = languages.0.clone();
    
    // borrowing ownership
    let first: &String = &languages.0;
    
    println!("{first} and {languages:?}");
}