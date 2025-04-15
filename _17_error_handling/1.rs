// Recoverable errors are errors that we can define code to handle.

// Unrecoverable errors are errors that cause the program to be unable to proceed.
// For example accessing a non existing index on a vector so the program panic or crash.

// The backtrace is the list of files and functions that were running at the point that the error occurred.

fn main() {
    None.unwrap();
}