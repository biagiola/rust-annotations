/*
    Summary: Using `Box<T>` for Recursive Data Structures

    One of the most important use cases for the `Box<T>` smart pointer is to
    enable the creation of "recursive data structures".

    1. What is a Recursive Data Structure?
       A recursive data structure is a type that contains itself as part of its
       own definition. A classic example is a linked list, where each node in
       the list contains a value and a link to the next node.

    2. The Problem with Recursive Types in Rust:
       Rust needs to know the exact size of every type at compile time. If you
       try to define a struct that directly contains itself, like:
       
       struct Node {
           value: i32,
           next: Node, // This is the problem
       }

       ...the compiler cannot determine the size. A `Node` contains a `Node`,
       which contains a `Node`, and so on, leading to an infinitely large type.
       The compiler will reject this code.

    3. The `Box<T>` Solution:
       `Box<T>` solves this problem by adding a layer of indirection. A `Box<T>`
       is a pointer, and a pointer has a fixed, known size at compile time,
       regardless of how large the data it points to is.

       By wrapping the recursive part in a `Box`, we break the infinite sizing
       problem for the compiler:

       struct Node {
           value: i32,
           next: Box<Node>, // This works!
       }

       Now, the compiler knows the size of a `Node` is the size of an `i32` plus
       the size of a `Box` (a pointer).

    4. Linked Lists:
       A linked list is a collection of "nodes". Each node contains two things:
       - A value.
       - A link (pointer) to the next node in the sequence.

       The nodes themselves are not necessarily next to each other in memory; they
       are connected only by these links. This structure is naturally recursive
       and is a perfect real-world scenario for using `Box<T>`.
*/