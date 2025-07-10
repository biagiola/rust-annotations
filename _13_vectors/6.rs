/*
VECTOR CAPACITY AND HEAP REALLOCATION

Understanding how vectors grow and why Rust's borrowing rules prevent data races
*/

// WHAT IS VECTOR CAPACITY?

// CAPACITY: The number of elements that the vector can hold WITHOUT reallocating
// LENGTH: The number of elements currently stored in the vector

// Example: A vector might have:
// - Length: 3 (currently holds 3 elements)
// - Capacity: 8 (can hold 8 elements before needing to reallocate)

// HEAP REALLOCATION PROCESS

// When a vector reaches its maximum capacity and you try to add more elements:
// 
// 1. Rust finds a NEW, LARGER location on the heap (usually 2x the current capacity)
// 2. COPIES all existing elements from the old location to the new location
// 3. DEALLOCATES the old memory location
// 4. Updates the vector's internal pointer to point to the new location

// This process is called "reallocation" and it's expensive because:
// - It requires copying all existing data
// - It involves memory allocation/deallocation
// - The vector's memory address changes completely

// CONNECTION TO BORROWING RULES AND DATA RACES

// Here's WHY Rust's borrowing rules prevent dangerous situations:

// SCENARIO: What could go wrong without borrowing rules?
// 
// 1. Thread A gets a mutable reference to a vector
// 2. Thread B gets another mutable reference to the same vector
// 3. Thread A adds many elements, causing reallocation
// 4. Thread B's reference now points to DEALLOCATED MEMORY!
// 5. Thread B tries to access/modify → CRASH or CORRUPTED DATA

// RUST'S SOLUTION: The "one mutable reference" rule prevents this
// 
// - Only ONE mutable reference can exist at a time
// - If you have a mutable reference, NO other references can exist
// - This ensures that if reallocation happens, there are no dangling references

// MEMORY SAFETY GUARANTEE

// Rust's borrowing rules ensure:
// 
// ✅ NO dangling pointers (references to deallocated memory)
// ✅ NO use-after-free errors
// ✅ NO data races in concurrent code
// ✅ NO memory corruption from simultaneous access
// 
// All of this is checked at COMPILE TIME, not runtime!

// REAL-WORLD EXAMPLE

// Without Rust's rules (hypothetical unsafe code):
// 
// let mut vec = Vec::new();
// let ref1 = &mut vec;      // First mutable reference
// let ref2 = &mut vec;      // Second mutable reference (❌ Not allowed in Rust!)
// 
// // Thread 1 uses ref1 to add 1000 elements → triggers reallocation
// // Thread 2 uses ref2 to access elements → CRASH! (ref2 points to old memory)

// With Rust's rules:
// 
// let mut vec = Vec::new();
// let ref1 = &mut vec;      // Only one mutable reference allowed
// // let ref2 = &mut vec;   // ❌ Compile error: cannot borrow as mutable more than once
// 
// Result: Memory safety guaranteed at compile time!

/*
Vector reallocation is a perfect example of why Rust's borrowing rules exist.
They prevent entire classes of memory safety bugs that plague other systems 
programming languages, all while maintaining zero-cost abstractions.

This is why Rust can be both FAST and SAFE without needing a garbage collector!
*/