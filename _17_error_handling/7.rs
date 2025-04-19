use std::fs::File;
use std::error::Error;
use std::process;
use std::io::stdin;

fn main() {
    println!("Please enter the name of the file you'd like to read:");
    let mut input: String = String::new();
    
    let user_requested_file = stdin().read_line(&mut input);
    
    // we're declaring a error variable and a block code conditionally
    if let Err(error) = user_requested_file { // remember the Err variant is also available from the rust prelude, the same will be to put it like this: Result::Err
        eprintln!("Something went wrong colleting user input. The error was {error}");
        process::exit(1)
    }
    
    // continues if there's no error
    let file = match File::open(&input.trim()) { // also, read_line captures the break line as one more character too, example story.txt/n
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong reading the file. The error was {error:?}");
            process::exit(1); // <- Exit the program so Rust knows we won't continue
        }
    };
    println!("{:#?}", file);
    
}
