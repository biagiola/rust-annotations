// Lecture: Associated Types – Juicer example
// Demonstrates how each impl of a trait chooses a concrete associated type.

// A trait whose implementers must decide what kind of juice they yield
trait Juicer {
    type Juice; // also called Output

    fn squeeze(self) -> Self::Juice;
}

#[derive(Debug)]
struct Orange;

#[derive(Debug)]
struct Apple;

// every concrete type that wishes to be a Juicer must declare what ‘juice’ it produces
// for in this case, if you want to use a Orange Juice, your return type will be String
// but if it's Apple Juice, it's a &str or a String again.

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

// After the trait come two empty marker structs, Orange and Apple. They exist only to receive implementations of Juicer.
// When we write `impl Juicer for Orange { … }` we are creating an implementation block that ties the generic behaviour
// described by Juicer to a specific concrete type, Orange. The first thing the compiler demands inside that block is a
// concrete definition for every associated type declared by the trait. Hence the line `type Juice = String;`. Here we are
// choosing the standard-library String type to stand in for Self::Juice whenever an Orange is squeezed. Once that choice is
// made we must satisfy the rest of the trait contract, filling in the body of fn squeeze. The function returns a String in
// accordance with the choice we just announced, producing the text “Sweet Orange Juice!”.

// The implementation for Apple follows the same pattern. It could have chosen an entirely different concrete type—perhaps a
// custom AppleJuice struct—but in the example it also chooses String. What matters is that the decision is fixed per implementation.
// All calls to .squeeze() on Apple values will fo