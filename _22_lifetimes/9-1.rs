// option 1
fn data(collection: &Vec<i32>) -> &Vec<i32> {
    collection
}
fn main() {
    // let cities: Vec = vec!["Longon", "Paris", "Barcelona"];
    let cities: Vec<i32> = vec![1, 2, 3];
    
    let result = data(&cities);
    println!("{result:?}");
}


// option 2
fn data(collection: &[i32]) -> &[i32] {
    collection
}
fn main() {
    // let cities: Vec = vec!["Longon", "Paris", "Barcelona"];
    let cities: Vec<i32> = vec![1, 2, 3];
    
    let result = data(&cities[..3]);
    println!("{result:?}");
}


// option 3
fn data(collection: &[i32]) -> &[i32] {
    &collection[..3]
}
fn main() {
    // let cities: Vec = vec!["Longon", "Paris", "Barcelona"];
    let cities: Vec<i32> = vec![1, 2, 3];
    
    let result = data(&cities);
    println!("{result:?}");
}


// option 4
fn data(collection: &[i32]) -> &[i32] {
    &collection[..3]
}
fn main() {
    // let cities: Vec = vec!["Longon", "Paris", "Barcelona"];
    let cities: Vec<i32> = vec![1, 2, 3];
    
    let result = data(&cities);
    println!("{result:?}");
}


// we cannot return a reference from a owned variable of the function we're calling
// fn data() -> &[i32] {
//     let collection = vec![100, 90, 80];
//     &collection
// }