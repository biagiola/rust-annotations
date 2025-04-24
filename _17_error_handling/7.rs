use std::fs::File;
use std::error::Error;
use std::process;
use std::io::{stdin, Read};

fn main() {
    // set file name to search
    println!("Please enter the name of the file you'd like to read:");
    let mut input: String = String::new();
    
    let user_requested_file = stdin().read_line(&mut input);
    if let Err(error) = user_requested_file {
        eprintln!("Something went wrong colleting user input. The error was {error}");
        process::exit(1)
    }

    // open the file
    let mut file = match File::open(&input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong opening the file. The error was {error:?}");
            process::exit(1)
        }
    };

    // read its content
    let mut file_content: String = String::new();
    let read_operation = file.read_to_string(&mut file_content);

    if let Err(error)  = read_operation {
        eprintln!("Something went wrong reading the file as a string. The error was {error}");
        process::exit(1)
    }

    println!("{:#?}", file_content)
    
}

// Side notes
// 1. About Read trait import, is a kind of trait we'll discuss more in later sections.
// 2. When we declaring let if statements, we're declaring a error variable and a block code conditionally.
// 3. Remember the Err variant is also available from the rust prelude, the same will be to put it like this: Result::Err
// 4. The mut file variable is more for the internal values of reading a file that will change to keeping track whether the file has been read or not.
// also, read_line captures the break line as one more character too, example story.txt/n
// ----------------
// 5 . let read_operation: Result<usize, Error> = file.read_to_string(&mut file_content);