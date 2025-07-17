// This is an example where we need to use Box over references

// This example demonstrates a key limitation of using plain references (`&'a T`)
// for recursive data structures, especially when dealing with function
// boundaries. When attempting to construct and return a reference-based linked
// list from a function, the compiler will prevent it. The issue stems from
// Rust's ownership and lifetime rules.The `next` field of a node only *borrows*
// a reference to the subsequent node. If the list is built inside a function,
// each node is a local variable owned by that function. When the function
// completes, its local variables are deallocated. Returning a list in this state
// would mean returning a node containing a dangling reference to memory that is
// no longer valid, which Rust's borrow checker correctly forbids. This scenario
// highlights a case where simple borrowing is insufficient because ownership must
// be transferred out of the function, a problem that `Box<T>` is designed to solve.

#[derive(Debug)]
enum LinkedListUsingReference<'a, T> {
    Empty,
    Node {
        value: T,
        next: &'a LinkedListUsingReference<'a, T>
    }
}

#[derive(Debug)]
enum LinkedListUsingBox<'a, T> {
    Empty,
    Node {
        value: T,
        next: &'a LinkedListUsingBox<'a, T>
    }
}

fn create_list<'a>() -> LinkedListUsingReference<'a, i32> {
    let second_node = LinkedListUsingReference::Node {
        value: 2,
        next: LinkedListUsingReference::Empty,
    }

    let first_node = LinkedListUsingReference::Node {
        value: 1,
        next: second_node,
    }

    first_node
}

fn main() {
    let result = create_list();
}