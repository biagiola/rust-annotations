// Lecture: Non-lexical lifetimes I
// this was a topic that originally was not in the rust
// implementation but was added over the time.
// The word Lexical means lasting until the end of the block.
// Non-lexical means not lasting until the end of the block.

// The borrow checker treats end of a reference's lifetime
// as the last place it is used; a reference has non-lexical scope.
// When we use reference is better to see where is used for the
// last time instead of its scope to know where ends its lifetime.
fn main() {
    let dog: String = String::from("Watson");
    let my_pet: &String = &dog;
    // 100 lines of code
    println!("{my_pet}"); // <--- after 100 lines lifetime persist
    // but if we didnt have those 100 lines it will end up earlier
    // because it's not used anymore.
}