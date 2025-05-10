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

// Copy is in the prelude, so we dont need to import it
impl Copy for Duration {}

fn main() {
    let one_hour = Duration::new(1, 0, 0);
    // let another_hour = one_hour.clone() // we can also impl the copy trait
    let another_hour = one_hour; // we can also impl the copy trait

    // so we're not transfering ownership but creating a full copy
    println!("{:?}", one_hour);
    println!("{:?}", another_hour);
}

// site note, with stack data type we have copy trait or full copy by default
// let a = 5;
// let b = a;