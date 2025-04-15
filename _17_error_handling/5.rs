// opening a file
use std::fs::File;
use std::error::Error;

fn main() {
    // let file: Result<File, Error> = File::open("story.txt"); also, when use Error type we're dealing with handle errors
    let file = File::open("story.txt");
    println!("{:#?}", file);
}