fn main() {
    let registrations: (bool, bool, bool) = (true, false, true);
    let first: bool = registrations.0;
    println!("{first} and {registrations:?}");

    let languages: (String, String) = (
        String::from("Rust"),
        String::from("Golang")
    );

    let first: String = languages.0.clone();

    // let first: &String = &languages.0;
    
    println!("{first} and {languages:?}")
}