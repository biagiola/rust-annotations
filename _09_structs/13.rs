// A trait is a contract that mandates that a type will implement some methods.
// A type that implements either the Display or Debug trait promises that it can be represented as a string
fn main() {
    // there are types that implements the display traits but not debug
    // and viceversa, there are type that implements both traits or neither
     
    // the array do not implement the display trait (default formatter)
    let values: [&str; 2] = ["hello", "world"];
    // default formatter
    // println!("{}", values);
    // debug default
    println!("{:?}", values);
    // debug prettier
    println!("{:#?}", values);
}