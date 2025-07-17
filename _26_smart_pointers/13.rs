#[derive(Debug)]
enum LinkedList<T> {
    Empty,
    Node {
        value: T,
        next: Box<LinkedList<T>>,
    }
}

fn main() {
    let im_with_you = LinkedList::Node {
        value: String::from("I'm With You"),
        next: Box::new(LinkedList::Empty),
    };

    let sk8ter_boi = LinkedList::Node {
        value: String::from("Sk8ter Boi"),
        next: Box::new(im_with_you), // transfering ownership here
    };

    let complicated = LinkedList::Node {
        value: String::from("Complicated"),
        next: Box::new(sk8ter_boi), // transfering ownership here
    };

    println!("{:#?}", complicated); // this is the start and prints the whole sequence
}