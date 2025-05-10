// Lecture: Implementing the Drop trait
// each data he allocated on the heap we can deallocated when
// it is our of the scope. These data types implements the Drop trait.
// And we can define our onw drop mechanism in our own custom types by
// implementing the drop trait.
use std::fmt::{Debug, Display, Formatter, Result};
use std::fs;
use std::ops::Drop; // this is actually automatically imported from the rust prelude.

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
        // we can tons of methods on the formatter to use
        formatter.debug_struct(" ** Apple **") // <- this returns a DebugStruct
            .field("Kind", &self.kind) // that's allow us to chainning on
            .field("Price", &self.price)
            .finish() // and now we're return the Result enum.
    }
}

// this drop fn is called automatically we Apple struct is out of scope
impl Drop for Apple {
    fn drop(&mut self) {
        // println!("Apple is being cleaned up");
        match fs::remove_file("apple.txt") {
            Ok(_) => println!("Goodbye, my sweet apple"),
            Err(error) => eprintln!("Error deleting file: {error}"),
        }
    }
}

fn main() {
    let lunch_snack: Apple = Apple {
        kind: AppleType::GrannySmith,
        price: 1.04,
    };

    // let dinner_snack: Apple = Apple {
    //     kind: AppleType::RedDelicious,
    //     price: 1.15,
    // };

    println!("{:?}", lunch_snack);
    // println!("{:?}", dinner_snack);
}