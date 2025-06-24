fn main() {
    let languages: [String; 2] = [
        String::from("Rust"),
        String::from("Golang")
    ];

    // String type of the heap does not implement the copy trait
    // so, if it not implements copy trait, we should implement
    // reference, and in that case languages it not longer we the full
    // owner of its values, it'll be partial owner of its elements
    // and that cannot make sense.
    // let first: String = languages[0];

    // first possible solution: use clone method
    let mut first: String = languages[0].clone();
    println!("{first}");

    // clone does not move ownership of the string rather
    // it just creates a full copy. Remember we have manually
    // explicitly Rust when we want to create a copy of heap data.
    
    // The problem with this approach is now we are duplicating
    // this Rust text on the heap.

    first.push_str(" Programming Language");
    println!("{first}");

    println!("{}", languages[0]); // it keeps its value due the clone method used.
}

