// opening a file
use std::fs::File;
use std::error::Error;

fn main() {
    // When use Error type we're dealing with handle errors
    let file = File::open("story.txt"); 
    
    // let file: Result<File, Error> = File::open("story.txt");
    // we can omit the type, but we know that File return a Result enum
    println!("{:#?}", file);
}