// Lecture: A preview of trait objects
// A trait Object is an instance of a type that implements a particular trait.
// Whose methods will be accessed at runtime using a feature called dynamic dispatch
use std::collections::HashMap;
use std::fmt::Display;

trait Accommadation {
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

impl<T> Hotel<T> {
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

impl<T> Accommadation for Hotel<T> {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl<T> Description for Hotel<T> {}

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
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}
impl Description for AirBnb {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }
}

fn book_for_one_night<T: Accommadation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

// using where clauses
fn mix_and_match<T, U>(
    first: &mut T,
    second: &mut U,
    guest: &str
) where
    T: Accommadation + Description,
    U: Accommadation
{
    first.book(guest, 1);
    first.get_description();

    second.book(guest, 3);
}

fn choose_best_place_to_stay() -> impl Accommadation + Description {
    AirBnb::new("The Luxe")
}

fn main() {
    let mut hotel = Hotel::new(String::from("The Luxe"));
    let mut airbnb: AirBnb = AirBnb::new("The Golden Standard");

    // example of trait objects for dynamic dispatch that mutate its values
    let mut stays: Vec<&mut dyn Accommadation> = vec![&mut hotel, &mut airbnb];
    stays[0].book("Adrew", 3);
    stays[1].book("Lore", 7);

    println!("{:#?}", hotel);
    println!("{:#?}", airbnb);

}

// now we're using Accommodation trait, that one add a new booking into our data struct
// we, is a perfect example of how use dynamic dispatch on mutable references. Remember,
// dynamic dispatch is only working on references and also.
