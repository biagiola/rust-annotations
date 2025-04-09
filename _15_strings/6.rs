use std::io;

fn main() {
    let mut name = String::new();
    println!("What it's your name?");
    
    // io::stdin().read_line(&mut name).expect("Failed to collect input from the user");
    // this throws us an Result with Ok or Err, if it's okay
    // it will return the number of bytes of the input.
    match io::stdin().read_line(&mut name) {
        Ok(_) => println!("Hello, {}.", name.trim()), // in the _ variable we have the number of types of the user's input. And also remove the print line enter user using the trim.
        Err(message) => println!("There was an error: {}", message)
    }
    
    
}