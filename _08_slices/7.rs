fn main() {
    let first_name: &str = {
        let action_hero: &str = "Arnold Schwarzenegger";
        &action_hero[0..6]
    }

    // action_hero is deleted after finish its block scope
    // and first_name is still valid.
    // the chunck of information about the aciton_hero
    // is still available and first_name is using that
    // but just a portion. In other words "Arnold Schwarzenegger"
    // string is embedded in memory from the compiled file.
    println!("{first_name}");

    // Everytime we're declaring a string literal
    // we're declaring a slice.

    // 1. String literals are stored directly in the binary (read-only memory)
    // 2. They are references (&str), pointing to a fixed location
    // 3. Unlike String, which is an owned heap-allocated type, a &str borrows the data
}