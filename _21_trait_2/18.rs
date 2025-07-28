// Lecture: Associated types I.
// this is a placeholder for a type that is required within a trait
// similar to generics but specific for trait definitions

// In our example 
use std::ops::Add; // we import the add trait

#[derive(Debug)]
struct Lunch {
    cost: f64
}

// also, we can return a new Lunch struct 
impl Add for Lunch {
    // this is the placeholder, it is called an associated type;
    // it is inherently linked—associated—with each implementation of the trait.
    type Output = Lunch;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost
        }
    }
}

fn main() {
    let one: Lunch = Lunch { cost: 19.99 };
    let two: Lunch = Lunch { cost: 29.99 };
    // println!("{:.2}", one + two);
    println!("{:?}", one + two);

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


// impl Add for Lunch {
//     type Output = f64; // this name is something that is defined in the trait
//                        // and we need to implement it for the add in our case
                          // but in other example we'll use other names

//     fn add(self, rhs: Self) -> Self::Output {
//         self.cost + rhs.cost + 2.0
//     }
// }