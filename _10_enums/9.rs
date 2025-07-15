// The match keyword III
// The `match` keyword provides exhaustive pattern matching, making it
// perfect for handling all possible variants of an enum.

enum LaundryCycle {
    // A "unit-like" variant with no data.
    Cold,
    // A "struct-like" variant with a named field.
    Hot { temperature: u32 },
    // A "tuple-like" variant with one data field.
    Delicate(String),
}

fn main() {
    wash_laundry(LaundryCycle::Cold);
    wash_laundry(LaundryCycle::Hot { temperature: 100 });
    wash_laundry(LaundryCycle::Delicate(String::from("Silk")));
}

fn wash_laundry(cycle: LaundryCycle) {
    // This `match` must be exhaustive. The compiler ensures we handle every
    // possible variant of `LaundryCycle`, preventing bugs.
    match cycle {
        LaundryCycle::Cold => {
            println!("Running the laundry with a cold temperature.");
        }
        // For struct-like variants, we destructure by field name.
        LaundryCycle::Hot { temperature } => {
            println!("Running the laundry with a hot temperature of {temperature}°C.");
        }
        // For tuple-like variants, we destructure by position (position matters).
        LaundryCycle::Delicate(fabric_type) => {
            println!("Running the laundry with a delicate cycle for {fabric_type}.");
        }
    }
}