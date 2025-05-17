// Continuation of the previews lecture

// Now let's consider the scenario where this fn returns
// reference parameter as its return value.
// This is the only situation where we can return a reference 
// parameter of a function, also, notice the return value
// is tide to the type of the parameter, or saying in other sense
// the lifetime of parameter is going to apply similarly to the
// returned reference. 
fn select_first_two_elements(items: &[String]) -> &[String] {
    &items[..2]
}

fn main() {
    let cities: Vec<String> = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    let two_cities: &[String] = select_first_two_elements(&cities);
    println!("{two_cities:?}");

    {
        let coffees: [String; 2] = [
            String::from("Latter"),
            String::from("Mocha"),
        ];
        let two_coffees: &[String] = select_first_two_elements(&coffees);
        println!("{two_coffees:?}")
    }
}

// Remember this code? Here we're creating item as a parameter of the fn
// and that will be drop it when the function ends.
fn create_slice(item: Vec<i32>) -> &[i32] {
    &items;
    // &item[0..2] // also not allowed
}

// Here we also create a parameter called items, but is a reference, so
// it's pointing to some variable on the function caller. And also the
// returned value from this function is the same of the variable from
// the function caller and the reference parameter of select_first_two_elements
// function.
fn select_first_two_elements(items: &[String]) -> &[String] {
    &items[..2]
}
