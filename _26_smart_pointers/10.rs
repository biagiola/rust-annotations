/*
    Summary: Solving the Recursive Type Problem with `Box<T>`

    This explains how `Box<T>` solves the "recursive type has infinite size"
    error from the previous example.

    1. The Solution:
       Instead of storing `next: LinkedList` directly, we use `next: Box<LinkedList>`.
       This simple change makes the code compile successfully.

    2. Why This Works:
       - A `Box<T>` is a smart pointer that stores data on the heap and holds
         a pointer to that data.
       - The `Box<T>` itself lives on the stack and has a fixed, known size
         (the size of a pointer), regardless of how large the data it points to is.
       - The compiler can now calculate the size of a `LinkedList`:
         * Size of `LinkedList` = Size of the largest variant
         * Size of `Node` = Size of `i32` + Size of `Box<LinkedList>`
         * Size of `Box<LinkedList>` = Size of a pointer (fixed and known)

    3. The Key Insight - Indirection:
       Instead of a node "containing" another node (direct storage), we now have
       a node "pointing to" another node (indirect storage). This breaks the
       infinite cycle because:
       - Each node stores a pointer (fixed size), not another full node.
       - The actual `LinkedList` data that the pointer refers to lives separately
         on the heap.
       - This creates a chain of pointers rather than nested data structures.

    4. Memory Layout:
       - Each `Box<LinkedList>` on the stack is the same size.
       - The actual `LinkedList` nodes are allocated individually on the heap.
       - Nodes are connected by pointers, not by direct containment.
       - This allows for lists of any length without the compiler needing to
         know the size in advance.

    The result is a true linked list: a chain of nodes connected by pointers,
    where each node can point to the next one in the sequence.
*/

#[derive(Debug)]
enum LinkedList {
    Empty,
    Node {
        value: i32,
        next: Box<LinkedList>
    }
}

fn main() {

}