// Lecture: implementing the partialEq trait on structs
// this trait establishes equality between two values

// #[derive(PartialEq)] // if we use this, instead of our implementation of partialEq
// it will compare all the fields
struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq for Flight {
    // self it's the flight and Self will be what we're going to compare
    fn eq(&self, other: &Self) -> bool {
        // we're suppose, according to our bussiness logic that,
        // with origin and destination, our structs are equals so...
        self.origin == other.origin && self.destination == other.destination
    }
}

fn main() {
    let a = Flight::new("New York", "London", "11:00am");
    let b = Flight::new("New York", "London", "12:00am");
    let c = Flight::new("New York", "Los Angeles", "15:00am");
    println!("{}", a == b);
    println!("{}", a.eq(&b)); // also in this way, that's used behind the scenes
    println!("{}", a.ne(&b)); // and its negation method as well
}
