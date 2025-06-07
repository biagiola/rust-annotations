// Lecture: The String.retain method I
// An example of FnMut trait. Rembember, FnMut is a sub trait
// of FnOnce, it can be called multiple times and will borrow
// mutable references to any captured values.

fn main() {
    let mut game_console = String::from("Playstation");

    // technically our closure it's not a FnMut coz
    // there's technically no mutation going on. Fn
    // trait is a sub trait of FnMut
    let closure = |character| character != 'a';
    game_console.retain(closure);
    println!("{game_console}");

    // Fn trait can also run multiple times. It won't move
    // ownership, and it won't move any captured value.
    // Characters implements the copy trait, so each time it
    // runs, it's simply going to be getting a copy of the
    // original character.
}

// Retain method definition
// pub fn retain(&mut self, mut f: F)
// where
//     F: FnMut(char) -> bool,
// {
//     ...
// }
// It retains only the characters specified by the predicate
// In other words, remove all characters 'c' such that f(c) returns false.
// This method operates in place, visiting each character exactly
// once in the original order, and preservers the order of the retained character
