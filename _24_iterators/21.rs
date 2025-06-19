// Lecture: The filter and find methods II

fn main() {
    let numbers = [10, 13, 23, 2, 8, 9, 6];

    let evens: Vec<&i32> = numbers
        .iter()
        .filter(|number| *number % 2 == 0) // |number: &&i32| // rust cannot dereference from one more level so we need to use the *
        .collect(); // now cannot return Vec<i32> as before but return Vec<&i32>

    println!("{evens:?}"); // here is the actual execution
}
