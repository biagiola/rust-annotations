use std::ops::Add;

fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    // At compile time Rust generates two concrete versions of the function
    // This transformation is called monomorphisation—Rust’s way of giving you zero-cost generics.
    // i32 version (Add<Output = i32> is implemented for i32)
    let integer_sum: i32 = add_two_numbers(1, 2);
    println!("{}", integer_sum);

    // f64 version (Add<Output = f64> is implemented for f64)
    let float_sum: f64 = add_two_numbers(1.5, 2.4);
    println!("{}", float_sum);
}

