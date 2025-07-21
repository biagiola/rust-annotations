fn main() {
    let my_result: Result<String, String> = operation(true);
    
    // let content: String = match my_result { // my_result variable loose ownership and pass it to 'content' variable
    //     Ok(message) => message,             // and because of that we cannot longer call to the unwrap method on content.
    //     Err(error) => error                 // Also neither of both Strings implements the copy trait.
    // };
    let content: &String = match &my_result { // borrow it
        Ok(message) => message,
        Err(error) => error
    };
    println!("{}", content);
}

fn operation(greate_success: bool) -> Result<String, String> {
    if greate_success {
        Ok("Success".to_string())
    } else {
        Err("Error".to_string())
    }
}