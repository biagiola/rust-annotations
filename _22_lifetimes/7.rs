// A function cannot return a reference to any value
// created inside of its body, or return a reference to
// an owned parameter, because either one of those situations
// would create a dangling reference.

fn create() -> &i32 {
    // It doesnt matter if the data is stored on the heap or the stack
    let age: i32 = 100;
    &age;
}

fn create_slice(item: Vec<i32>) -> &[i32] {
    &items;
    // &item[0..2] // also not allowed
}

fn create_number_reference(number: i32) -> &i32 {
    &number; // same as above but with a stack value, not allowed also.
}
// you can return the number itsel, the value, but not the reference
 
fn main() {

}