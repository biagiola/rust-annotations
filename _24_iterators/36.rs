// Lecture: the reduce method
// The reduce method is similar to fold but it supplies the
// iterator's first element as the started value.

fn main() {
    let earnings = [4, 7, 9, 13];

    // newer compiler-project versions
    // let sum: Option<i32> = earnings
    //     .into_iter()
    //     .reduce(|total, current| &(total + current));
    // println!("{sum:?}");

    // agnostic version
    let sum: Option<i32> = earnings
        .iter()
        .copied()
        .reduce(|total, current| total + current);
    println!("{sum:?}");

}

// In Rust 2018, .into_iter() on an array like [i32; 4] yields &i32 (references).
// In Rust 2021, .into_iter() on an array yields i32 (owned values).
// Your code expects i32, but in Rust 2018, it gets &i32, causing a type mismatch.