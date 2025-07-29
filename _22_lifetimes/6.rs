// Lecture: Invalid lifetimes II
// A dangling reference is a reference to data that no longer exists
fn main() {
    let cities: Vec<String> = vec![
        String::from("Longon"),
        String::from("New York"),
        String::from("Barcelona"),
    ];

    let places: Vec<String> = cities; // we transfering ownership here
    let favorite_cities: &[String] = &cities[0..2]; // so cities does not exists anymore
    println!("{favorite_cities:?}");
}

// another similar situation
fn main() {
    let cities: Vec<String> = vec![
        String::from("Longon"),
        String::from("New York"),
        String::from("Barcelona"),
    ];

    let favorite_cities: &[String] = &cities[0..2]; // we borrow cities to favorite_cities
    let places: Vec<String> = cities; // cannot move out cities coz it is borrowed on line 23
    println!("{favorite_cities:?}"); // borrowed variable used here again
    // at line 25 there's a dangling reference, coz favorite_cities was a reference to cities
    // variable and that's is no longer the owner. The cities lifetime as ended and favorite_cities
    // lifetime is out living the lifetime of cities vector.

    // But we can make this work if we move the println of favorite_cities one line up so...
}

// we ended up with this code:
fn main() {
    let cities: Vec<String> = vec![
        String::from("Longon"),
        String::from("New York"),
        String::from("Barcelona"),
    ];

    let favorite_cities: &[String] = &cities[0..2];
    println!("{favorite_cities:?}");
    let places: Vec<String> = cities;
    // rust must to guarantee there's not dangling references
    // and also with the ability of non-lexical lifetime, favorite_cities
    // lifetime is ended when we used it last, and after rust deallocated
    // avoid the dangling reference in the previous example.
    // Now the reference's lifetime is completely contained within the 
    // referent's lifetime.

}