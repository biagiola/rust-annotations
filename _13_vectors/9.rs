// How to keep ownership of the data
// Option 1: Clone the Strings

fn main() {
    let pepperoni: String = String::from("Pepperoni");
    let mushroom: String = String::from("Mushroom");
    let sausage: String = String::from("Sausage");

    // Clone the strings - original variables remain usable
    let mut pizza_toppings: Vec<String> = vec![
        pepperoni.clone(),
        mushroom.clone(),
        sausage.clone()
    ];

    pizza_toppings[1] = String::from("Olives");
    println!("{pizza_toppings:#?}");

    // Original variables are still accessible!
    println!("Original pepperoni: {}", pepperoni);
    println!("Original mushroom: {}", mushroom);
    println!("Original sausage: {}", sausage);
}

// Option 1 (clone): Best when you need owned strings in both places