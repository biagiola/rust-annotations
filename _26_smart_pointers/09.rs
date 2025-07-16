// This file is meant to be an example of the "recursive type has infinite size"
// error. It will not compile as-is.
//
// The `next` field inside the `Node` variant tries to store a `LinkedList`
// value directly. The compiler cannot determine the size of `LinkedList` because
// it contains itself, leading to a potentially infinite structure.

#[derive(Debug)]
enum LinkedList {
    Empty,
    Node {
        value: i32,
        next: LinkedList
    }
}

fn main() {

}

/*
    Summary: The "Recursive Type has Infinite Size" Error

    This `LinkedList` enum demonstrates a common error when defining recursive
    data structures in Rust.

    1. The Goal:
       A linked list is made of nodes. Each node should contain:
       - A `value` (e.g., an `i32`).
       - A `next` link to the subsequent `LinkedList` element, which could be
         another `Node` or the end of the list (`Empty`).

    2. The Problem:
       The definition `next: LinkedList` tells the compiler that a `Node`
       directly contains another `LinkedList` inside it.

       Rust needs to calculate the exact size of every type at compile time.
       When it tries to calculate the size of `LinkedList`, it sees this:
       - Size of `LinkedList` = Size of `Node`
       - Size of `Node` = Size of `i32` + Size of `LinkedList`
       - ...which means Size of `LinkedList` = Size of `i32` + Size of `LinkedList`

       This creates an infinite loop. The compiler can't determine the size
       because a `Node` can contain a `Node`, which contains a `Node`, and so on
       forever. This is why it produces the error "recursive type has infinite size".

    3. The Solution Hint ("Indirection"):
       The compiler suggests adding "indirection". This means instead of storing
       the next `LinkedList` value directly, we should store a *pointer* to it.
       A pointer (like a `Box`, `&`, etc.) has a fixed, known size.

       By storing a pointer, we break the infinite sizing cycle. The compiler
       can calculate the size of `Node` as:
       - Size of `Node` = Size of `i32` + Size of a Pointer

       This gives `LinkedList` a finite, known size, which satisfies the compiler.
*/