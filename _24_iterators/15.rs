// Lecture: The map method III

fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    for number in numbers.into_iter().map(|number: i32| number.pow(2)) {
        println!("Square: {number}");
    }
}

// side notes: You can take your started piece of data like numbers in our case
// and just call map directly, but that will not work. In rust, we first have to
// transform it into an iterator with a method like iter, into_iter or iter_mut
// in order to then call map on that to get a brand new iterator and then to
// exhaust that final iterator.

// another wrong miss understand could be that, numbers.iter() is equavalent
// to a borrow, so we're going to remove the iter() call on numbers and try to
// put directly a ref in front of numbers without the iter() method and think
// that it will use references, but we need to be explicit to work with iterator
// so we need to call the iter() or into_iter() to map it correctly
for number in &numbers.map(|number: i32| number.pow(2)) {
    println!("Square: {number}");
}