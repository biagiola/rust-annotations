/*
    Introduction to Smart Pointers

    In Rust, a "smart pointer" is a data structure that acts like a pointer but
    also has additional metadata and capabilities. Unlike regular references, which
    only borrow data, smart pointers often own the data they point to.

    Think of them as wrappers that provide enhanced functionality and safety
    guarantees over traditional raw pointers.

    - Core Characteristics:
      1. Wrapper Structure: They are typically structs that wrap a pointer or
         reference to data stored elsewhere (often on the heap).
      2. Metadata: They store additional information alongside the pointer. For
         example, a `String` stores its capacity and length.
      3. Implemented Traits: They implement traits like `Deref` and `Drop`, which
         allow them to behave like pointers (e.g., be dereferenced with `*`) and
         to define custom cleanup logic when they go out of scope.

    - Analogy:
      A regular reference (`&T`) is like a slip of paper with a house address
      written on it. You can find the house, but that's it.
      A smart pointer is like a detailed property deed. It contains the same
      address but also includes extra data like property tax info, square footage,
      and the rules for transferring ownership.

    The primary benefit of smart pointers is that they manage memory and enforce
    borrowing rules at compile time, providing capabilities that regular
    references lack while maintaining a high level of safety.
*/