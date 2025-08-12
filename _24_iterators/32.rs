// Lecture: Partition method
// partition method groups and returns the values for which
// the closure returns true and for which the closure returns false

fn main() {
    let numbers = [4, 8, 15, 16, 23, 42];

    let groups: (Vec<i32>, Vec<i32>) = numbers
        // .into_iter()
        .iter()
        .partition(|number| *number % 2 == 0);
    println!("{groups:?}");

    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers
        // .into_iter()
        .iter()
        .partition(|number| *number % 2 == 0);
    println!("Evens: {evens:?}");
    println!("Odds: {evens:?}");
}

// if we use into_iter the compiler will ask to change to iter(). Why the warning?
// In Rust 2015/2018, calling .into_iter() on an array like [i32; 6] actually iterates over references (&i32), not owned values.
// In Rust 2021, .into_iter() on an array will iterate over owned values (i32), not references.
// This change was made for consistency, but it means that code written for older editions might behave differently in Rust 2021.
// The compiler warns you because your code might break or change meaning if you upgrade to Rust 2021.