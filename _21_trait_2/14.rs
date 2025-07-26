// Lecture: Defining equality for different types (two diffrent structs)

struct BusTrip {
    origin: String,
    destination: String,
    time: String,
}

impl BusTrip {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

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

// now we have to pass the generic Rhs "right hand side"
impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        // now, this time, let's say our bussiness logic is that
        // same time fields is equal to our equaly, just for the
        // sake of different comparation fields depends on us desire
        self.time == other.time
    }
}

fn main() {
    let a = Flight::new("Sao Pablo", "London", "11:00am");
    let b = BusTrip::new("Tokio", "Osaka", "11:00am");
    let c = Flight::new("Sao Pablo", "London", "12:00am");

    println!("{}", a == b);
    println!("{}", c == b);
    
    // Doesn't work. Right hand side is Flight in our definition
    // println!("{}", b == a);

}

// To implement the other way around we'll need this
// impl PartialEq<Flight> for BusTrip { ... }