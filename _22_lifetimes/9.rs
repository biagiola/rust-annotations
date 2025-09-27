// Continuation of the previews lecture

// Now let's consider the scenario where this fn returns a reference
// parameter as its return value.
//
// This is the only situation where we can return a reference
// parameter of a function, just because it's tied to the variable
// from the function caller.
//
// Also, the lifetime of items parameter is going to apply similarly to
// the returned reference, the slice or portion of items.

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
        let coffees: [String; 2] = [String::from("Latter"), String::from("Mocha")];
        let two_coffees: &[String] = select_first_two_elements(&coffees);
        println!("{two_coffees:?}")
    }
}

// Remember this code? Here we're creating items as a parameter of the fn
// and that will be drop it when the function ends.
// fn create_slice(items: Vec<i32>) -> &[i32] {
//     &items;
// }

// The 'items' parameter is a borrow a reference to data owned by the main caller.
// The function does not take ownership; it only gets temporary access.
// The returned value is a new reference slice that points to the first two
// elements of that exact same data, ensuring the slice's lifetime is tied to the original data.
//
// Lifetime Elision: Since the function has one input lifetime (&[String]),
// the compiler automatically assigns the same lifetime to the output reference.
// The signature is treated as:
// fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String].
