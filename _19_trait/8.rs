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

impl<T> Hotel<T> { // because we dont need the display trait affected on the construct, we separete it from the summarize one
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

impl<T> Description for Hotel<T> {} // in this way we're just using the default fn there.

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

// multiple trait bound for generic trait bounds example
// the entity must to implement both, not just one.
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
    let hotel = Hotel::new(String::from("The Luxe"));
    let airbnb: AirBnb = AirBnb::new("The Golden Standard");

    let stays: Vec<&dyn Description> = vec![&hotel, &airbnb]; // the instance elements on the vec are trait object
    println!("{}", stays[0].get_description());
    println!("{}", stays[1].get_description());

    // rust know that "stays" could have get_description method due the Description trait, but
    // just on runtime he will dispatch or figure it out if it's a Hotel or AirBnb trait and its type.
}

// dynamic dispatch tends to be slower than static dispatch and only work for references.
// With static dispatch, as soon as we assign the first value to the stays variable, its type will be inferred
// from its first value. We might think that we can avoid this by using a trait implemented by our 
// types, Hotel and Airbnb, which is what we've been doing so far with function parameters and return 
// values. Thus, we could consider defining something like this:
// let stays: Vec<impl Description> = vec![hotel, airbnb];
// However, this is not allowed in this case. Almost correct, but it's not enough yet.
// For this scenario, we need to use the `dyn` keyword. For example:
// let stays: Vec<dyn Description> = vec![hotel, airbnb];
// This is what we call dynamic dispatch.
// But there's one more thing to note: dynamic dispatch only works for references. So, the correct
// solution will be:
// let stays: Vec<&dyn Description> = vec![&hotel, &airbnb];