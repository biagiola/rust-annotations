// `Option<T>` is a built-in enum that represents a value that could be
// present or absent. It is Rust's primary tool for handling "null" values safely.
// Its two variants are:
// - `Some(T)`: Represents a present value of type T.
// - `None`: Represents an absent value.
//
// Because `Some` and `None` are so common, they are included in the prelude,
// so you can use them directly without the `Option::` prefix.

// This function might find a number, or it might not.
fn find_first_even_number(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            // Found one, return it wrapped in `Some`.
            return Some(num);
        }
    }
    // Found nothing, return `None`.
    None
}

fn main() {
    let some_numbers = vec![1, 3, 5, 6, 7, 9];
    let result = find_first_even_number(&some_numbers);

    // To use the value inside an Option, you must handle both possibilities.
    // `match` is a great way to do this.
    match result {
        Some(number) => println!("Found the first even number: {}", number),
        None => println!("No even numbers were found."),
    }

    // --- Type Annotations ---
    // Usually, the compiler can infer the type of `T` in `Option<T>`.
    let a = Some(32); // Compiler knows this is Option<i32>.

    // If the type is ambiguous, you may need to add an annotation.
    // For example, `None` on its own has no type.
    let b: Option<f64> = None;

    // You can also use the "turbofish" syntax `::<T>` to be explicit.
    // This is sometimes needed but can be redundant if the compiler already knows.
    let c = Some::<i8>(5);
    println!("Explicitly typed Some: {:?}", c);
}
// Note: A similar enum, `Result<T, E>`, is used for handling errors
// that can be recovered from. It has variants `Ok(T)` and `Err(E)`.
