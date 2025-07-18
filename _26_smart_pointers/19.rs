#[derive(Debug)]
enum LinkedListUsingBox<T> {
    Empty,
    Node {
        value: T,
        next: Box<LinkedListUsingBox<T>>
    }
}

fn create_list() -> LinkedListUsingBox<i32> {
    let second_node = LinkedListUsingBox::Node {
        value: 2,
        next: Box::new(LinkedListUsingBox::Empty),
    };

    let first_node = LinkedListUsingBox::Node {
        value: 1,
        next: Box::new(second_node), // here next is the new owner and we dont care about internal variable existance
    };

    first_node
}

fn main() {
    let list = create_list();
    println!("{list:#?}");
}