// Lecture: Why iterator can be immutable?
// Here we trying to explain why we don't need mut keyword
// on the for loop variable called number.

fn main() {
    let my_vector = vec![4, 8, 12, 16, 23, 42];
    let mut my_iterator = my_vector.into_iter(); // mut here is useless

    // You made it mut, but you don’t actually mutate my_iterator yourself
    // Rust’s for loop takes ownership of it and uses it internally.
    for number in my_iterator {
        println!("{number}");
    }
}

// Even though iterators need to be mutable to call `.next()`,
// we don’t need to declare `my_iterator` as `mut`.
// That’s because the `for` loop takes ownership of `my_iterator`
// and, behind the scenes, makes it mutable so it can call `.next()`.
// We’re not manually calling `.next()` or modifying `my_iterator`
// ourselves, so there’s no need for `mut` in our code.

// When you do this:
for number in my_iterator {
    println!("{number}");
}

// Rust desugars it to something like:
let mut iter = my_iterator;
loop {
    match iter.next() {
        Some(number) => println!("{number}"),
        None => break,
    }
}
// Here’s the key: Rust itself creates a mutable variable (iter)
// from your my_iterator, and calls .next() on it.
// So even though calling .next() does require a &mut self (meaning it does mutate the iterator),
// that mutation happens inside the desugared loop, not in your code.