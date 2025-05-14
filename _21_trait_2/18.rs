// Lecture: Associated types.
// this is a placeholder for a type that is required within a trait
// similar to generics but specific for trait definitions

use std::ops::Add; // we import the add trait

#[derive(Debug)]
struct Lunch {
    cost: f64
}

impl Add for Lunch {
    type Output = f64; // this name is something that is defined in the trait
                       // and we need to implement it 

    fn add(self, rhs: Self) -> Self::Output {
        self.cost + rhs.cost + 2.0
    }
}

fn main() {
    let one: Lunch = Lunch { cost: 19.99 };
    let two: Lunch = Lunch { cost: 29.99 };
    println!("{:.2}", one + two);

    // now, when rust see two structs and the plus sign
    // he knows that method to implement, the Add method from
    // the Lunch struct. Add is already imported from the operations
    // trait, so when we impl the Add method for the Lunch
    // we must to honor that trait, but of course we can make our
    // our logic operation there. For example for some kind of rules
    // our sums is a + b plus an extra fees o whatever. 
}

// Also, think of an associated type as a required “fill-in-the-blank”
// that lives inside the trait impl, whereas a generic parameter is
// a blank that lives outside on every use-site of the trait.