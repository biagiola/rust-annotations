// Lecture: static lifetimes

// Static lifetimes ('static) represent data that lives for the entire duration of the program
// - from start to finish. It's the longest possible lifetime in Rust.

// What 'static means
// . When you see &'static str or &'static i32, it means:
// . This reference points to data that will never be deallocated
// . The data exists from program startup until program termination
// . You can safely use this reference anywhere, anytime in your program

fn say_hello() -> &'static str {
    // this reference is data that is guaranteed to live for
    // the existance of the entire program coz it's direclty
    // embebed into the binary of the file.
    "Hello"
}

// another example of static lifetime are in constants, they are available
// and defined at compile time and similarly have a static lifetime
const COUNT: i32 = 400;
fn value() -> &'static i32 {
    &COUNT
}

fn main() {
    let greeting: &str = say_hello();
    println!("{greeting}");

    let count: &i32 = value();
    print!("{count}");
}

