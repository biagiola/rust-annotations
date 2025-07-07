fn main() {
    let first_name: &str = {
        let action_hero: &str = "Arnold Schwarzenegger";
        &action_hero[0..6]
    };

    // The variable action_hero goes out of scope after finishes its block scope,
    // but the string literal data "Arnold Schwarzenegger" has 'static lifetime and remains in memory.
    // Variable first_name still points to valid static data.
    // The chunk of information about the action_hero is still available and first_name
    // is using that but just a portion.
    // In other words "Arnold Schwarzenegger" string is embedded in memory from the compiled file.
    println!("{first_name}");

    // Every time we're declaring a string literal we're declaring a slice.

    // 1. String literals are stored directly in the binary (read-only memory)
    // 2. They are references (&str), pointing to a fixed location
    // 3. Unlike String, which is an owned heap-allocated type, a &str borrows the data
}