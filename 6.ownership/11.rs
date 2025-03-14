fn main() {
    // String: A dynamically allocated, growable mutable piece of text stored on the heap.
    // let mut s = String::from("Hello");
    // s.push_str(", world!");
    // println!("{}", s);

    // &String ("ref String"): A reference to a String, typically used for borrowing.
    // Avoid using &String when &str is enough (e.g., in function parameters).
    // &str can accept both String and string literals (&'static str).
    let s = String::from("Rust");
    print_string(&s); // we borrow s to the invocking fn
    println!("{s}");

    // str: A string slice (not commonly used by itself in Rust code).
    // &str ("ref str"): A reference to a string slice, typically used to reference hardcoded text (string literals) or substrings.
}

fn print_string(s: &String) {
    println!("{}", s);
    // after finish this scope, the fn deallocates s
}