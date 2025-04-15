fn main() {
    let a: i32 = 1;
    panic!("Something went wrong. Finish execution.");
    // variable 'a' is clean up when the program finish, and that process
    // it is called unwinding.
    // Something we says panic in a little bit misleading because it's actually
    // a pretty orderly safe clean up process. It's not a crash in the tradicional
    // computer science sense.
}