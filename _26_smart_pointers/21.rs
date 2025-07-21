use std::cmp::Ordering;

#[derive(Debug)]
enum BinarySearchTree {
    Empty,
    Node {
        value: i32,
        left: Box<BinarySearchTree>,
        right: Box<BinarySearchTree>,
    },
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree::Empty
    }

    fn insert(&mut self, new_value: i32) {
        match self {
            BinarySearchTree::Empty => {
                // * does not mean the address like &, instead the access to the
                // actual value on that address in memory. So, we are saying that the value
                // in that memory will be our values of the node we wanna set
                *self = BinarySearchTree::Node {
                    value: new_value,
                    left: Box::new(BinarySearchTree::Empty),
                    right: Box::new(BinarySearchTree::Empty),
                }
            },
            BinarySearchTree::Node { value, left, right } => {
                match new_value.cmp(value) {
                    Ordering::Equal => (), // we ignore duplicates, sake of simplicity.
                    Ordering::Less => left.insert(new_value),
                    Ordering::Greater => right.insert(new_value),
                }
            },
        }
    }

    fn contains(&self, target: i32) -> bool {
        match self {
            BinarySearchTree::Empty => false,
            BinarySearchTree::Node { value, left, right } => {
                Ordering::Equal => true,
                Ordering::Less => left.contains(target),
                Ordering::Greater => right.contains(target),
            }
        }
    }
}

fn main() {
    let mut tree = BinarySearchTree::new();
    tree.insert(5);
    tree.insert(2);
    tree.insert(4);
    tree.insert(8);
    tree.insert(13);
    tree.insert(7);

    println!("{tree:#?}");

    println!("{}", tree.contains(13));
}