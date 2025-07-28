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

// every concrete type that wishes to be a Juicer must declare what ‘juice’ it produces
// for in this case, if you want to use a Orange Juice, your return type will be String
// but if it's Apple Juice, it's a &str.

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
    let orange_juice: <Orange as Juicer>::Juice = my_orange.squeeze();
    println!("I made: {}", orange_juice);

    let my_apple: <Apple as Juicer>::Juice = Apple;
    let apple_juice = my_apple.squeeze();
    println!("I made: {}", apple_juice);
}

// At main you see how this plays out. let orange_juice = my_orange.squeeze(); compiles because the compiler knows
// from the impl that Orange’s Self::Juice is String. No generic type arguments are written at the call site;
// the association is baked into the implementation, giving very ergonomic syntax to the caller. The same happens for Apple.


// Side note about typification: if type of orange_juice is <Apple as Juicer>::Juice.
// There's a problem here! You are squeezing an Orange (my_orange.squeeze()), which produces an Orange::Juice
// which is String, but you are trying to assign it to a type declared as Apple::Juice.
// In this specific case, both Orange::Juice and Apple::Juice happen to be String, so it compiles.
// But if Apple::Juice was, for example, f64, you would get a compile-time error
// because you're trying to assigna String to an f64.

// Implicit Type Inference (Most Common and Idiomatic):
// let orange_juice = my_orange.squeeze(); // Rust infers 'String'

// Explicit Type Annotation using the Implementor's Associated Type:
// let orange_juice: <Orange as Juicer>::Juice = my_orange.squeeze();

// Explicit Type Annotation using the Concrete Type (less common, but valid if you know it):
// let orange_juice: String = my_orange.squeeze();