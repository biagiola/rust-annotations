// Lecture: Solve a problem with iteration I
// count the words in a sentance

use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<&str, u32> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();

    for word in words {
        // grab the value by the word if exists otherwise insert it
        // with a default value of zero.
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    counts
}

fn main() {
    println!("{:?}", count_words("Sally sells sea shells by the sea shore"));
}
