fn main() {
    let mut my_array: [i32; 5] = [10, 15, 20, 25, 30];
    let my_slice: &[i32] = &my_array[2..4];

    println!("{:?}", my_array);
    println!("{:?}", my_slice);

    my_array[0] = 33;
    println!("{:?}", my_array);
    // println!("{:?}", my_slice); // if we print it we'll get an error
    // my_slice was declared as immutable but we change the variable
    // where that slice was pointing on.
    // Rust’s borrow checker ensures that immutable and mutable borrows
    // don’t overlap, but since my_slice isn’t accessed after mutation,
    // there's no conflict.
}