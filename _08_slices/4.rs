fn main() {
    let mut action_hero: String = String::from("Arnold Schwarzenegger");

    let ref1: &mut String = &mut action_hero;
    ref1.push_str(" is the Terminator");

    println!("{:p} {}", ref1, ref1); // ✅ Address and value of ref1
    // action_hero still exists
    println!("{:p} {}", &action_hero, action_hero); // ✅ Address and value of action_hero
}

// 1. Immutable Borrowing (&T) → Many immutable references allowed at the same time.
// 2. Mutable Borrowing (&mut T) → Only one mutable reference allowed at a time.
// 3. {:p} format specifier requires a reference/pointer, not the value itself.