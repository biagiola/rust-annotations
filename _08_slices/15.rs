fn add_last_name(name: &str) -> &str {
    "Arnold Schwarzenegger"
}

fn main() {
    let action_hero: &str = "Arnold";
    let new_name = add_last_name(action_hero);
    println!("{}", new_name);
}

// If we use &str, we cannt modify it, to pointing to
// a new string, so action_hero can no be affected outside
// its scope in other fn.