// The key point is that Rust allows only one implementation of a given trait for a specific pair of types,
// but each pair is considered separately:
// (Flight, Flight) ← one impl
// (BusTrip, BusTrip) ← one impl
// (Flight, BusTrip) ← one impl
// (BusTrip, Flight) ← one impl

// Because these four pairs are distinct, you can derive the “default” equality for the same-type pairs and
// hand-write the mixed-type pairs.

// field-by-field equality for BusTrip == BusTrip
#[derive(Debug, PartialEq)]
struct BusTrip {
    origin: String,
    destination: String,
    time: String,
}

// field-by-field equality for Flight == Flight
#[derive(Debug, PartialEq)]
struct Flight {
    origin: String,
    destination: String,
    time: String,
}

// custom rule for Flight (lhs) == BusTrip (rhs)
impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        self.time == other.time          // your business rule
    }
}

// optional: the other direction BusTrip == Flight
impl PartialEq<Flight> for BusTrip {
    fn eq(&self, other: &Flight) -> bool {
        self.time == other.time
    }
}

fn main() {
    let a = Flight::new("Sao Pablo", "London", "11:00am");
    let b = BusTrip::new("Tokio", "Osaka", "11:00am");
    let c = Flight::new("Sao Pablo", "London", "12:00am");

    println!("{}", flight1 == flight2);   // default, all fields
    println!("{}", bus1 == bus1);         // default, all fields
    println!("{}", flight1 == bus1);      // custom, compares time
    println!("{}", bus1 == flight1);      // works only if the 4th impl exists
}