// The Result Enum
// Option is more for absent or present

// The Result Option doesn't implement the display trait but it does implement the debug trait
// Result Option is also included into the Rust prelude
fn main() {
    let ok: Result<i8, &str> = Result::Ok(5);
    println!("{ok:?}");

    let disaster: Result<i32, &str> = Result::Err("Something went wrong");
    println!("{disaster:?}");
    println!("{:?}", disaster);

    let ok: Result<i8, &str> = Ok(5);
    let disaster: Result<i32, &str> = Err("Something went wrong");
}