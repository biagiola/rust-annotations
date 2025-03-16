fn main() {
    let action_hero: String = String::from("Arnold Schwarzenegger");
    let string_reference: &String = &action_hero; // moving ownership

    // Now action_hero is borrowring the value of string_reference
    // so we can't print its address due the dangling reference
    println!("{0}", action_hero);

    // this drop is gonna fail coz it was borrowed
    // drop(action_hero);
    println!("{:p}: {0}", string_reference);

    // {:p} prints the memory address of string_reference, which is stored on the stack.
    // {0} prints the value of action_hero, which is stored on the heap.
    // Since string_reference is still valid, the reference can be safely used.
}

