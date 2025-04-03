fn main() {
    let pepperoni: String = String::from("Pepperoni");
    let mushroom: String = String::from("Mushroom");
    let sausage: String = String::from("Sausage");

    let pizza_toppings: Vec<String> = vec![pepperoni, mushroom, sausage];
    // The get method extract a vector element by index position.
    // It returns an Option enum.

    let topping: Option<&String> = pizza_toppings.get(2);
    // println!("{topping:?}");

    match topping {
        Some(value) => println!("The topping is {value}"),
        None => println!("No value at that index position")
    }
}