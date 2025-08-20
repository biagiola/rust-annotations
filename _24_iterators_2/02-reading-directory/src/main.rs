/*
The fs::read_dir() function returns a io::Result<ReadDir> enum.
The ReadDir struct implements the Iterator trait.
The iterator yields Result<DirEntry, Error> enums.
The DirEntry struct supports a 'path' method.
The fs::metadata function returns a Metadata struct.
The Metadata struct includes an 'is_file' and 'is_directory' method.
The fs::read_to_string function returns a io::Result<String>
*/
use std::fs;
use std::process;

fn main() {
    let directory = fs::read_dir("./").unwrap_or_else(|error| {
        eprintln!("Could not read directory: {error}");
        process::exit(1);
    });

    for entry_result in directory {
        match entry_result {
            Ok(entry) => println!("{:?}", entry.path()),
            Err(error) => {
                eprintln!("Could not read entry: {error}");
            }
        }
    }
}
