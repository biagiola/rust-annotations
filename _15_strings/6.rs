use std::io;

fn main() {
    let mut name = String::new();
    println!("What's your name?");
    
    // io::stdin().read_line(&mut name).expect("Failed to collect input from the user");
    // This returns a Result<usize, Error>. If Ok, it contains the number of bytes read.
    // If Err, it contains the error details.
    match io::stdin().read_line(&mut name) {
        Ok(bytes_read) => {
            println!("Hello, {}!", name.trim()); // Remove the newline character (enter) pressed by the user
            println!("(Read {} bytes of input)", bytes_read);
        },
        Err(error) => println!("There was an error reading input: {}", error)
    }
}