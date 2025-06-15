// Lecture: The collect method II

fn main () {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    // tell rust to do your best to figure it out the type
    // will be returned by the collect method, but at the 
    // end is better to use explicit type like the
    // previous code example.
    // let squares: Vec<_> = numbers.iter().map(|number: &i32| number.pow(2)).collect();

    // also we have the turbofish typification style
    let squares = numbers
        .iter()
        .map(|number: &i32| number.pow(2))
        .collect::<Vec<i32>>();
    
    // brand new independent vector
    println!("{squares:?}");

    // numbers didn't loose ownership coz iter use references
    println!("{numbers:?}"); 
}