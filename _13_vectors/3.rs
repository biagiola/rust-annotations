fn main() {
    let pizza_diameters: Vec<i32> = vec![8, 10, 12, 14];
    // i32 implements the copy trait
    let value: i32 = pizza_diameters[2]; // value is a full copy
    println!("{value}");
    println!("{pizza_diameters:?}"); // so if we modify value variable it is not affected by the original vector

    // but with String ownership is transfered
    let pepperoni: String = String::from("Pepperoni");
    let mushroom: String = String::from("Mushroom");
    let sausage: String = String::from("Sausage");

    // all the previous owners move here now
    let pizza_toppings = vec![pepperoni, mushroom, sausage];

    // add a new element to the vector
    pizza_toppings.push(String::from("Cheese"));
    println!("{pizza_toppings:?}");

    // remove the last element from the vector
    pizza_toppings.pop();
    println!("{pizza_toppings:?}");   
}
