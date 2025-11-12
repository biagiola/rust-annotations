// Same example is coming from the 6.rs file but adding the display trait for each of our enums.
//
// It's the standard way to make a type convertible to a user-facing String is to implement
// the std::fmt::Display trait for it.
// This trait tells Rust how to create a human-readable representation of your type.
// And, we cannot use Display traits with enums direclty so we need to implemented manually.
// Unlike Debug (which can be derived with #[derive(Debug)]), the Display trait cannot be
// automatically derived.
// 
// There's no single "correct" way to display a type to users. For the Meat enum, should
// it display as "Chicken", "chicken", "CHICKEN", "🐔", or something else? The compiler
// can't decide this for you.
//
// Once you implement Display, you can:
// Use the {} format specifier in println! and other formatting macros.
// Call the .to_string() method on instances of your type to get an owned String.
//
// Also, we need to use the match keyword to do so.
// Final note, the write! macro is coming from std::fmt. It is not part of the Rust prelude.
use std::fmt;

// To convert an enum to a String, we must implement the `Display` trait.
#[derive(Debug)]
enum Meat {
    Chicken,
    Steak
}

// First, we implement `Display` for the inner enum, `Meat`.
impl fmt::Display for Meat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Meat::Chicken => write!(f, "Chicken"),
            Meat::Steak => write!(f, "Steak"),
        }
    }
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito(Meat),
    Bowl(Meat),
    VeganPlate
}

// Now we implement `Display` for the outer enum, `RestaurantItem`.
// This implementation can use the `Display` implementation of `Meat`.
impl fmt::Display for RestaurantItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RestaurantItem::Burrito(meat) => write!(f, "{} Burrito", meat),
            RestaurantItem::Bowl(meat) => write!(f, "{} Bowl", meat),
            RestaurantItem::VeganPlate => write!(f, "Vegan Plate"),
        }
    }
}

// tuple style
fn main() {
    let lunch: RestaurantItem = RestaurantItem::Burrito(Meat::Steak);
    let dinner: RestaurantItem = RestaurantItem::Bowl(Meat::Chicken);
    let abandoned_meal: RestaurantItem = RestaurantItem::VeganPlate;

    // Now that `Display` is implemented, we can use the `{}` formatter
    // for a clean, user-facing output.
    println!("Lunch was a {} and dinner was a {}.", lunch, dinner);
    println!("Nobody ate the {}.", abandoned_meal);

    // The `Display` trait also gives us the `.to_string()` method for free.
    let dinner_string: String = dinner.to_string();
    println!("The dinner order as a String: \"{}\"", dinner_string);
}
