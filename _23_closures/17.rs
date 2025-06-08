
// Lecture: Passing in a Function to Fn trait parameter
// A function is an invocable procedure. If the function's logic does
// not involve capturing a value from the external environment, the outer scope,
// we can actually pass a function in as an argument where an FN trait type is expected.

// A closure is a totally valid argument there as well, and the most common example
// we've seen. But I want to demonstrate that function arguments are valid as well.

// That's why the most accurate statement to make is to state that the FN trait
// is not really mandating a closure, it's more so that it is mandating some
// kind of procedure that can be invoked with parentheses.
// A closure is satisfactory, and sometimes a function can be as well.

// A function and a closure are two distinct entities, but they are similar
// in that they can both be called. Both can be used where a Fn trait is expected,
// depending on the capture behavior.

// Many beginners assume Fn is “for closures only,” but in fact it's just
// a trait bound for anything callable with a certain signature

fn execute_thrice<F>(mut procedure: F)
where
    F: FnMut(),
{
    procedure();
    procedure();
    procedure();
}

fn main() {
    let option: Option<Vec<String>> = None;
    // here we have an example where a function (not a closure) is passed in place of a FnOnce.
    let collection = option.unwrap_or_else(Vec::new);
    println!("{:?}", collection);
}