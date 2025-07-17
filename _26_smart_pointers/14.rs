/*
    Summary: Why `Box<T>` vs References for Recursive Data Structures

    When we first tried to create a recursive `LinkedList` type on file 09.rs,
    the compiler suggested several indirection options to break the infinite size cycle:

    Compiler suggestion:
    "insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle"

    1. What is Indirection?
       Indirection means storing an address to a value rather than the value
       itself. Both `Box<T>` and references (`&T`) provide indirection.

    2. Why Not Use References?
       While references would technically solve the infinite size problem,
       they introduce significant complexity:
       - **Lifetime management**: References require explicit lifetime annotations
         to ensure the referenced data lives long enough.
       - **Borrow checker complexity**: You must satisfy borrowing rules, which
         can be challenging with recursive structures.
       - **Ownership ambiguity**: References don't own data, making it unclear
         who is responsible for cleanup.

    3. Why `Box<T>` is Superior:
       - **Owned type**: Fits naturally into Rust's ownership model.
       - **No lifetimes**: No need to worry about lifetime annotations.
       - **Automatic cleanup**: When a `Box<T>` goes out of scope, it automatically
         deallocates its heap data.
       - **Simplicity**: Clean, straightforward API without borrowing complexity.

    The `Box<T>` solution provides the indirection we need while maintaining
    the simplicity and safety of Rust's ownership system, but in the next lesson we
    are going to see how to use &T instead of Box
*/