// Lecture: implementing the partialEq trait on structs
// this trait establishes equality between two values

// PartialEq trait can be used on the Flight struct as usual, #[derive(PartialEq)], but
// the default implementation will compare all the fields. In our case we're just comparing
// origin and destination fields.

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
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

fn main() {
    let a = Flight::new("New York", "London", "11:00AM");
    let b = Flight::new("New York", "London", "12:00AM");
    let c = Flight::new("Barcelona", "Los Angeles", "15:00AM");

    println!("{}", a == b);
    // same as above. this is used behind the scenes
    println!("{}", a.eq(&b));

    // this is the opposite
    println!("{}", a.ne(&b));
}