// Lecture: The filter and find methods I
// The filter method is an example of an adaptar that extracts a
// subset of values that satisfy a condition. Pass a closure that
// returns true for the elements to keep and false for the elements
// to exclude.

fn main() {
    let numbers = [10, 13, 23, 2, 8, 9, 6];

    // into_iter will attempt to take ownership over the original integers
    // but integers implement the copy trait, so actually will not move
    // ownership away from numbers variable, it's just going to create a 
    // full copy of the original integers. This only happens with primitives
    // data type like integers, floats or booleans.
    // Also regarless your iteration method, filter will use references. Rust
    // doesn't want to accidentally move ownership when it's using the original
    // values.
    // So even we're using into_iter that takes ownership this will compile anyways
    // with a warning to change .into_iter() for .iter() and again, the filter
    // method always use references anyways.
    let evens: Vec<i32> = numbers
        .into_iter()
        .filter(|number| *number % 2 == 0) // |number: &i32|
        .map(|n| *n)            // .copied() is the same here
        .collect::<Vec<i32>>(); // remember also that iterator are lazy, meaning that we're designating the 
        // transformation logic, but not executing or exhausting yet.

    println!("{evens:?}"); // here is the actual execution

    // // example of heap data and iterate with into_iter that normally takes ownership
    // let names = ["David", "Dana", "Oscar", "Jules"];
    // let members = names.into_iter().filter(|name| name.len() > 5).collect();
    // println!("{members}");
}

// For arrays like numbers variable, in Rust before 1.53, calling .into_iter()
// would not take ownership of the elements — you'd get &i32.
// But since Rust 1.53, arrays of length 1 to 32 now implement IntoIterator directly,
// which yields owned items (i32, not &i32)
// . i32 is Copy, so even if ownership is "taken", the values are copied, and numbers
// can still be used later because it's not actually moved in a meaningful way.
// . If you tried this with something non-Copy (like String), then yes, ownership
// would be taken, and you couldn’t use the array again unless it was moved or cloned.