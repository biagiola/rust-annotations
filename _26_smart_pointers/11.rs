#[derive(Debug)]
enum LinkedList {
    Empty,
    Node {
        value: i32,
        next: Box<LinkedList>, // Box provides indirection to break infinite recursion
    }
}

fn main() {
    // Example 1: Single-node list containing just the value 1
    // let list = LinkedList::Node {
    //     value: 1,
    //     next: Box::new(LinkedList::Empty) // Box::new() wraps the Empty variant
    // };

    // Example 2: Multi-node list: 1 -> 2 -> 3 -> Empty
    // Each node points to the next via Box<LinkedList>
    let list = LinkedList::Node {
        value: 1,
        next: Box::new(LinkedList::Node {
            value: 2,
            next: Box::new(LinkedList::Node {
                value: 3,
                next: Box::new(LinkedList::Empty) // Final node points to Empty
            })
        })
    };

    // Print the entire linked list structure
    println!("{:#?}", list);
}