// Lecture: Lifetimes in structs
// So far, all of the structs that we've defined have only
// held types that either implement the copy trait.
// In those situations, the struct is the owner of its fields,
// and the fields are the owners of their values.

// And in general, that's actually a good design pattern because
// you do not have to worry about references and lifetimes.
// It's usually pretty easy to reason about your structs if
// all of the values are owned by the struct or a type that
// is an owned type, like a string.

// Now, you might be sacrificing some bit of speed by not
// using references and sharing memory, but the trade off
// is usually worth it.
// That been said, I do want to show an example of what the
// code would look like when your struct stores references.

// struct TrainSystem {
//     name: String,
// }

#[derive(Debug)]
struct TrainSystem<'a> {
    name: &'a str,
    // The struct must be deallocated first before the string slice is deallocated
    // to avoid dangling reference on the fields for the struct.
}

fn main() {
    // let nj_transit: TrainSystem = TrainSystem {
    //     name: String::from("NF Transit"),
    // };

    // println!("{nj_transit}");

    let name: String = String::from("NJ Transit");
    let nj_transit: TrainSystem<'_> = TrainSystem { name: &name }; // TODO: Why '_ works and why the undeclared lifetime error if we use a name like 'a

    println!("{nj_transit:#?}");
}
