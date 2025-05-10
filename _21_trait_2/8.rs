// Lecture: Implementing custom debug trait
// we can modify or extend the default debug trait of rust
use std::fmt::{Debug, Display, Formatter, Result};

// #[derive(Debug)] // the thing is we're going to edit, the default
// rust debug trait for our AppleType enum and also the apple struct
enum AppleType {
    RedDelicious,
    GrannySmith,
}

impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "🍎 Delicious 🍎"),
            AppleType::GrannySmith => write!(formatter, "🍏 Granny Smith 🍏"),
        }
    }
}

impl Debug for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "AppleType::RedDelicious"),
            AppleType::GrannySmith => write!(formatter, "AppleType::GrannySmith"),
        }
    }
}

struct Apple {
    kind: AppleType,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{} for {}", self.kind, self.price)
    }
}

impl Debug for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(
            formatter,
            "Apple ::: [ kind: {}, Price: {} ]",
            self.kind, self.price
        )
        // notice we're relying on the debug trait we created
        // for kind, we can use the our display if we want
    }
}

fn main() {
    let lunch_snack: Apple = Apple {
        kind: AppleType::GrannySmith,
        price: 1.04,
    };

    let dinner_snack: Apple = Apple {
        kind: AppleType::RedDelicious,
        price: 1.15,
    };

    // println!("{:?}", lunch_snack); // this will use the display trait
    println!("{:?}", lunch_snack); // this will use the debug trait, not the default one, but ours.
    println!("{:?}", dinner_snack);
}