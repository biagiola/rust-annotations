fn main() {
    let pepperoni: String = String::from("Pepperoni");
    let mushroom: String = String::from("Mushroom");
    let sausage: String = String::from("Sausage");

    let mut pizza_toppings: Vec<String> = vec![pepperoni, mushroom, sausage];

    pizza_toppings[1] = String::from("Olives");
    println!("{pizza_toppings:#?}");

    // Partially transfer ownership error due the String does not implement the copy trait
    // let target_topping: String = pizza_toppings[2];
    let target_topping: &String = &pizza_toppings[2]; // but we can use its reference
    // target_topping.push_str(" and Meatballs");
    println!("{target_topping}");

    // we cannot have two mutable borrows at the same time
    // let another_topping: &mut String = &mut pizza_toppings[2];

    

    // but here it's okay due the ended lifetime of the target_topping variable
    // TODO: we cannot have multiple mut references, so check this code
    // let another_topping: &mut String = &mut pizza_toppings[2];
    // let another_one: &mut String = &mut pizza_toppings[2];
    println!("{pizza_toppings:#?}");
}