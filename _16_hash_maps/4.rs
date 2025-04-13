use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&String, &String> = HashMap::new();
    let drink: String = String::from("Latte");
    let milk: String = String::from("Oat Milk");

    coffee_pairings.insert(&drink, &milk);
    println!("{:?}", coffee_pairings);
    println!("{}", coffee_pairings.len());

    // in this casze we use references insted of the String value directly.
    // we cannot mix between String types anymore.
    // Use references or String but not both for each HashMap register.
    // We can mix between key and values types, but next register must
    // keep the same patter type.
}