// A hast set is a collection type that stores, not just unike keys, like a
// normal hash map, but also unique values.
// In some other languages it is called just set.
// This data structure prohibit repeated values.

use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Molly");
    concert_queue.insert("Megan");

    println!("{:?}", concert_queue);
    println!("{:}", concert_queue.len());

    concert_queue.insert("Molly"); // this will be rejected
    println!("{}", concert_queue.remove("Megan")); // true
    println!("{}", concert_queue.remove("Jackson")); // false

    println!("{:?}", concert_queue.get("Molly")); // Some("Molly")
    println!("{:?}", concert_queue.get("Joe")); // None

    // this data structure help use to manage uniqueness and avoid duplicates

    // every data type in rust exists for solve a different problem.
    // an array and tuple exists to solve the problem or order.
    // a hash map exists to solve the problem of association.
    // a hash set exists to solve the problem duplication
}