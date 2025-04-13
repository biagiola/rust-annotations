use std::collections::HashMap;
 
fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    
    coffee_pairings.insert("Flat White", "Almond Milk");
    
    let drink: String = String::from("Flat White");
    let milk: String = String::from("Pistachio Milk");
 
    coffee_pairings.insert(&drink, &milk);
    
    println!("{:?}", coffee_pairings);
    
    // in this scenario, the insert is just an overrating when key match
    // another existing one changing its value, so keep that in mind.
 
    // also, the entry method always return a mut ref to the actual value
    // within the entry. So, if the key value pair exists you're going to get
    // the mutable reference to the existing value, and if you just added the
    // new key value pair, because the key does not exists, we're going to get
    // a mutable reference to the newly added value.
    
    // And, the Occupied and Vacant posibillities are actually enums defined 
    // in the rust compiler within the Entry enum
}