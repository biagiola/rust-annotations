// Nounces of unwrap Method on Result
fn main() {
    let my_result: Result<String, &str> = operation(true);

    let content: &String = match my_result { // borrow it
        Ok(message) => message,
        Err(error) => error.to_string()
    };
    println!("{}", my_result.unwrap());

    // but what happens if instead of have Strings we have &str?
    // string slice does implement the copy trait, because they are immutable references
    // they don't follow the rules of ownership and so they are so cheap and easy to copy

    // if the first param for operation fn is String and the other a &str we will
    // still have an error of partially move error due the String variable where
    // it moves ownership thus there is nothing else to unwrap again.
}

fn operation(great_success: bool) -> Result<String, &'static str> {
    if great_success {
        Ok("Success".to_string()),
    } else {
        Err("Error")
    }
}