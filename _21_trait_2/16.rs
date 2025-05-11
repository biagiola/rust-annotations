// Lecture: Implementing the Eq trait
// the Eq trait is a child of partialEq
#[derive(PartialEq, Eq)]
struct Flight {
    origin: String,
    destination: String,
    time: String
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

fn main() {
    let a: Flight = Flight::new("New York", "London", "08:00am");
    let b: Flight = Flight::new("New York", "London", "08:00am");
    let c: Flight = Flight::new("New York", "London", "08:00am");

    println!("{}", a == a);
    println!("{}", a == b);
    println!("{}", b == a);
    println!("{}", b == c);
    println!("{}", a == c);

}

// side notes
// . reflexive: a == a , we should compare itself and it should give us true
// . symmetric a == b implies b == a (required by PartialEq as well)
// . transitive: a == b and b == c implies a == c (required by PartialEq as well)

// Most of the time we're going to derive both traits simultaneously because
// the additional requirement of the Eq will apply most of the time to our
// custom types.

// the rust teams choose to separate these two traits because could be rare
// situantion where a type may implement only the PartialEq supertrait.
// For example the f32 and f64, they will implement the PartialEq not the Eq trait.
// and that's because the floats can't promise that htey fulfill the reflexivity
// principle of the equality or Eq subtrait
fn main() {
    let division: f64 = 0.0 / 0.0; // this is f64 but thorws a NaN
    println!("{}", value == value);

    let value: f64 = 3.4;
    println!("{}", value == value);
    println!("{}", division == division); // NaN is not equal to NaN technically, it's like comparing two undefined things

    // in this case for float we cannot guarantee the Eq, so we use just the partialEq
}