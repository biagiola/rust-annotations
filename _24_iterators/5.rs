// Lecture: Why iterator can be immutable?

fn main() {
    let my_vector = vec![4, 8, 12, 16, 23, 42];
    let mut my_iterator = my_vector.into_iter(); // mut here is useless

    for number in my_iterator {
        println!("{number}");
    }
}

// Behind the scenes technically speaking, the for loop calls
// into_iter first to get the iterator itself, but then it
// transfers ownership and declares its new ownership in the for loop
// as mutable. So we are technically transferring ownership from a 
// mutable iterator to another mutable owner who makes its ownership
// mutable, implicitly behind the scenes.
// Therefore, the original mutable my iterator variable is never
// actually mutated before its ownership changes to the for loop,
// and thus, because it's not modifying the iterator in any way,
// it does not need the mute keyword.