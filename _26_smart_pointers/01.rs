// A raw pointer is a variable that stores a memory address without any of Rust's
// safety checks or guarantees. When we use them, we are opting out of the compiler's
// protections that we get with regular references.

// Raw pointers are necessary for certain low-level tasks, such as interfacing with
// C libraries (FFI) or building custom data structures, where Rust's strict
// borrowing rules are too restrictive.

// --- IMMUTABLE RAW POINTERS ---
// A raw pointer can be created from a reference.
// Syntax 1: Using `&raw const` to explicitly create an immutable raw pointer.
//
// Syntax 2: Casting a reference to a raw pointer.
// By providing the type annotation, Rust will coerce the regular immutable
// reference (`&my_string`) into an immutable raw pointer.
//
// --- MUTABLE RAW POINTERS ---
// Syntax 1: Using `&raw mut` to explicitly create a mutable raw pointer.
//
// Syntax 2: Casting a mutable reference. By providing the type annotation,
// Rust will coerce the regular mutable reference (`&mut my_string`) into a
// mutable raw pointer.

// Examples:
// let pointer: *const String = &raw const my_string;  // immutable raw pointer using raw pointer syntax
// let pointer: *const String = &my_string;            // immutable raw pointer using reference syntax
// let pointer: *mut String = &raw mut my_string;      // mutable raw pointer using raw pointer syntax
// let pointer: *mut String = &mut my_string;          // mutable raw pointer using reference syntax

// Unlike Rust's safe references, the compiler allows the creation of multiple
// mutable raw pointers to the same data. The danger isn't in creating them,
// but in *using* (dereferencing) them.

fn main() {
    let mut sushi: String = String::from("Yellowtail");

    let sushi_raw_pointer_1: *const String = &raw const sushi;
    let sushi_raw_pointer_2: *const String = &sushi;

    let sushi_raw_pointer_mut_1: *mut String = &raw mut sushi;
    let sushi_raw_pointer_mut_2: *mut String = &mut sushi;


    // --- DEREFERENCING RAW POINTERS (THE UNSAFE PART) ---
    // To dereference a raw pointer, you must use an `unsafe` block.
    // The `unsafe` keyword doesn't make the code inside it unsafe; it's an
    // explicit acknowledgement from the programmer that they are taking full
    // responsibility for maintaining memory safety because the compiler can no
    // longer provide its guarantees for the operations within the block.

    // This is a "dangling pointer" scenario. We are explicitly deallocating the
    // memory for the `sushi` String. After this line, our raw pointers are
    // pointing to invalid memory.
    drop(sushi);

    unsafe {
        // Attempting to dereference the dangling pointer. This gives us UNDEFINED BEHAVIOR.
        // - The program might crash.
        // - It might print garbage data.
        // - It might appear to work correctly by chance.
        // The outcome is unpredictable. Here, it will likely print nothing or garbage
        // depending on the operating system we are running.
        println!("sushi_raw_pointer_1 is: {}", *sushi_raw_pointer_1);

        // This demonstrates the danger: we can have multiple co-existing mutable
        // pointers, which is strictly forbidden with safe references to prevent data races.
        println!("sushi_raw_pointer_mut_1 is: {}", *sushi_raw_pointer_mut_1);
        println!("sushi_raw_pointer_mut_2 is: {}", *sushi_raw_pointer_mut_2);
    }

    // Key takeaway:
    // Regular references are safe but restrictive.
    // Raw pointers are flexible but require `unsafe` and manual safety management.
    // Smart pointers (which we'll see next) provide safer abstractions over this power.
}