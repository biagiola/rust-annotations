// Lecture: Closure introduction
// This section introduces closures, which are unnamed (anonymous)
// functions, often called lambdas in other languages.
// They're useful for short, one-time procedures that don’t need a full function definition.

// Closures are a core part of functional programming, a paradigm where functions are treated
// like values—they can be passed around, returned, assigned to variables, or stored in collections.
// Rust supports this approach in parts of its design.

fn main() {
    let multipler = 5;

    fn multiply_by(value: i32) -> i32 {
        value * multipler // multiply_by function cannot access to the outside multipler variable
    }

    println!("{}", multiply_by(5));
}

// fn another_func() {
//     // nested code is one example where we can use closures
//     multiply_by(3);
// }