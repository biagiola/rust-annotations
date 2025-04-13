use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
 
    let drink: String = String::from("Latte");
    let milk: String = String::from("Oat Milk");
 
    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat White", "Almond Milk");

    // this will panic for a not found value
    // let value: &str = coffee_pairings("Cappuchino"); // error: no entry found for key

    // to avoid non found keys, we use the get method
    let value: Option<&&str> = coffee_pairings.get("Cappuchino");
    println!("{:?}", value);

    // we have the &str as the type for our variable coffee_pairings, &str are slices,
    // that technically are references, and above of that, for not taking any ownership
    // of that slice reference (&str), we grab the reference to that address pointing to 
    // the real text. So we have double amperson symbol to do that. All of this because
    // no take any ownership on the Option enum.
}