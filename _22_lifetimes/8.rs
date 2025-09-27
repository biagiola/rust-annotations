// Lecture: the only cases where we use or return references
// on function parameters

// First approach: passing &Vec to our function but not returning
fn select_first_two_elements(items: &Vec<String>) {
    let selected_items: &[String] = &items[..2];
    println!("{selected_items:?}");
}

fn main() {
    let cities: Vec<String> = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    select_first_two_elements(&cities);
}

// Second approach: in the next example, items is more flexible now,
// receiving a slice to some collection of Strings.
// We can pass a reference to a vector of strings
// or we can also pass a reference to an array of strings.
// Either one will work coz rust will use deref coercion
// to coerce either type to this expected type of some
// fragment of some collection of strings.

// So now, let's focus on the actually topic of the lecture.
// We said we can return references from a fn only when those references
// point to data that outlives the function call.
//
// So we can return reference of items, that at the same time
// it's a reference of another variables on the funcion call,
// for example cities and coffes on main fn. we're going to see concrete
// examples on that topic in the next lecture.
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

    // btw, something to point about lifetimes fn that doesnt have returns.
    // Even with two different lifetimes, the select_first_two_elements fn
    // is still able to operate.
    // The borrow checker does not have to worry
    // coz it does not have to track the lifetime of 'items'
    // parameter after the selec fn ends. There's no return value
    // so no risk for dangling reference
    {
        let coffees: [String; 2] = [String::from("Latter"), String::from("Mocha")];
        select_first_two_elements(&coffees);
    }
}
