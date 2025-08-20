/*
The fs::read_dir() function returns a io::Result<ReadDir> enum.
The ReadDir struct implements the Iterator trait.
The iterator yields Result<DirEntry, Error> enums.
The DirEntry struct supports a 'path' method.
The fs::metadata function returns a Metadata struct.
The Metadata struct includes an 'is_file' and 'is_directory' method.
The fs::read_to_string function returns a io::Result<String>
*/
// use std::fs;
// use std::process;
// fn main() {
//     let directory = fs::read_dir("./").unwrap_or_else(|error| {
//         eprintln!("Could not read directory: {error}");
//         process::exit(1);
//     });

//     for entry_result in directory {
//         match entry_result {
//             Ok(entry) => println!("{:?}", entry.path()),
//             Err(error) => {
//                 eprintln!("Could not read entry: {error}");
//             }
//         }
//     }
// }

// use std::fs;
// use std::io;
// fn main() -> Result<()> {
//     for entry_result in fs::read_dir("./")? {
//         if let Ok(entry) = entry_result {
//             println!("{:?}", entry.path());
//         }
//     }

//     Ok(())
// }

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    for entry_result in fs::read_dir("./")? {
        let entry = entry_result?;
        let entry_name = entry.path();
        let metadata = fs::metadata(&entry_name);
        if metadata?.is_file() {
            println!("{entry_name:?} \n-------------");
            let contents = fs::read_to_string(&entry_name);
            println!("{contents:?}");
        }
    }

    Ok(())
}