// The match keyword III
// example of a enum with three differents variants inside of it.
enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String)
}

fn main() {
    //  wash_laundry(LaundryCycle::Cold);
    wash_laundry(LaundryCycle::Hot { temperature: 100 });
    wash_laundry(LaundryCycle::Delicate(String::from("Slik")));
}

fn wash_laundry(cycle: LaundryCycle) {
    match cycle {
        LaundryCycle::Cold => {
            println!("Running the laundry with cold temperature")
        }
        LaundryCycle::Hot { temperature } => {
            println!("Running the laundry with hot temperature of {temperature}")
        }
        LaundryCycle::Delicate(fabric_type) => {
            println!("Running the laundry with a delicate cycle for {fabric_type}")
        } // for type types, position matters
    }
}