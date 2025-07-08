fn add_last_name(name: &str) -> &str {
    "Arnold Schwarzenegger"
}

fn main() {
    let action_hero: &str = "Arnold";
    let new_name = add_last_name(action_hero);
    println!("{}", new_name);
}

// &str is immutable - we cannot modify the string data it points to.
// We can only return references to existing strings or string literals.
// The original action_hero variable is unaffected because &str doesn't allow modification.