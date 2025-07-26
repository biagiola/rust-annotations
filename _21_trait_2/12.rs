// Lecture: the copy trait.
// this is a sub trait from the Clone super trait
// if a type chooses to implement the Copy trait,
// behind the scenes it must to implement the Clone trait

#[derive(Debug, Clone)]
struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Duration {
    fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            hours, minutes, seconds
        }
    }
}

// Copy is a sub trait of the Clone trait, we already derive it on line 6.
// now we are implementing Copy trait more especific.
impl Copy for Duration {}

fn main() {
    let one_hour = Duration::new(1, 0, 0);
    
    // Normally this assignment would MOVE `one_hour`, transferring ownership.
    // Because `Duration` is Copy the move becomes an implicit, bit-for-bit copy,
    // just like copying an `i32`. Both variables now hold the same value but in
    // separate memory locations, so mutating one will not affect the other.
    let another_hour = one_hour;
    // let another_hour = one_hour.clone(); // explicit Clone call (works even without Copy)

    println!("{:?}", one_hour);
    println!("{:?}", another_hour);
}
