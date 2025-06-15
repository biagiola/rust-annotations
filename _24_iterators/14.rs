// Lecture: The map method II

fn main() {
    let numbers: Vec<i32> = vec![4, 8, 15, 16, 23, 42];

    // we are using the direct value instead of thier references
    let my_iterator = numbers.into_iter();

    // the traversal of the map hasn't occured yet
    let squares = my_iterator.map(|number: i32| number.pow(2));

    // it occurs right here when we exhaust the squares iterator and
    // it uses an original iterator source as the basis for creating
    // a bran new one, squares iterator.
    for number in squares {
        println!("Square: {number:?}");
    }

    // now that square was exhausted it's not valid to use
    // println!("{squares:?}");
}