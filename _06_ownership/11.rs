fn main() {
    // String: A dynamically allocated, growable mutable piece of text stored on the heap.
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    // &String ("ref String"): A reference to a String, typically used for borrowing.
    // Avoid using &String when &str is enough (e.g., in function parameters).
    // &str can accept both String and string literals (&'static str).
    let s = String::from("Rust");
    print_string(&s); // we borrow s to the invoking fn
    println!("{s}");

    // str: A string slice (not commonly used by itself in Rust code).
    // &str ("ref str"): A reference to a string slice, typically used to reference hardcoded text (string literals) or substrings.
}

// fn print_string(s: &String) { // we normally avoid this
fn print_string(s: &str) {
    println!("{}", s);
    // after this scope ends, the reference goes out of scope
    // (the original string data remains owned by the caller).
    // when this function ends, the parameter 's' (which is a reference) 
    // gets deallocated from the function's stack frame.
    // the original string data remains owned by the caller.
}