use std::collections::HashMap;

fn main() {
    // use ref str
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();

    let drink: String = String::from("Latte");
    let milk: String = String::from("Oat Milk");

    // reference string insertion as before
    coffee_pairings.insert(&drink, &milk);

    // hardcoded string insertion also
    coffee_pairings.insert("Flat White", "Almond Milk");

    println!("{:?}", coffee_pairings);
    println!("{}", coffee_pairings.len());


    // Also, in some situations rust can convert &String -> &str where it
    // is need it through the deref coercion.
    // With the the type &str and the deref coercion we can deal both scenarios.
}