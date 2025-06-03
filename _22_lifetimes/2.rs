// Lecture: lifetime for references
// The Borrow Checker is the part of the Rust compiler that
// validates that all borrows (i.e., references) are valid.

// The referent is the value being borrowed (dog).
// The reference is the actual borrow (my_pet).
// Also, sometimes the referent is called the lender.
fn main() {
    let dog: String = String::from("Watson"); // referent
    let my_pet: String = &dog; // reference
}

// the lifetime of a reference must be within the lifetime
// of its referent, the reference depends on the existing
// of its referent lifetime.

// another example
fn main() {
    let dog: String = String::from("Watson");

    {
        let my_pet: &String = &dog;
        println!("{}", my_pet);
    }

    println!("{}", dog);

    {
        // we can have another independent block scope
        let my_pet: &String = &dog;
        println!("{}", my_pet);
    }
}

// we can see that the refence lifetime is within the lifetime
// of its referent, and also, we know my_pet lifetime ends
// after the println! of it.
