// Lecture: The collect method

use std::collections:MahMap;

fn main () {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    let squares = numbers
        .iter()
        .map(|number: &i32| number.pow(2))
        .collect::<HashMap<i32>>();
    
    // of course in the HashMap is not ordered but the key takeaway
    // here is by altering the actual type annotation, not the code
    // but the type annotation, we are able to collect or gather the
    // iterator's values in a totally distinct and different data type
    println!("{squares:?}");
    println!("{numbers:?}"); 
}