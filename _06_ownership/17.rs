fn main() {
    let mut sushi: &String = &String::from("Salmon");
    println!("Sushi: {}", sushi);
    println!("Whatever message");
    println!("Sushi: {}", sushi); // sushi still lives
    
    let _a: i32 = 22; // more code and...
    println!("Sushi: {}", sushi); // sushi still lives
    
    sushi = &String::from("another"); // but here, sushi is dropped
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


// DETAILED BREAKDOWN OF LINE 10: sushi = &String::from("another");
//
// Step-by-step execution:
// • Step 1: String::from("another") creates a new heap-allocated String
//   - Contains the text "another"
//   - This is a temporary value (no variable owns it)
//
// • Step 2: & takes a reference to the temporary String
//   - Creates a reference pointing to the "another" String on the heap
//   - sushi now points to this new heap location
//
// • Step 3: Assignment completes
//   - sushi is now pointing to the "another" String
//   - The old "Salmon" String gets dropped (nothing references it anymore)
//
// • Step 4: End of statement (CRITICAL PART!)
//   - The temporary String "another" goes out of scope
//   - Rust drops the "another" String because it's a temporary with no owner
//   - sushi is now pointing to freed memory (dangling reference)
//
// Key insights:
// • You're assigning a REFERENCE to the "another" string, not the string itself
// • The actual String object "another" is created on the heap
// • No variable owns the actual String object "another"
// • Since it's a temporary with no owner, Rust immediately drops it
// • sushi is left holding a reference to freed memory
//
// Visual timeline:
// Before line 10: sushi → "Salmon" (heap)
// During line 10: sushi → "another" (heap) + "Salmon" dropped
// After line 10:  sushi → [freed memory] + "another" dropped
//
// The problem: Reference assignment vs. value assignment
// The issue isn't with the reference assignment itself, but with the fact that
// the actual String object being referenced has no owner and gets dropped immediately.
