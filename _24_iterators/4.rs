// Lecture: for loop with iterators
// the for loop call the .next() method automatically
// to keep unwrapping the value in the Some variant
// and when it reach the None varianty the for loop terminates
// and the iterator is exhausted.

fn main() {
    let my_vector = vec![4, 8, 12, 16, 23, 42];
    let my_iterator = my_vector.into_iter(); // Explicitly create an iterator
    
    // for number in my_iterator { ... }: The for loop in Rust is designed to work with iterators.
    // When you iterate over my_iterator, the loop repeatedly calls the next() method on the iterator
    // until it returns None. Each Some(value) returned by next() is assigned to number.
    for number in my_iterator { // Consume the iterator
        println!("{number}");
    }

    // println!("{my_iterator:?}"); // it was exausted by the for loop
}

fn main() {
    let my_vector = vec![4, 8, 12, 16, 23, 42];

    // Relies on the for loop's syntactic sugar to implicitly call into_iter() on the collection.
    // This is more concise and common when you simply want to iterate over and consume the collection's elements directly.
    for number in my_vector { // Implicitly creates and consumes an iterator
        println!("{number}");
    }

    // println!("{my_iterator:?}"); // it was exausted by the for loop
}

// side notes
// 1 . my_iterator tooks ownership of my_vector but
// the for loop also take ownership of my_iterator.
// 2 . the for loop also calls the into_iter method
// on whatever collection you pass, for example vec!
// that does not implement the iterable by default.
// So that means that we can delete the line 9 and 
// the program will compile it without any problem.
// 3 . The for loop support either the original collection
// type, like the vector, or and iterator made from the original
// collection type. It will also prevent unnecesarry copies from
// being made because, if it gets an iterator, it's not going
// to create a copy of that iterator, instead it's going to
// return itself.

// So to summarize
// if the for loop gets a vector or another original collection type,
// rust will create an iterator by calling the into iter method and
// then exhaust that. Conversely, if the for loop gets an iterator,
// the for loop will call into iter on that once again, but that will
// give back the original iterator upon which the method was invoked,
// sparing the need to create any duplicate iterators.