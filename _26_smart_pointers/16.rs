#[derive(Debug)]
enum LinkedListUsingReference<'a, T> {
    Empty,
    Node {
        value: T,
        next: &'a LinkedListUsingReference<'a, T>, // Reference needs lifetime 'a
    }
}

fn main() {
    let second_node = LinkedListUsingReference::Node {
        value: 2,
        // Reference to Empty variant - Rust extends its lifetime automatically
        next: &LinkedListUsingReference::Empty,
    };

    let first_node = LinkedListUsingReference::Node {
        value: 1,
        // first_node cannot outlive second_node due to this reference
        next: &second_node,
    };

    // If we drop first_node I can still use second_node but not the other way around.
    // Safe: we can drop the *referencing* node; the *referenced* node is still alive.
    drop(first_node);

    println!("{second_node:#?}");
}
/*
    Lifetime rule: the **referenced** node must outlive the **referencing** node

    • Referenced node  = the value being pointed to (`second_node`).
    • Referencing node = the value that contains the reference (`first_node`).

    For `first_node.next` to be valid the compiler must guarantee:
        lifetime(`second_node`) ≥ lifetime(`first_node`)

    Otherwise `first_node.next` could dangle after `second_node` is dropped, and
    Rust forbids dangling references.

    Practical consequences
    1. Create `second_node` *before* `first_node`, so the value exists when the
       reference is stored.
    2. Do not move, drop, or let `second_node` go out of scope while `first_node`
       (which holds the reference) is still in use.

    The explicit lifetime `'a` in `LinkedListUsingReference<'a, T>` lets the
    borrow-checker enforce these rules at compile time.
*/