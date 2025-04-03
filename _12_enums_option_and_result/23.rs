fn main() {
    let result: Result<f64, String> = divide(10.0, 2.0);
    let result: Result<f64, String> = divide(10.0, 0.0);

    match result {
        Ok(calculation) => println!("Result: {}", calculation),
        Err(message) => println!("Error: {}", message)
    }
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}
