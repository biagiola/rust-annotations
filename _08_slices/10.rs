fn main() {
    // the length of a string slice refers to a count of its bytes,
    // not its characters. 
    let pizza_emoji: &str = "🍕";
    println!("{}", pizza_emoji.len());

    // error: not valid byte boundry
    let emoji_slice: &str = &pizza_emoji[0..2];
    // the full emoji takes 4 bytes and we cannot make
    // and slice of it. we should take 4 byte at least.
    println!("{}", emoji_slice);
}