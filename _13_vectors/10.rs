// How to keep ownership of the data
// Option 2: Use References (Vec<&String>)

fn main() {
    let pepperoni: String = String::from("Pepperoni");
    let mushroom: String = String::from("Mushroom");
    let sausage: String = String::from("Sausage");

    let mut pizza_toppings: Vec<&String> = vec![&pepperoni, &mushroom, &sausage];

    // To modify one element from the vector 
    // we need to create the new topping as a variable first
    let olives: String = String::from("Olives");
    
    // Now we can reference the olives variable
    pizza_toppings[1] = &olives;

    // we cannot do directly this way. It creates a reference to that temporary string
    // that is dropped immediately after the line finishes..
    // pizza_toppings[1] = &String::from("Olives");

    println!("{pizza_toppings:#?}");

    // all the variables are still available!
    println!("{pepperoni}");
    println!("{mushroom}");
    println!("{sausage}");
    // println!("{olives}");
}

// Option 2 (references): Good when the vector is read-only
// Note: With references, you can't easily modify the vector contents like pizza_toppings[1] = ...
// because you'd need another reference with the same lifetime. Lifetime means the time the reference is valid.