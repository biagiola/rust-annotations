fn main() {
    let mut sauces: Vec<&str> = vec!["Mayonaise", "Ketchup", "Ranch"];

    while let Some(sauce) = sauces.pop() {
        println!("The next sauce is {sauce}")
    }
    
    // if let Some(sauce) = sauces.pop() {
    //     println!("The next sauce is {sauce}");
    // }
    
    // if let Some(sauce) = sauces.pop() {
    //     println!("The next sauce is {sauce}");
    // }
    
    // if let Some(sauce) = sauces.pop() {
    //     println!("The next sauce is {sauce}");
    // }
}