use std::collections::HashMap;
use std::fmt::Display;

pub trait Accommadation {
    fn book(&mut self, name: &str, nights: u32);
}

pub trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}

#[derive(Debug)]
pub struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>
}

impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    pub fn summarize(&self) -> String {
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
pub struct AirBnb {
    host: String,
    guests: Vec<(String, u32)>
}

impl AirBnb {
    pub fn new(host: &str) -> Self {
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

pub fn book_for_one_night<T: Accommadation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

pub fn mix_and_match<T, U>(
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
