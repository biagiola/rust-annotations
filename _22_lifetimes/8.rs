// Lecture: the only case where we can return references
// on function parameters

// First approach: passing &Vec to our function
// fn select_first_two_elements(items: &Vec<String>) {
//     let selected_items: &[String] = &items[..2];
//     println!("{selected_items}");
// }

// fn main() {
//     let cities: Vec<String> = vec![
//         String::from("London"),
//         String::from("New York"),
//         String::from("Barcelona"),
//     ];
//     select_first_two_elements(&cities);
// }

// Second approach: items is more flexible now,
// receiving a slice to some collection of Strings.
// We can pass a reference to a vector of strings
// or we can also pass a reference to an array of strings.
// Either one will work coz rust will use deref coercion
// to coerce either type to this expected type of some
// fragment of some collection of strings.
fn select_first_two_elements(items: &[String]) {
    let selected_items = &items[..2];
    println!("{selected_items:?}");
}

fn main() {
    let cities: Vec<String> = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    select_first_two_elements(&cities);

    // even with two different lifetimes, the select fn
    // is still able to operate.
    // Also, the borrow checker does not have to worry 
    // coz it does not have to track the lifetime of 'items'
    // parameter after the selec fn ends. There's no return value
    // so no risk for dangling reference
    {
        let coffees: [String; 2] = [
            String::from("Latter"),
            String::from("Mocha"),
        ];
        select_first_two_elements(&coffees);
    }
}
