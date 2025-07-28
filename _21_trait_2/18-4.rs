// Lecture: Generic parameter version of the Juicer trait (no associated type)

// Contrast this with ordinary generics that live outside the trait. If Juicer had instead been declared
// with a generic parameter <J> on its methods, every call‐site would have to specify which concrete type
// substituted for J, or the compiler would have to use type inference to guess. Associated types shift
// that burden away from callers to implementers, making the API surface simpler and avoiding ambiguity when
// many implementations could exist for the same pair of types but different return types.

// Here the output type is a generic parameter chosen at the *use-site* rather
// than an associated type fixed by the implementation.

// A generic trait: the caller supplies the concrete `J` each time.
trait Juicer<J> {
    fn squeeze(self) -> J;
}

#[derive(Debug)]
struct Orange;

#[derive(Debug)]
struct Apple;

// One specific instantiation: `J = String` for Orange.
impl Juicer<String> for Orange {
    fn squeeze(self) -> String {
        "Sweet Orange Juice!".to_string()
    }
}

// Another specific instantiation: `J = String` for Apple.
impl Juicer<String> for Apple {
    fn squeeze(self) -> String {
        "Crisp Apple Juice!".to_string()
    }
}

fn main() {
    let my_orange = Orange;
    let orange_juice: String = my_orange.squeeze(); // J inferred as String
    println!("I made: {}", orange_juice);

    let my_apple = Apple;
    // Explicit turbofish syntax could be used: my_apple.squeeze::<String>()
    let apple_juice = my_apple.squeeze();
    println!("I made: {}", apple_juice);
}
