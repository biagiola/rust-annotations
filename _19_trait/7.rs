// Lecture: Traits Bound and conditionally implements methods

// now,  in the Hotel struct , the name field will not be a String anymore
// String implements Display trait for println! and also for format!, but now
// we're using generics, and we are not sure about if the incomming value wil
// implements the Display trait so we need to handle that, specially in the 
// summary method.
use std::collections::HashMap;
use std::fmt::Display;

trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}

trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}

#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>
}

impl<T> Hotel<T> { // because we dont need the display trait be affected on the construct, we separete it from the summarize one
    fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl <T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl<T> Description for Hotel<T> {} // we're using the default definition

#[derive(Debug)]
struct AirBnb {
    host: String,
    guests: Vec<(String, u32)>
}

impl AirBnb {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnb {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

impl Description for AirBnb {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }
}

// multiple trait bound for generic trait bounds example
// the entity must to implement both, not just one.
fn book_for_one_night<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

// using where clauses
fn mix_and_match<T, U>(
    first: &mut T,
    second: &mut U,
    guest: &str,
) where 
    T: Accommodation + Description,
    U: Accommodation
{
    first.book(guest, 1);
    first.get_description();

    second.book(guest,3);
}

fn choose_best_place_to_stay() -> impl Accommodation + Description {
    AirBnb::new("The Luxe")
}

fn main() {
    let hotel1 = Hotel::new(String::from("The Luxe")); // this implement the display trait
    println!("{}", hotel1.summarize()); //

    let hotel2 = Hotel::new(String::from("The Golden Standard")); // this implement the display trait too
    println!("{}", hotel2.summarize());

    let hotel3 = Hotel::new(vec!["The Sweet Escape", "Hilton Edition"]);

    // Error: Missing trait bound and coherence rules
    // this doesn't work, summarize exists but cannot be used for vec!, coz they not implement the display trait
    // method cannot be called on `Hotel<Vec<&str>>` due to unsatisfied trait bounds
    // println!("{}", hotel3.summarize());
}

// The compilation failure you're facing (for hotel3.summarize()) is related to trait bounds not being
// satisfied. This is about coherence rules and method resolution.
