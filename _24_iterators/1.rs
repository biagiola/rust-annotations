// Lecture: imperative vs declarative iterations
// Full declarative and rust idiomatic way, more consice
// and reduction in chance or errors.

// full imperative
fn main() {
    let numbers = vec![4,8, 15, 16, 23, 42];

    let mut current_index: usize = 0;
    let final_index: usize = number.len - 1;

    loop {
        if current_index > final_index {
            break;
        }

        println!("{}", numbers[current_index]);
        current_index += 1;
    }
}

// dont use break, but we need still to use the greater
// comparation and also update the counter
fn main() {
    let numbers = vec![4,8, 15, 16, 23, 42];

    let mut current_index: usize = 0;
    let final_index: usize = number.len - 1;

    while current_index > final_index {
        println!("{}", numbers[current_index]);
        current_index += 1;
    }
}

// full declarative
fn main() {
    let numbers = vec![4,8, 15, 16, 23, 42];

    for number in numbers {
        println!("{number}");
    }
}

