// Lecture: Invalid lifetimes I
// A dangling reference is a reference to data that no longer exists
fn main() {
    let cities: Vec<String> = vec![
        String::from("Longon"),
        String::from("New York"),
        String::from("Barcelona"),
    ];

    let favorite_cities: &[String] = &cities[0..2];
    drop(cities); // we delete from the heap, so references is on dandling states
    println!("{favorites_cities:?}");
}

// another example
fn main() {
    let some_cities: &[String] = {
        let cities: Vec<String> = vec![
            String::from("Longon"),
            String::from("New York"),
            String::from("Barcelona"),
        ];

        &cities[0..2]; // we trying to return a reference to something is deleted
        // of that ref can not be reach and save it into some_cities variable
    }
}

