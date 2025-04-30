// Lecture: Trait must be in the scope to use its definitions

// lasted lectures we defined our traits in the same file but now
// in our case we need to imported because its definition is somewhere else
use std::ops::Add; // be brint the Add trait from the standard operation module
fn main() {
    let a: i32 = 5;
    let b: i32 = 10;
    let sum = a.add(b);
    println!("{sum}");
}