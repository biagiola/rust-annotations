// Lecture: The map method I
// An adapter method is one that transforms an iterator
// into another iterator based on some logic.
// The map method applies a closure to each item in an
// iterator to arrive at a new iterator of values.

fn main() {
    let numbers: Vec<i32> = vec![4, 8, 15, 16, 23, 42];

    // we're borrow references to no took ownership
    let my_iterator = numbers.iter();
    let squares = my_iterator.map(|number: &i32| number.pow(2));

    // nobody took ownerhsip of numbers
    println!("Numbers: {numbers:?}");

    // squares took ownership over my_iterator
    // println!("{my_iterator:?}");

    // pow is not apply jet until we begin to exhaust the variable
    // we have defined the clousure, but it will apply when we loop it.
    println!("Squares: {squares:?}");

    // here, we begin to make use of the pow operation
    for number in squares {
        println!("Square: {number:?}");
    }

    // now that square was exhausted it's not valid to use
    // println!("{squares:?}");
}