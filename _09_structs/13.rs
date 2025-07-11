// A trait is a contract that mandates that a type will implement some methods.
// A type that implements either the Display or Debug trait promises that it can be represented as a string

fn main() {
    // There are types that implement the Display trait but not Debug,
    // and vice versa. There are types that implement both traits or neither.
     
    // Arrays do not implement the Display trait (default formatter)
    let values: [&str; 2] = ["hello", "world"];
    
    // Default formatter
    // println!("{}", values); // doesn't work
    
    // Debug formatter
    println!("{:?}", values); // works
    
    // Debug prettier formatter
    println!("{:#?}", values); // works
}