// Lecture: Implementing the partialEq trait on enums

// #[derive(PartialEq)] // we're implement our custom partialEq just for learning purpose
enum Musician {
    SingerSongWriter(String),
    Band(u32),
}

use Musician::{Band, SingerSongWriter}; // refer variables without the prefix

// hence, there's no generics in the partialEq, that means
// we're comparing Musician enum with another Musician enum
// and that's include all the posible variants
impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        match self {
            SingerSongWriter(name) => match other {
                SingerSongWriter(other_name) => name == other_name,
                Band(_) => false
            },
            Band(members) => match other {
                SingerSongWriter(_) => false,
                Band(other_members) => members == other_members
            }
        }
    }
}
// all this logic is handle by the #[derive(PartialEq)]

fn main() {
    let rusting_bieber: Musician = SingerSongWriter("Rustin".to_string());
    let rustin_timberlake: Musician = SingerSongWriter("Rustin".to_string());
    let holly: Musician = SingerSongWriter("Holly".to_string());

    let rust_no_one: Musician = Band(5);
    let unrustworthy: Musician = Band(4);
    let rust_for_vegeance: Musician = Band(5);

    println!("{}", rusting_bieber == holly); // false
    println!("{}", rusting_bieber == rustin_timberlake); // true
    println!("{}", rusting_bieber == rust_no_one); // false

    println!("{}", rust_no_one == unrustworthy);
    println!("{}", rust_no_one == rust_for_vegeance);
}

