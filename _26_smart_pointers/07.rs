/*
    Summary: The `Box<T>` Smart Pointer

    `Box<T>` is the simplest smart pointer. Its primary purpose is to allocate
    any value of type `T` onto the heap.

    1. What It Is:
       - A `Box<T>` is an owned wrapper around a raw pointer that points to
         heap-allocated data.
       - It allows data that would normally live on the stack (like a single `i32`)
         to be stored on the heap instead.
       - The `Box<T>` struct itself lives on the stack. Its size is fixed and
         known at compile time because it only contains a pointer to the heap.
         The data on the heap, however, can be of any size.

    2. How It Behaves:
       - Ownership: `Box<T>` is an owned type. It follows Rust's standard
         ownership and move semantics. It does not implement the `Copy` trait.
         When a `Box<T>` goes out of scope, it is dropped, and the heap memory
         it points to is deallocated automatically.
       - Dereferencing: It implements the `Deref` trait, allowing you to use the
         dereference operator (`*`) to access the underlying data on the heap,
         just like a regular reference or raw pointer.
       - Ergonomics: It provides a safe and simple API (`Box::new()`) to allocate
         memory on the heap without needing `unsafe` code, lifetimes, or manual
         memory management. It hides all the complexity.

    3. Main Use Cases:
       - Storing a large amount of data on the heap to keep the stack frame small.
       - Transferring ownership of data without copying large amounts of data.
       - Storing a type whose size cannot be known at compile time (a "trait object").
       - Creating recursive data structures (like a linked list) where a struct
         needs to contain another instance of itself.
*/
fn main() {
    // `Box::new(100)` allocates memory on the heap and places `100` in it.
    // `my_box` is the smart pointer on the stack that points to this data.
    let my_box: Box<i32> = Box::new(100);

    // To access the data on the heap, we use the dereference operator `*`.
    // This follows the pointer and gets the value.
    println!("{}", *my_box);

    // `Box<T>` implements the `Display` and `Debug` traits (if `T` does).
    // Rust performs "deref coercion" automatically, so we don't have to `*` manually.
    println!("{}", my_box);
    println!("{:?}", my_box);
    println!("{my_box}");

    // `Box` is an owned type and doesn't implement `Copy`.
    // Assigning `my_box` to `your_box` moves ownership.
    let your_box: Box<i32> = my_box;
    println!("{your_box}");

    // After the move, `my_box` is no longer valid and cannot be used.
    // The following line would cause a compile-time error:
    // println!("{}", my_box); // error[E0382]: use of moved value: `my_box`

    // When `your_box` goes out of scope at the end of this function, its
    // destructor runs, and the heap memory it points to is deallocated.
}