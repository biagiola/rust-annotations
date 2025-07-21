use std::collections::HashMap;
use std::collections::hash_map::Entry;
 
fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    
    coffee_pairings.insert("Flat White", "Almond Milk");
    
    let drink: String = String::from("Flat White");
    let milk: String = String::from("Pistachio Milk");
 
    coffee_pairings.insert(&drink, &milk);
    
    println!("{:?}", coffee_pairings);
    
    // in this scenario, the insert is just an overrating when key match
    // another existing one changing its value, so keep that in mind.
 
    // The `entry()` method returns a mutable reference to an `Entry` enum.
    // This `Entry` can be `Occupied` (key exists) or `Vacant` (key doesn't exist),
    // allowing for conditional insertion or modification.

    // The entry() method returns an Entry enum, which has two variants:
    // Occupied(OccupiedEntry): This means the key already exists in the map.
    // Vacant(VacantEntry): This means the key does not exist.
    coffee_pairings.entry(&drink).or_insert(&milk);
    match coffee_pairings.entry("Flat White") {
        Entry::Occupied(_) => {
            println!("Flat White already exists");
        }
        Entry::Vacant(entry) => {
            entry.insert("Pistachio Milk");
        }
    }

    println!("{:?}", coffee_pairings);
    
    // And, the Occupied and Vacant posibillities are actually enums defined 
    // in the rust compiler within the Entry enum
}
