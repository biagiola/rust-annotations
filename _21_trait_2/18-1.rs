// Lecture: Associated Types – Juicer example
// Demonstrates how each impl of a trait chooses a concrete associated type.

// A trait whose implementers must decide what kind of juice they yield
trait Juicer {
    type Juice;

    fn squeeze(self) -> Self::Juice;
}

#[derive(Debug)]
struct Orange;

#[derive(Debug)]
struct Apple;

// When we put an Orange in the juicer, we get orange juice (here represented by a String)
impl Juicer for Orange {
    type Juice = String;

    fn squeeze(self) -> Self::Juice {
        "Sweet Orange Juice!".to_string()
    }
}

// When we put an Apple in the juicer, we get apple juice (also a String for this demo)
impl Juicer for Apple {
    type Juice = String;

    fn squeeze(self) -> Self::Juice {
        "Crisp Apple Juice!".to_string()
    }
}

fn main() {
    let my_orange = Orange;
    let orange_juice = my_orange.squeeze();
    println!("I made: {}", orange_juice);

    let my_apple = Apple;
    let apple_juice = my_apple.squeeze();
    println!("I made: {}", apple_juice);
}