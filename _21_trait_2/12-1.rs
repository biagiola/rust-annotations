// Lecture: the copy trait.

// we can also implement the Copy trait directly on the derive of the struct
#[derive(Debug, Clone, Copy)]
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

// we dont need this anymore
// impl Copy for Duration {}

fn main() {
    let one_hour = Duration::new(1, 0, 0);
    let another_hour = one_hour;

    println!("{:?}", one_hour);
    println!("{:?}", another_hour);
}
