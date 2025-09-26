// Lecture: Non-lexical lifetimes II

fn main() {
    let mut data = vec!['a', 'b', 'c']; // create a vector

    let slice = &mut data[..];

    // slice is passed into a function, transfering ownership
    // and no longer live, but its referent is still valid.
    operate_char_array(slice);
    operate_char_vector(&mut data);

    data.push('d'); // and before instruction non-lexical lifetimes on rust
    data.push('e'); // this will throws errors, but now, slice lifetime is
    data.push('f'); // ended but it not affect to the original variable lifetime
    println!("{:?}", data);
}

fn operate_char_array(word: &mut [char]) {
    // we cannot push to a array, it's fixed size
    println!("whatever, {:?}", word);
}

fn operate_char_vector(words: &mut Vec<char>) {
    words.push('z');
    println!("whatever, {:?}", words);
}

// side notes.
// . data[..] creates a slice that references all elements of the vector
// . &mut takes a mutable reference to that slice
// . No values are copied - slice is a reference pointing to the same memory as data
// . The slice contains references to the original data, not copies of the values.
