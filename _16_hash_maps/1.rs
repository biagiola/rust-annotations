use std::collections::HashMap;

fn main() {
    let mut menu: HashMap<String, f64> = HashMap::new();

    menu.insert(String::from("Steak"), 29.99);
    menu.insert(String::from("Tuna"), 29.99);
    menu.insert(String::from("Burger"), 29.99);

    println!("{menu:#?}");

    // let mut country_capitals: HashMap<&str, &str> = HashMap::new();
    let mut country_capitals = HashMap::<&str, &str>::new();

    country_capitals.insert("France", "Paris");
    country_capitals.insert("Germany", "Berlin");
    println!("{:#?}", country_capitals);

    // also, the hash map always must to know the type declaring its type
    // or intefrring with a one first initialization value.
}