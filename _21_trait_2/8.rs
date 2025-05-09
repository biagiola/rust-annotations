// Lecture: Implementing the debug trait
use std::fmt::{Debug, Display, Formatter, Result};

// #[derive(Debug)] // the thing is we're going to edit the default rust debug trait
enum AppleType {
    RedDelicious,
    GrannySmith,
}

impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "🍎 Delicious 🍎"), // TODO: something else need it to do  here
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

// #[derive(Debug)]
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

    println!("{:?}", lunch_snack); // now it is using the default debug trait instead of the display one
    println!("{:?}", dinner_snack);
}