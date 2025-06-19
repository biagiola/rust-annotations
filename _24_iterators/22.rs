// Lecture: The filter and find methods III

fn main() {
    let numbers = [10, 13, 23, 2, 8, 9, 6];

    let evens: Vec<i32> = numbers
        .iter() // Iter<'_, i32>
        .filter(|number| *number % 2 == 0) // |number: &&i32| // rust cannot dereference from one more level so we need to use the *
        .copied() // impl Iterator<Item = i32>
        .collect(); // now cannot return Vec<i32> as before but return Vec<&i32>

    println!("{evens:?}"); // here is the actual execution
    println!("{numbers:?}")
}

// copied() method: This is useful when you have an iterator over `&T`, but you
// need an iterator over `T`, in our case T is i32.

// We had an iterator of references and we wanted to use an adapter method in
// this case copied to convert that to be an iterator of the actual original
// values coming from those references.
// This forced another copy of those integers.
// So now we can actually collect the final iterator which is now going to be
// an iterator of just i32 instead of &i32.

// It basically copies all of the elements based on their original references.
// So now we have those copies, and then we can collect from the iterator into a vector.

// The filter() method is just an adapter that gives you a different iterator.
// that method is still lazy right? And the copied() method is another adapter that gives
// you another iterator. Still lazy.
// And in fact, if you got rid of collect() method and just wrote the other ones, the
// compiler would warn you that nothing is actually happening.

// In that scenario, we're just defining three sequential transformation recipes in a
// pipeline, but nothing's happening.
// So it's when we call collect that the actual exhaustion of the final created
// iterator occurs and that's when we gather our vector of i32.
