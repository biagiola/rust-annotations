// Lecture: Display trait implemented on struct
use std::fmt::{Display, Formatter, Result};

struct Apple {
    kind: String,
    price: f64,
}

impl Display for Apple {
    // fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
    //     write!(formatter, "{} 🍏 for {}", self.kind, self.price)
    // }

    // Option 2: Using formatter methods directly
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.kind)?;
        f.write_str(" 🍏 for ")?;
        f.write_str(&self.price.to_string())
    }
}

fn main() {
    let lunch_snack: Apple = Apple {
        kind: String::from("Granny Smith"),
        price: 1.04
    };

    println!("{}", lunch_snack);
}

// The Display trait has a specific contract that you must follow exactly:
// pub trait Display {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
// }
// we need to use the exact signature to overwrite it.

// Also, the write! macro is actually using the formatter behind the scenes.
// The formatter needs to be mutable because:
// It accumulates text as it builds the final string
// It tracks internal state (position, formatting flags, etc.)
// Writing to it modifies its internal buffer
// impl Display for Apple {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         // Option 1: Using write! macro (what you're doing)
//         write!(f, "{} 🍏 for {}", self.kind, self.price)
        
//         // Option 2: Using formatter methods directly
//         // f.write_str(&self.kind)?;
//         // f.write_str(" 🍏 for ")?;
//         // f.write_str(&self.price.to_string())
//     }
// }