// A generic is a placeholder for a concrete type. It's like a function
// parameter, but for a type (like i32, String, or bool) instead of a concrete value.

// Without generics, we would need to write a specific function for each type.
fn identity_i32(value: i32) -> i32 {
    value
}

fn identity_bool(value: bool) -> bool {
    value
}

// These two functions are identical except for their types.
// With a generic type `T`, we can write a single function that works for any type.
fn identity<T>(value: T) -> T {
    value
}

fn main() {
    // We can call our generic `identity` function with different types.
    let num = identity(10); // The compiler infers that T is i32.
    let is_true = identity(true); // The compiler infers that T is bool.
    let message = identity("hello"); // The compiler infers that T is &str.

    println!("The number is: {}", num);
    println!("The boolean is: {}", is_true);
    println!("The message is: {}", message);

    // You can also be explicit about the type if needed, using the "turbofish" syntax.
    let explicit_num = identity::<i32>(20);
    println!("The explicit number is: {}", explicit_num);
}