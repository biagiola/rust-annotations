use std::fs::File;
use std::error::Error;
use std::process;
use std::io::{stdin, Read}; // import our Read trait; trait is something it'll be discuss in deep details later sections

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
    // this mut file variable is more for the internal values of reading a file
    // that will change to keeping track wheather the file has been read etc.
    let mut file = match File::open(&input.trim()) { // also, read_line captures the break line as one more character too, example story.txt/n
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong opening the file. The error was {error:?}");
            process::exit(1); // <- Exit the program so Rust knows we won't continue
        }
    };

    let mut file_content: String = String::new();
    // let read_operation: Result<usize, Error> = file.read_to_string(&mut file_content);
    let read_operation = file.read_to_string(&mut file_content);

    if let Err(error)  = read_operation {
        eprintln!("Something went wrong reading the file as a string. The error was {error}");
        process::exit(1)
    }

    println!("{:#?}", file_content)
    
}
