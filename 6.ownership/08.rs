// The 'drop' builtin function is for deallocates memory on the heap.
// The variable must be the owner of heap allocated memory.
// Stack memory doesn't work with the 'drop' fn at all.
fn main() {
    let person: String::from("David");
    // we don't need it but we can invoke drop fn
    drop(person);
}