/*
    Smart Pointers and Ownership

    A defining feature of smart pointers is their relationship with Rust's
    ownership system. Understanding this distinction is key.

    - Regular References (`&T`):
      A regular reference *borrows* data. It points to data owned by some other
      variable. References are never responsible for cleaning up (deallocating)
      the data they point to. When a reference goes out of scope, nothing happens
      to the data it was referring to.

    - Smart Pointers (e.g., `Box<T>`, `String`):
      A smart pointer typically *owns* the data it points to. It is not just
      a borrower; it is the owner. When the smart pointer goes out of scope,
      it is responsible for cleaning up the memory for the data it owns. This
      is handled automatically by the `Drop` trait.

    The "smart" part is how this complexity is managed. Even though a smart
    pointer is a complex wrapper that owns and manages memory, we can interact
    with it as if it were a simple, regular owned value. It hides the difficult
    details of memory management, giving us the power of pointers without the
    manual safety burdens of `unsafe` code.
*/

