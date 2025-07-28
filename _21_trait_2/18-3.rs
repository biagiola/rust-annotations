// Lecture: Associated Types – Juicer example
// Demonstrates how each impl of a trait chooses a concrete associated type.

// A trait whose implementers must decide what kind of juice they yield
trait Juicer {
    type Juice; // This is our associated type

    fn squeeze(self) -> Self::Juice;
}

#[derive(Debug)]
struct Orange;

#[derive(Debug)]
struct Apple;

// When we put an Orange in the juicer, we get an owned String
impl Juicer for Orange {
    type Juice = String; // Orange produces an owned String

    fn squeeze(self) -> Self::Juice {
        "Sweet Orange Juice!".to_string() // Creates and returns an owned String
    }
}

// When we put an an Apple in the juicer, we get a string slice (&'static str)
impl Juicer for Apple {
    type Juice = &'static str; // Apple produces a static string slice

    fn squeeze(self) -> Self::Juice {
        "Crisp Apple Juice!" // This is a string literal, which has type &'static str
    }
}

fn main() {
    let my_orange = Orange;
    let orange_juice = my_orange.squeeze(); // Type will be inferred as String
    println!("I made: {}", orange_juice);
    // At this point, orange_juice is an owned String.
    // It owns its data and can be modified or moved freely.

    let my_apple = Apple;
    let apple_juice = my_apple.squeeze(); // Type will be inferred as &'static str
    println!("I made: {}", apple_juice);
    // At this point, apple_juice is a string slice with static lifetime.
    // It's a reference to data embedded directly in the program's binary.
}