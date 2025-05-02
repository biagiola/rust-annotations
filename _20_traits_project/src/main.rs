use traits::{
    book_for_one_night,
    mix_and_match,
    Hotel,
    AirBnb,
    Accommadation,
    Description
};

fn main() {
    let mut hotel = Hotel::new(String::from("The Luxe"));
    hotel.book("Dana", 5);
    println!("{}", hotel.summarize());

    let mut airbnb: AirBnb = AirBnb::new("The Golden Standard");
    book_for_one_night(&mut airbnb, "Piers");
    println!("{}", airbnb.get_description());
    
    mix_and_match(&mut hotel, &mut airbnb, "Dan");
}
