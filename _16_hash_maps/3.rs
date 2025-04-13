use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<String, String> = HashMap::new();
    let drink: String = String::from("Latte");
    let milk: String = String::from("Oat Milk");

    coffee_pairings.insert(drink, milk);
    println!("{:?}", coffee_pairings);
    println!("{}", coffee_pairings.len());

    println!("{:?}", milk); // this has been deallocated
    // we can use clone for this Strings copy otherwise use references
}