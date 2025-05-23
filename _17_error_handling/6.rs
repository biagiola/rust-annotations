use std::fs::File;
use std::error::Error;
use std::process;

fn main() {
    let file = match File::open("story.txt") {
        Ok(file) => file,
        Err(message) => {
            eprintln!("Something went wrong reading the file. The error was {message:?}");
            process::exit(1);
        }
    };

    println!("file:?");
}