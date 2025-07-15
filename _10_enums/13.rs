// The `let-else` construct, introduced with rust 1.6.5 allows for "refutable" pattern matching.
// It's a concise way to ensure a variable matches a certain pattern
// and to diverge (e.g., return) if it doesn't.

enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String },
}

fn handle_milk(my_beverage: Milk) {
    // We attempt to match the `Milk::Lowfat` pattern against `my_beverage`.
    let Milk::Lowfat(percent) = my_beverage else {
        // If the pattern does NOT match, this block is executed.
        println!("The beverage is not lowfat milk.");
        // We must diverge (e.g., return) from the function here.
        return;
    };

    // This code is only reachable IF the pattern matched successfully.
    // The `percent` variable was created and is now available.
    println!("Success! You have {percent}% lowfat milk.");
}

fn main() {
    println!("--- Testing Lowfat Milk ---");
    let my_beverage: Milk = Milk::Lowfat(2);
    handle_milk(my_beverage);

    println!("\n--- Testing Whole Milk ---");
    let my_beverage: Milk = Milk::Whole;
    handle_milk(my_beverage);
}