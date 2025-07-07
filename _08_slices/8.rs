fn main() {
    let action_hero: String = String::from("Arnold Schwarzenegger");
    let action_hero_: &str = "Arnold Schwarzenegger";

    println!("{action_hero}");
    println!("{action_hero_}");
}

// String::from creates a heap-allocated String:
// 1. Allocates memory dynamically on the heap
// 2. Owns the string data, so it can be modified
// 3. Not stored in the binary (unlike string literals)

// String literals (&str) are stored in the binary:
// 1. Stored in the program's read-only memory (.rodata section)
// 2. Exist for the entire duration of the program (static lifetime)
// 3. Cannot be modified because they're immutable