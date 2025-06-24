fn main() {
    let mut sushi: &String = &String::from("Salmon");
    println!("Sushi: {}", sushi);
    println!("Whatever message");
    println!("Sushi: {}", sushi); // sushi still lives
    
    let a: i32 = 22;
    println!("Sushi: {}", sushi); // after any operation sushi still lives
    
    sushi = &String::from("another"); // here, sushi is dropped
    // Now, sushi is pointing to a dangling reference 
    println!("Sushi: {}", sushi);

    // SOLUTION
    // let another_sushi = String::from("another"); // ✅ Own the string
    // sushi = &another_sushi; // ✅ Reference to a valid variable
    // println!("Sushi: {}", sushi); // ✅ Works fine

}

// Why Doesn't Rust Extend the Lifetime Like Before?
// Rust only extends the lifetime of a temporary value in these cases:
// 1 . When it’s used in a function argument (println!("{sushi}") ✅).
// 2 . When it’s assigned to a non-reference type (let sushi = String::from("another"); ✅).

// But in our case:
// 1 . sushi is a mutable reference (&String).
// 2 . The temporary "another" is NOT tied to a variable that owns it.
// 3 . Since there's nothing holding ownership of "another", Rust immediately deallocates it.