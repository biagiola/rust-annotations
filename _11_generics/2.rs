// This file demonstrates how a single generic function can be used with many
// different concrete types. It also explains the compile-time process of
// "monomorphization" that makes this possible and efficient.

fn identity<T>(value: T) -> T {
    value
}

#[derive(Debug)]
struct DeliSandwich {}

fn main() {
    // We can call the same generic function with various concrete types.
    println!("{}", identity(5));
    println!("{}", identity(5.99));
    println!("{}", identity("5.99"));
    println!("{}", identity("hello"));
    println!("{}", identity(String::from("Example")));
    println!("{:?}", identity(DeliSandwich {}));
}

// MONOMORPHIZATION
//
// The comment below explains what the compiler does "under the hood".
//
// At compile time, Rust looks at all the places a generic function is called
// and generates a non-generic version of that function for each concrete type
// it's called with. This process is called monomorphization.
//
// For the `main` function above, the compiler would generate something like this:
/*
    fn identity_i32(value: i32) -> i32 { value }
    fn identity_f64(value: f64) -> f64 { value }
    fn identity_str(value: &'static str) -> &'static str { value }
    fn identity_string(value: String) -> String { value }
    fn identity_deli_sandwich(value: DeliSandwich) -> DeliSandwich { value }
*/
//
// Because this happens at compile time, there is no runtime performance cost
// for using generics compared to writing specialized functions by hand.