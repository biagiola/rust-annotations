// Lecture: traits for functions parameters
use std::collections::HashMap;

trait Accommadation {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }

    fn book(&mut self, name: &str, nights: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>
}

#[derive(Debug)]
struct AirBnb {
    host: String,
    guest: Vec<(String, u32)>
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new()
        }
    }
}

impl AirBnb {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guest: vec![]
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
        self.guest.push((name.to_string(), nights));
    }
}

fn book_for_one_night(entity: &mut impl Accommadation, guest: &str) {
    // instead of put directly the struct AirBnb or Hotel, we use the trait Accommadation for more flexibility.
    // entity is a mutable reference to some type that implements the Accommadation trait.
    entity.book(guest, 1);
}

fn main() {
    let mut hotel = Hotel::new("The Luxe");
    book_for_one_night(&mut hotel, "Piers");
    println!("{:#?}", hotel);

    let mut air_bnb = AirBnb::new("Peter Schmidt");
    book_for_one_night(&mut air_bnb, "Amanda");
    println!("{:#?}", air_bnb);
}