// A function cannot return a reference to any value
// created inside of its body, or return a reference to
// an owned parameter, because either one of those situations
// would create a dangling reference.

// We absolutely can return references from a function,
// but only when those references point to data that outlives the function call
// and we're talking about in the next lecture.

// If the term outlives is not clear to you, you can put it in other words like:
// Point to a referent that last or lives longer than the referenced one.
// Our "father" variable must to persists beyond, remeains after the reference.

fn create() -> &i32 {
    // It doesnt matter if the data is stored on the heap or the stack
    let age: i32 = 100;
    &age; // age is dropped here
}

// here we're returning something that is going to be destroyed first
// error: expected `&Vec<i32>`, found `()
fn create_slice(item: Vec<i32>) -> &'static Vec<i32> {
    &item;
    // &item[0..2] // also not allowed
}

fn create_number_reference(number: i32) -> &i32 {
    &number; // same as above but with a stack value, not allowed also.
}
// you can return the number itsel, the value, but not the reference

fn main() {
    let values: Vec<i32> = vec![2, 4, 8, 10, 12];
    create_slice(values);
}

// . Local bindings: (age, number) live only for the duration of the function.
// . Owned parameters (items: Vec<i32>) are moved in, and then dropped when the function exits.
// . Returning a &T to any of those would leave a reference pointing at freed memory—which Rust forbids.
