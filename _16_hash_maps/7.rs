use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    coffee_pairings.insert("Flat White", "Almond Milk");

    // the copied method maps an Option<&T>, in our case Option<&&str> to an Option<T>,
    // in this case just the Option<&str>, by copying the content of the option.
    let value: Option<&str> = coffee_pairings.get("Flat White").copied();
    println!("{:?}", value); // Some("Almond Milk")

    // but we still wrapped into the option of the enter to the final value
    let value: &str = coffee_pairings.get("Flat White").copied().unwrap();
    println!("{:?}", value); // Almond Milk

    // also, remember that we can provide a fallback message in case of errors
    let value: &str = coffee_pairings.get("Cappuccino").copied().unwrap_or("Unknow milk");
    println!("{:?}", value);

}