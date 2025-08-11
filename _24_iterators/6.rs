// Lecture: The iter method
// The iter method will create an iterator that yields
// immutable references to the collection's elements

fn main() {
    let my_vector = vec![4, 8, 12, 16, 23, 42];
    let my_iterator = my_vector.iter(); // no ownership was taken

     // we need to use the iterator; number: &i32
    for number in my_iterator {
        println!("{number}");
    }
    println!("{my_vector:?}"); // still available
    // however, my_iterator lost ownership on the for loop
    // println!("{my_iterator:?}");

    // short version to avoid to use .iter()
    for number in &my_vector {
        println!("{number}");
    }
    println!("{my_vector:?}"); // still available

    // example with heap data
    let cities: Vec<String> = vec![
        String::from("Phonix"),
        String::from("Dallas")
    ];

    // for city in cities.iter() {
    for city in &cities {
        println!("{city:?}");
    }
    println!("{cities:?}"); // still available
}