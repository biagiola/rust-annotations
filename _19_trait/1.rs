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

fn main() {
    let mut hotel = Hotel::new("The Luxe");
    // println!("{}", hotel.get_description());
    println!("{}", hotel.summarize());
    hotel.book("Piers", 5);
    println!("{:#?}", hotel);

    let mut air_bnb = AirBnb::new("Peter Schmidt");
    println!("{}", air_bnb.get_description());
    air_bnb.book("Anderson", 15);
    println!("{:#?}", air_bnb);
}

// Side notes
// . in other languages we call traits as interfaces
// . traits use Pascal case names.
// . we must to use all the fn declared in the trait for the Hotel.
// . we can explicity return a value from a fn on the trait
// fn book(&mut self, name: &str, nights: u32) -> {};
// remember that the default return in rust are the empty unit.
// . notice that the first implementation on Accomadation saves the data on a HashMap
// and the second on a vector for book fn.

// . for summarize fn, we dont have the get_description fn on the Hotel impl's for it
// takes from the Accomodation trait.
// . we can use the default implementation of get_description that we have in the the accommodation or
// or we can define our own get_description method from the AirBnB struct for example. But
// we cannot mix both. We cannot relay on the trait definition version and then add on a 
// funcionality on top of that. That's something possible with other language but not in Rust