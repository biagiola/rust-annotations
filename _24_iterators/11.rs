// Lecture: Solve a problem with iteration I
// count the characters in a sentance

use std::collections::HashMap;

fn count_characters(text: &str) -> HashMap<char, u32> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        for character in word.chars() {
            let count = counts.entry(character).or_insert(0);
            *count += 1;
        }
    }

    counts
}

fn print_each_word(text: &str) {
    for word in text.split_whitespace() {
        println!("{word}");
    }
}

fn main() {
    // print_each_word("Sally sells sea shells by the sea shore");
    println!("{:?}", count_characters("Sally sells sea shells by the sea shore"));
}
