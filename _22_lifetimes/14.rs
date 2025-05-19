// Lecture: Multiple parameters I
// The lifetime elision rules are insufficient to infer the relationships between the lifetimes of the
// parameters and the lifetime of the return value, and that is because there are multiple possibilities.
// The borrow checker needs our help when a function declares more than one reference parameter.

fn choose_favorite(first: &str, second: &str) -> &str {
    let some_condition: bool = true; // just let's imagine this is not hardcoded

    // and flow goes on
    if some_condition {
        first
    } else {
        second
    }
    // sometimes we're going to return the reference for first sometimes and others the seconds reference
    // so the borrow checker does not know anything about the lifetime of our desire &str and the outlive
    // variable where both must be connected.
}

// that's is another example. If we dont put the 'a the signature will be not define exactly
// in terms of the lifetime annotations for our borrow checker. In this example we only care
// about the existence of the variable that has 'a, first variable in our case
fn choose_favorite<'a>(first: &'a str, second: &str) -> &'a str {
    println!("{second}");
    first
}

fn main() {
    
}
