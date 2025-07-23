// Lecture: Traits Bound and conditionally implements methods
// Trait bound and Orphan rules

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}

trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}

#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>
}

impl<T> Hotel<T> { // because we dont need the display trait be affected on the construct, we separete it from the summarize one
    fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl <T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl<T> Description for Hotel<T> {} // we're using the default definition

// Side note
// The Orphan rule is when we trying to implement the default Display trait with the Vec type,
// both comming from another crate, and trying to use directly into my own and actual crate.

// But, here we are implementing our custom Display trait, with the help of NameList struct.
struct NameList(Vec<&'static str>);

// Doing this allows us to satisfy the T: Display bound in our generic
impl Display for NameList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

fn main() {
    let hotel1 = Hotel::new(String::from("The Luxe")); // this implement the display trait
    println!("{}", hotel1.summarize());

    let hotel2 = Hotel::new(String::from("The Golden Standard")); // this implement the display trait too
    println!("{}", hotel2.summarize());

    let hotel3 = Hotel::new(vec!["The Sweet Escape", "Hilton Edition"]);

    // println!("{}", hotel3.summarize());
}

// The compilation failure you're facing (for hotel3.summarize()) is related to Rust’s coherence rules and
// trait bounds not being satisfied. This is about coherence rules and method resolution.

// The method summarize() exists only on Hotel<T> where T, that is the generic used for the field name,
// implements the Display like String, not like Vec<&str>.

// In particular, the method summarize only exists if T: Display. This is called a conditional method implementation
// or specialization based on trait bounds.

// When you try to call hotel3.summarize() and T is Vec<&str>, the compiler says:
// “Hold on, I don’t see a summarize method for this instance of Hotel<T> — because you only defined that
// method for T: Display, and Vec<&str> doesn’t implement Display.”

// This is intentional coherence enforcement by Rust: it ensures that the compiler can always unambiguously
// resolve method calls based on their trait bounds and type context.


// Orphan Rule
// The orphan rule in Rust is a restriction that prevents you from implementing a trait for a type if
// neither the trait nor the type is defined in the current crate (i.e., your current code module or library).
// This rule ensures that there are no conflicting implementations of traits for a type, which helps maintain
// consistency and prevents ambiguity.

// To put it simply, you can only implement a trait for a type if at least one of them (either the trait or the type)
// is defined locally in your crate. And, in our case, neither the Display trait nor the Vec type was defined in our locally crate
