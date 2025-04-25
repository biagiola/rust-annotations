use std::collections::HashMap;

trait Accommadation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32);
}

fn main() {

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
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxury", self.name);
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

// Side notes
// . in other languages we call traits as interfaces
// . traits use Pascal case names.
// . we must to use all the fn declared in the trait for the Hotel.
// . we can explicity return a value from a fn on the trait
// fn book(&mut self, name: &str, nights: u32) -> {};
// remember that the default return in rust are the empty unit.
