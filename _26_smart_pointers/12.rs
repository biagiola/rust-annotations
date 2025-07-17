#[derive(Debug)]
enum LinkedList<T> {
    Empty,
    Node {
        value: T,
        next: Box<LinkedList<T>>,
    }
}

fn main() {
    let list = LinkedList::Node {
        value: 1.3,
        next: Box::new(LinkedList::Node {
            value: 2.6,
            next: Box::new(LinkedList::Node {
                value: 3.9,
                next: Box::new(LinkedList::Empty)
            })
        })
    };

    println!("{:#?}", list);
}