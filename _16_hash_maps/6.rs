use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();

    let drink: String = String::from("Latte");
    let milk: String = String::from("Oat Milk");

    coffee_pairings.insert(&drink, &milk);

    // Directly insert string literals. &str is the native type for string literals.
    coffee_pairings.insert("Flat White", "Almond Milk");

    // let value: &str = coffee_pairings("Cappuchino"); // This line would panic

    // Use the `get` method to safely retrieve a value. It returns an `Option<&Value>`.
    // The type `Option<&&str>` arises because `get` returns a reference to the value,
    // and our value type itself is already a reference (`&str`).
    let value: Option<&&str> = coffee_pairings.get("Cappuchino");
    println!("{:?}", value); // Prints `None` as "Cappuchino" is not in the map.

    // When working with `Option<&T>`, you get a `&&str` (a reference to a reference)
    // because `get` always returns a reference to the value stored in the HashMap,
    // and in this case, the stored value is already an `&str` (a string slice reference).
    // This design avoids taking ownership of the stored `&str` when retrieving it.
}