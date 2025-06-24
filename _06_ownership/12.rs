
// String: A dynamically allocated, growable mutable piece of text stored on the heap.
// &String ("ref String"): A reference to a String, typically used for borrowing.
// str: A string slice (not commonly used by itself in Rust code).
// &str ("ref str"): A reference to a string slice, typically used to reference hardcoded text (string literals) or substrings.

fn main() {
    let ice_cream: &str = "Cookies and Crem";
    // this reference points neither the stack or the heap
    // it's reference to the hardcoded text that was embedded into
    // the executable file that is then loaded into the memory.

    println!("{}", ice_cream)

    // Try the extension on VSCode: Hex Editor for see the binaries
}