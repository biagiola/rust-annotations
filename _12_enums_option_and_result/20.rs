// A **mini-reimplementation** of Rust's built-in `Option<T>`
// ----------------------------------------------------------
// The real `Option` can wrap any type, but for demonstration
// purposes we'll support only `i32` and call our enum `MyOption`.
//
// Deriving `Copy` + `Clone` means every `MyOption` value is *duplicated*
// bit-by-bit on assignment or function calls.  That lets us pass the enum
// around (or return it) without moving ownership—handy for these tiny
// examples where cloning an `i32` is effectively free.
#[derive(Debug, Copy, Clone)]
enum MyOption {
    Some(i32),
    None
}

impl MyOption {
    fn unwrap(self) -> i32 { // `self` is **copied**, so the caller keeps its original value.
        match self {
            MyOption::Some(value) => value,
            // Using `panic!` here mimics the behavior of `Option::unwrap()`
            // when it encounters a `None` at runtime.
            MyOption::None => panic!("called `MyOption::unwrap()` on a `None` value")
        }
    }

    fn unwrap_or(self, fallback_value: i32) -> i32 {
        // If we have a value, return it; otherwise return the caller-supplied
        // default (`fallback_value`).  Exactly like `Option::unwrap_or()`.
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value
        }
    }
}

fn main() {
    let some_option2: MyOption = MyOption::Some(100);
    // `unwrap_or` returns the inner value (100) because this is `Some`.
    println!("{}", some_option2.unwrap_or(13));

    let some_option2: MyOption = MyOption::None;
    // Returns the fallback (13) because this is `None`.
    println!("{}", some_option2.unwrap_or(13))
}