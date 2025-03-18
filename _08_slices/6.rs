fn main() {
    let mut aciton_hero: String = String::from("Arnold Schwarzenegger");

    {   // we create a new block
        let ref1: &String = &action_hero; // Immutable borrow
        println!("{ref1}"); // Allowed because we're only reading
    } // ref1 goes out of scope here

    let ref2: &mut String = &mut action_hero; // Now mutable borrow is allowed
    ref2.push_str(" is the terminator");

    println!("{ref}");
}