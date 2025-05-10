// Lecture: Display trait for enums
use std::fmt::{Display, Formatter, Result};

enum AppleType {
    RedDelicious,
    GrannySmith,
}

// Enum does not implement the Display trait of rust core by default so
// we have to implement it, for the two arms of the enum in our case
impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "🍎 Delicious 🍎"),
            AppleType::GrannySmith => write!(formatter, "🍏 Granny Smith 🍏"),
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
        // The write! macro is used here to format the string. 
        // Since `AppleType` now implements `Display`, the enum variant gets formatted as a string.

        // `self.kind` is an enum variant, so we pass it to the write macro.
        // Thanks to the `Display` impl on `AppleType`, it automatically gets formatted into a string.
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

    println!("{}", lunch_snack);
    println!("{}", dinner_snack);
}