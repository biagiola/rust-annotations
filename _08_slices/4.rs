fn main() {
    let mut action_hero: String = String::from("Arnold Schwarzenegger");

    let ref1: &mut String = &mut action_hero;
    ref1.push_str(" is the Terminator");
    println!("{:p} {0}", ref1); // ✅ Works because ref1 is the only reference in scope
    // action_hero still exists
    println!("{:p} {0}", action_hero);
}

// 1. Immutable Borrowing (&T) → Many immutable references allowed at the same time.
// 2. Mutable Borrowing (&mut T) → Only one mutable reference allowed at a time.