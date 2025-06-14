fn main() {
    let my_vector = vec![4, 8, 12, 16, 23, 42];
    let my_iterator = my_vector.into_iter();
    println!("{:?}", my_iterator);

    // Prints Option<Some>
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());

    // Prints Option<None>
    println!("{:?}", my_iterator.next());

    // here there's nothing more left
    println!("{:?}", my_iterator);
}