use std::collections::HashMap;

fn main() {
    // we will borrow its keys and values.
    let mut coffee_pairings: HashMap<&String, &String> = HashMap::new();

    // Create String instances for the drink and milk.
    let drink: String = String::from("Latte");
    let milk: String = String::from("Oat Milk");

    // Insert references to 'drink' and 'milk' into the HashMap.
    // The HashMap now holds borrows to the data owned by 'drink' and 'milk'.
    coffee_pairings.insert(&drink, &milk);

    println!("{:?}", coffee_pairings);
    println!("{}", coffee_pairings.len());

    // Important considerations when using references as HashMap keys/values:
    // 1. Lifetimes: The borrowed data (like 'drink' and 'milk' here) must outlive
    //    the HashMap or the specific entries within it. If 'drink' or 'milk'
    //    were to go out of scope before 'coffee_pairings', the references
    //    inside the HashMap would become dangling, leading to compile-time errors.
    // 2. Type Consistency: Once you declare a HashMap with specific key and value
    //    types (e.g., `&String` and `&String`), all subsequent insertions
    //    must adhere to those types. You cannot mix `&String` with `String`
    //    for the same key or value type within the same HashMap.
    // 3. Ownership vs. Borrowing: This example demonstrates borrowing. If you
    //    wanted the HashMap to *own* the String data, you would declare its types
    //    as `HashMap<String, String>` and insert `String` values directly
    //    (which would move ownership into the HashMap).
}