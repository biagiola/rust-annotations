// Lecture: The fold method
// the fold method exhaust an iterator to build up and produce
// a single value at the end of the iteration

// .fold(
//     starting_value,
//     | accumulator, current_element | {}
// )

fn main() {
    let earnings = [4, 7, 9, 13];

    // let sum = earnings.into_iter().fold(0, |total, current| {
    let sum = earnings.iter().fold(0, |total, current| {
        println!("Total: {total}, current: {current}");
        total + current
    });

    println!("{sum}");
}