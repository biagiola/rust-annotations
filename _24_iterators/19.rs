// Lecture: The map method III
// we can chain iterators and transforms its data in the process
// and then we perform the actual exhaustion.
fn main() {
    let names = [
        String::from("Jimmy"),
        String::from("Cleveland"),
        String::from("Boris"),
    ];

    // let name_lengths = names
    //     .iter()
    //     .map(|name| name.to_lowercase().replace("i", "@@").len())
    //     .collect::<Vec<usize>>();

    let name_lengths = names
        .iter()
        .map(|name| name.to_lowercase())    // &String
        .map(|name| name.replace("i", "@@")) // String
        .map(|name| name.len())              // String
        .collect::<Vec<usize>>();

    println!("{name_lengths:?}");
}