fn main() {
    let mut pizza_diameters: Vec<i32> = vec![8, 12, 14];
    pizza_diameters.push(16); // adds to the end
    pizza_diameters.push(18);
    pizza_diameters.insert(0, 4); // position, value

    println!("{pizza_diameters:?}");

    let last_value: Option<i32> = pizza_diameters.pop(); // removes the last one
    println!("{last_value:?}");

    let third_diameter = pizza_diameters.remove(2); // panic at runtime if index doesn't exists
    println!("Third: {third_diameter}")

    // match last_pizza_diameter {
    //     Option::Some(value) => value,
    //     Option::None => 0
    // }
}