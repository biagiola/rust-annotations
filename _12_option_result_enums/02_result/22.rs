use std::num::ParseIntError;

fn main() {
    let text: &str = "50";
    let text_as_number: Result<i32, ParseIntError> = text.parse::<i32>();
    println!("{:?}", text_as_number);

    // In this case we avoid panic errors
    let text: &str = "Alabama";
    let text_as_number: Result<i32, ParseIntError> = text.parse::<i32>();
    println!("{:?}", text_as_number);
}