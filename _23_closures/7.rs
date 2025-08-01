// Lecture: The move keyword
// Rust has a special keyword called move that will force
// the movement of ownership of values into a closure even
// when the closure won't do so by default.
// There are situations where this move keyword is required
// particularly when dealing with multithreaded programs.

fn main() {
    let first_name = String::from("Alice");
    let last_name = String::from("Wonder");

    // the default rust behaviour will not take ownership
    // let capture_string = || {
    //     // since we're just reading, we're not borrowing anything.
    //     println!("{first_name}");
    // };

    // but with the move keyword we can force to take ownership
    // so force the borrow of immutable references of rust.
    let capture_string = move || {
        println!("{first_name} {last_name}");
    };
    // println!("{first_name}"); // all values has been moved

    // the big confution is that if we see the type that rust analyzer
    // give us is just a Fn closure (impl Fn()) and we can indeed
    // invoke it multiple times like a normal Fn does.
    capture_string();
    capture_string();
    capture_string();

    // So, it's ultimately how the moved values are used in the closure,
    // after the movement of ownership that determines how the compiler
    // will infer the type of the closure and which one of those three
    // FN traits it chooses.

    // we cannot implement those values again using the move keyword
    // println!("{first_name} {last_name}");

    // we can wonder why is a FN and why is it capable of running multiple times?
    // Why is it not an FnOnce
    // We can find more discussion about this on:
    // https://www.udemy.com/course/learn-to-code-with-rust/learn/lecture/47564919#overview
}

