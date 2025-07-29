// Lecture: Non-lexical lifetimes II

fn main() {
    let mut data = vec!['a', 'b', 'c']; // create a vector

    let slice = &mut data[..];

    capitalize(slice); // slice is passed into a function, transfering ownership
                       // and no longer live

    data.push('d'); // and before instruction non-lexical lifetimes on rust
    data.push('e'); // this will throws errors, but now, slice lifetime is    
    data.push('f'); // ended but it not affect to the original variable lifetime
}

fn capitalize(word: &mut [char]) {
    println!("whatever, {:?}", word);
}

// side notes.
// . data[..] creates a slice that references all elements of the vector
// . &mut takes a mutable reference to that slice
// . No values are copied - slice is a reference pointing to the same memory as data
// . The slice contains references to the original data, not copies of the values.