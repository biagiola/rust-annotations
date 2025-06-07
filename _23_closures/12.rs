// Lecture: The String.retain method II

// retain calls the closure multiple times (once per character),
// it needs the ability to mutate state across calls if needed.

fn main() {
    let mut game_console = String::from("PlayStation");
    let mut deleted_characters = String::new();

    let closure = |character| {
        let is_not_a: bool = character != 'a';
        
        if is_not_a {
            true
        } else {
            deleted_characters.push(character);
            false
        }
    };

    game_console.retain(closure);
    println!("{deleted_characters}");
}