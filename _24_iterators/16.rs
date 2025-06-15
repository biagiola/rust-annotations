// Lecture: The collect method I
// The collect method exhaust the iterator and gathers
// the resulting values in a new collection type
// So the idea is to set new values from the original vector
// to a new one where we have the squares with the help of iterators
fn main () {
    let numbers = vec![4, 8, 15, 16, 23, 42];
    let squares: Vec<i32> = numbers.iter().map(|number: &i32| number.pow(2)).collect();
    
    // brand new independent vector
    println!("{squares:?}");

    // numbers didn't loose ownership coz iter use references
    println!("{numbers:?}"); 
}