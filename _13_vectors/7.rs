fn main() {
    // specify the number of element the vector can hold
    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!("Capacity: {}, Length: {}", seasons.capacity(), seasons.len());

    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");
    println!("Capacity: {}, Length: {}", seasons.capacity(), seasons.len());

    seasons.push("Summer");  // This push causes reallocation!
    println!("Capacity: {}, Length: {}", seasons.capacity(), seasons.len());

    // IMPORTANT: When we added the 5th element, the vector reallocated!
    // Notice how the capacity jumped from 4 to 8 (doubled).
    // 
    // HYPOTHETICAL DANGER (prevented by Rust's borrowing rules):
    // If there were any other references pointing to the vector's old memory location
    // before reallocation, they would become DANGLING REFERENCES after reallocation.
    // 
    // Example of what Rust prevents:
    // let reference_to_element = &seasons[0];  // Points to old memory location
    // seasons.push("Summer");                  // Reallocation happens here
    // println!("{}", reference_to_element);    // ❌ Would be dangling reference!
    // 
    // Rust's borrowing rules ensure this scenario is impossible:
    // - You can't have a reference AND mutate the vector simultaneously
    // - This prevents dangling references at compile time, not runtime!
}
