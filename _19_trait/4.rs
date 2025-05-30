// Lecture: Multiple Trait Bound
use std::collections::HashMap;

trait Accommadation {
    fn book(&mut self, name: &str, nights: u32);
}

trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>
}

#[derive(Debug)]
struct AirBnb {
    host: String,
    guests: Vec<(String, u32)>
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl AirBnb {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![]
        }
    }
}

impl Accommadation for Hotel {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl Accommadation for AirBnb {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

impl Description for Hotel {} // in this way we're just using the default fn there.

impl Description for AirBnb {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }
}

//  example of multiple trait bund.
//  the entity must to implement both traits, not just one.
fn book_for_one_night<T: Accommadation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

// now we split the trait into Accommadation and Description, our first argument needs
// to implements both traits
fn mix_and_match(
    first: &mut (impl Accommadation + Description),
    second: &mut impl Accommadation,
    guest: &str
) {
    first.book(guest, 1);
    first.get_description();

    second.book(guest, 3);
    // second.get_description(guest, 3); // the compiler cannot guarantee this is going to work
}

fn main() {
    let mut hotel = Hotel::new("The Luxe");
    let mut air_bnb = AirBnb::new("Peter Schmidt");
    mix_and_match(&mut hotel, &mut air_bnb, "Piers");
}