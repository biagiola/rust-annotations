fn main() {
    let my_vector = vec![4, 8, 12, 16, 23, 42];
    let mut my_iterator = my_vector.into_iter();
    println!("{:?}", my_iterator);

    // Prints Option<Some>
    println!("{:?}", my_iterator.next()); // each iteration
    println!("{:?}", my_iterator.next()); // we're empty out
    println!("{:?}", my_iterator.next()); // our my_iterator variable
    println!("{:?}", my_iterator.next()); // so each iteration is
    println!("{:?}", my_iterator.next()); // mutating.
    println!("{:?}", my_iterator.next()); // here is the last remained value

    // Prints Option<None>
    println!("{:?}", my_iterator.next());

    // here there's nothing more left
    println!("{:?}", my_iterator);
}