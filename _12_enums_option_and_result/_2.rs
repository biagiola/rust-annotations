// The `.get()` method on slices/arrays is a perfect real-world example of `Option`.
// Accessing an index with `[]` (e.g. `instruments[3]`) will panic if it's out of bounds.
// Accessing with `.get()` returns an `Option`, which safely handles this case.

fn main() {
    let instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drum"),
        String::from("Bass"),
    ];

    // `.get(2)` returns `Some(&instruments[2])` because the index is valid.
    // We declare `bass` as mutable so we can reassign it later.
    let mut bass: Option<&String> = instruments.get(2);
    println!("Initially, bass is: {:?}", bass);

    // To create a new `Option` containing a reference to `new_instrument`,
    // you wrap a reference (`&new_instrument`) inside `Some()`.
    let new_instrument = String::from("modified");
    bass = Some(&new_instrument);
    println!("After reassigning, bass is: {:?}", bass);

    // Now, let's see what happens with an invalid index.
    // `.get(3)` handles an out-of-bounds index gracefully by returning `None`.
    let invalid_instrument: Option<&String> = instruments.get(3);
    println!("Getting an invalid index returns: {:?}", invalid_instrument);

    // insturments keep its original values
    println!("{:?}", instruments);
}