fn main() {
    // specify the number of element the vector can hold
    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!("Length: {}, Capacity: {}", seasons.len(), seasons.capacity());

    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");
    println!("Length: {}, Capacity: {}", seasons.len(), seasons.capacity());

    seasons.push("Summer");
    println!("Length: {}, Capacity: {}", seasons.len(), seasons.capacity());

    // if there is any another reference, mutable or immutable, that is still
    // pointing to the old memory location before the expanded version of it
    // so we're having a dangling reference.
}