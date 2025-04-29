// Lecture: Trait Bound Syntax
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

impl Accommadation for Hotel {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
struct AirBnb {
    host: String,
    guests: Vec<(String, u32)>
}

impl AirBnb {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![]
        }
    }
}

impl Accommadation for AirBnb {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}


// this is example of Trait bound
fn book_for_one_night<T: Accommadation>(entity: &mut T, guest: &str) {
    // instead of put AirBnb or Hotel, we use this for more flexibility
    // entity is a mutable reference to some type that implements the accommadation
    entity.book(guest, 1);
}

// first and second can be any type of the accommadation traits. They are not obligated to be the same type exactly.
// this is an example of the previous lesson without using the trait bound
fn mix_and_match(first: &mut impl Accommadation, second: &mut impl Accommadation, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 3);
}

// this will be mix_and_match using the trait bound
// fn mix_and_match<T: Accommadation>(first: &mut T, second: &mut T, guest: &str) {
//     first.book(guest, 1);
//     second.book(guest, 3);
// }
// This example is almost ready, but we have only one generic called T, so when we pass
// the first argument, the second automatically also takes the same type, so second, if it want
// to be a air_bnb, but the first is hotel, it will be overide the type. We need multiple traits bounds for that.

// but now this will work
// fn mix_and_match<T: Accommadation, U: Accommadation>(first: &mut T, second: &mut T, guest: &str) {
//     first.book(guest, 1);
//     second.book(guest, 3);
// }

fn main() {
    let mut hotel = Hotel::new("The Luxe");
    let mut air_bnb = AirBnb::new("Peter Schmidt");
    mix_and_match(&mut hotel, &mut air_bnb, "Peters");
}

// the book_for_one_night implementation with the Accommadation is a syntax sugar (a shorter version)
// of what we call Trait Bound Syntax (with a help of generics). Trait Bound Syntax uses a generics
// but it then limits the generic type to only types that implement a given trait. So a regular generic
// type allows any type whatsoever, while a trait bound allows us to limit the generic type to only
// those that implement a trait. They generic type is bound by the constraint that the generic type
// must implement a given trait.
