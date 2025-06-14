// Lecture: The iter method
// The iter method will create an iterator that yields
// immutable references to the collection's elements

fn main() {
    let my_vector = vec![4, 8, 12, 16, 23, 42];
    let my_iterator = my_vector.iter(); // no ownership was taken

    // however, my_iterator lost ownership on the for loop
    // for number in my_vector { // number: &i32
    for number in &my_vector { // short version to avoid to use .iter()
        println!("{number}");
    }

    println!("{my_vector:?}");

    // example with heap data
    let cities: Vec<String> = vec![
        String::from("Phonix"),
        String::from("Dallas")
    ];

    // for city in cities.iter() {
    for city in &cities {
        println!("{city:?}");
    }
    println!("{cities:?}");
}