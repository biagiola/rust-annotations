// Comparison: Box<T> vs References for Recursive Data Structures
// This demonstrates why Box<T> is preferred over references for recursive types.

#[derive(Debug)]
enum LinkedListUsingBox<T> {
    Empty,
    Node {
        value: T,
        next: Box<LinkedListUsingBox<T>>, // Box owns the data, no lifetime issues
    }
}

// Using references requires explicit lifetime annotations
#[derive(Debug)]
enum LinkedListUsingReference<'a, T> {
    Empty,
    Node {
        value: T,
        next: &'a LinkedListUsingReference<'a, T>, // Reference needs lifetime 'a
    }
}

fn main() {
    // --- Reference-based approach ---
    // As in Box example, we wust to create nodes in reverse order.
    // In terms of lifetime, the referenced node must outlive the referencing node
    
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

    println!("{first_node:#?}");
    
    // Note: This approach works but introduces lifetime complexity.
    // The Box approach is simpler and doesn't require lifetime management.
}

/*
    SIDE NOTE: Rust's "Temporaries" Feature

    The line `next: &LinkedListUsingReference::Empty` works because of a Rust
    feature that extends the lifetime of "temporary" values.

    Normally, a temporary value like `Empty` is dropped at the end of the
    statement. However, because a reference is taken to it, Rust ensures the
    temporary's lifetime matches the variable that holds the reference
    (`second_node` in this case).

    This prevents the reference from becoming dangling, but it highlights the
    lifetime complexities that the simpler, ownership-based `Box<T>` approach
    avoids.
*/