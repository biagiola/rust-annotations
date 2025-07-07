// Will this code compile?
// No. A mutable reference cannot coexist with an immutable reference.

fn main() {
    let mut candy = String::from("M&M's");
    let some_ref = &mut candy;      // Can modify candy
    let another_ref = &candy;       // Can only read candy
    
    // What if this happened simultaneously:
    some_ref.push_str(" chocolate"); // Thread 1: Writing
    println!("{}", another_ref);     // Thread 2: Reading

    // so that's why we cannot have both at the same time.
    // println!("{some_ref} {another_ref}");
}
