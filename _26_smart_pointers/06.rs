/*
    Example: `String` is a Smart Pointer

    We have been using a common smart pointer all along: `String`. It serves as a
    perfect practical example of the concepts.

    A `String` manages a block of memory on the heap to store its text data.
    It fits the definition of a smart pointer perfectly:

    1. It acts like a pointer:
       It manages a pointer to the heap-allocated, UTF-8 encoded text. However,
       this complexity is completely hidden from us. We don't see or interact
       with the raw pointer.

    2. It has additional metadata:
       A `String` stores two pieces of metadata alongside its internal pointer:
       - `length`: The number of bytes the string is currently using.
       - `capacity`: The total number of bytes allocated for it on the heap.
       This allows a `String` to be grown efficiently without reallocating memory
       on every single character push.

    3. It owns its data and manages its lifecycle:
       A `String` owns the heap-allocated buffer containing its text. When a
       `String` variable goes out of scope, its `Drop` implementation is called,
       which deallocates the heap memory. We never have to manage this ourselves.

    The key takeaway is that `String` provides a simple, safe, and ergonomic API
    for what is actually a complex memory management task. This is the goal of
    all smart pointers: to provide powerful capabilities in a safe and easy-to-use
    wrapper.
*/


