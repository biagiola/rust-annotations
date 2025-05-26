// Lecture: static lifetimes

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

