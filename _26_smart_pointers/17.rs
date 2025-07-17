// Comparation using Box and references
// In comparing `Box<T>` and references (`&'a T`) for creating recursive data
// structures like a linked list, both solutions provide the necessary indirection
// to give the type a known size at compile time. However, they differ
// significantly in complexity and ownership semantics. Using plain references
// introduces lifetime annotations (`'a`), creating a chain of lifetime
// dependencies where each node must outlive the one pointing to it. This can
// make the code harder to reason about as the list grows.
//
// The `Box<T>` approach, while incurring a minor memory overhead, aligns more
// cleanly with Rust's ownership model. When a node is wrapped in a `Box`,
// ownership is transferred, making the head of the list the ultimate owner of
// all subsequent nodes. This simplifies lifetime management, as you only need to
// handle the lifetime of the head node. In the reference-based version, ownership
// is only borrowed, allowing the referenced nodes to exist independently, whereas
// in the `Box` version, the owned node can no longer be accessed directly after
// being moved into the `Box`. This clear ownership transfer makes `Box<T>` often
// the cleaner and more idiomatic choice.

#[derive(Debug)]
enum LinkedListUsingReference<'a, T> {
    Empty,
    Node {
        value: T,
        next: &'a LinkedListUsingReference<'a, T>, // Reference needs lifetime 'a
    }
}

#[derive(Debug)]
enum LinkedListUsingBox<T> {
    Empty,
    Node {
        value: T,
        next: Box::new<LinkedListUsingBox<T>>,
    }
}


fn main() {
    // References
    let second_node = LinkedListUsingReference::Node {
        value: 2,
        next: &LinkedListUsingReference::Empty,
    };

    let first_node = LinkedListUsingReference::Node {
        value: 1,
        next: &second_node,
    };

    // Box
    let second_node = LinkedListUsingBox::Node {
        value: 2,
        next: Box::new(LinkedListUsingBox::Empty)
    };

    let first_node = LinkedListUsingBox::Node {
        value: 1,
        next: Box::new(second_node) // now, the next field fro mthe first_node variable is the new owner
    };

    // println!("{second_node:#?}"); // this does not exists anymore
    println!("{first_node:#?}");
}
