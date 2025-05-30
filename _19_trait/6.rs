// Lecture: Traits as function return values
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

impl Description for Hotel {}

impl Description for AirBnb {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }
}

// multiple trait bound for generic trait bounds example
// the entity must to implement both, not just one.
fn book_for_one_night<T: Accommadation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

// each trait implementation per parameter as previously
// fn mix_and_match(
//     first: &mut (impl Accommadation + Description),
//     second: &mut impl Accommadation,
//     guest: &str
// ) {
//     first.book(guest, 1);
//     first.get_description();

//     second.book(guest, 3);
// }

// using trait bound on generics
// fn mix_and_match<T: Accommadation + Description, U: Accommadation>(
//     first: &mut T,
//     second: &mut U,
//     guest: &str
// ) {
//     first.book(guest, 1);
//     first.get_description();

//     second.book(guest, 3);
// }

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
    // Hotel::new("The Luxe") // work for both structs
    AirBnb::new("The Luxe")

    // we can return just one of the two in our fn logic
    // for example this will be wrong:

    // let likes_luxury = true;
    
    // if likes_luxury {
    //     Hotel::new("The Luxe")
    // } else {
    //     AirBnb::new("Common Apart")
    // }
}

fn main() {
    let mut hotel = Hotel::new("The Luxe"); // if we use the rust analyser from vs code extension, it will show us now a type of "impl Accommadation" instead of "Hotel"
    let mut air_bnb = AirBnb::new("Peter Schmidt");
    // mix_and_match(&mut hotel, &mut air_bnb, "Piers");
    let result = choose_best_place_to_stay();
    // println!("{result:#?}");                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    
}