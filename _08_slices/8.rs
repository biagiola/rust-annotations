fn main() {
    let action_hero: String = String::from("Arnold Schwarzenegger");
    let action_hero_: &str = "Arnold Schwarzenegger";

    println!("{action_hero}");
    println!("{action_hero_}");
}

// String::from creates a heap-allocated String:
// 1. It allocates memory dynamically on the heap
// 2. It owns the string data, so it can be modified
// 3. It is not stored in the binary (unlike string literals)

// In the case of string literals, I mean, the &str version
// they are store into the 'binary', in other words, it's
// embedded directly into the compiled program's read-only memory,
// static memory, instead of being allocated dynamically on the heap.
// 1. Stored in the program's read-only memory (.rodata section)
// 1. Exists for the entire duration of the program (static lifetime)
// 2. Cannot be modified, because it's immutable