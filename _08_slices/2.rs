fn main() {
    let action_hero: String = String::from("Arnold Schwarzenegger");
    let string_reference: &String = &action_hero; // borrowing, NOT moving ownership

    // string_reference is borrowing FROM action_hero (not the other way around)
    // We can still print action_hero because immutable borrowing allows this
    println!("{0}", action_hero);

    // This drop would fail because action_hero is currently borrowed by string_reference
    // drop(action_hero);
    println!("{:p}: {0}", string_reference);

    // {:p} prints the stack address where action_hero is stored
    // {0} prints the value that string_reference points to ("Arnold Schwarzenegger")
    // No dangling reference here - both action_hero and string_reference are valid
}

