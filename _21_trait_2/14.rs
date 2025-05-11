// Lecture: Defining equality for different types
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
    let a = Flight::new("New York", "London", "11:00am");
    let b = BusTrip::new("New York", "London", "11:00am");

    println!("{}", a == b);

    // and this is not going to work. remember, we're using
    // the a.eq(b) behind, so the left right hand side was
    // declared for a, that is a Flight, so a got the eq function not the other
    // away around. BusTrip is not implementing the PartialEq method
    // println!("{}", a == b);
    
}

// side notes: we can add the Partial equal to the BusTrip and make the comparation
// b == a
// impl Partial<Flight> for BusTrip() {
//    fn eq(&self, other: &Flight) -> bool {
//         self.time == other.time // we can use others fields of course
//     }
// }; 
// remember to assign the generic, otherwise we're going to make a BusTrip comparation
// against another BusTrip struct.
// Also we cannot compare BusTrip with another BusTrip direclty, to do so, we need to
// implement the PartialEq with our the generics on BusTrip.

// and if you want to keep simple, and all the comparation are sutable, we can use
// the #[derive(PartialEq)] in our struct

