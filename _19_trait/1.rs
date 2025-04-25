use std::collections::HashMap;

trait Accommadation {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay"),
        guest: vec![],
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
    println!("{}", hotel.get_description());
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