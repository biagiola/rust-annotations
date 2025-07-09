fn main() {
    let airline = "United";
    
    // ❌ WRONG: This doesn't compile!
    // let first_initial = &airline[0];

    // UTF-8 safety: Index 0 might split a multi-byte character

    // ✅ CORRECT METHODS:
    
    // Method 1: Get first character as Option<char>
    let first_char = airline.chars().next();
    match first_char {
        Some(c) => println!("First character: '{}'", c),
        None => println!("String is empty"),
    }
    
    // Method 2: Get first character at specific index
    if let Some(first_char) = airline.chars().nth(0) {
        println!("First character (using nth): '{}'", first_char);
    }
    
    // Method 3: Get first character as string slice (DANGEROUS for non-ASCII!)
    let first_slice = &airline[0..1];  // Only safe for ASCII
    println!("First byte as slice: '{}'", first_slice);
    
    // Method 4: Safe way to get first N characters
    let first_three: String = airline.chars().take(3).collect();
    println!("First three characters: '{}'", first_three);
    
    // ⚠️ DEMONSTRATION: Why byte indexing is dangerous
    let german = "Über";
    println!("\n--- Dangerous vs Safe ---");
    println!("German word: '{}'", german);
    
    // This would PANIC: let bad_slice = &german[0..1];
    // Because 'Ü' takes 2 bytes in UTF-8
    
    // Safe way:
    if let Some(first_char) = german.chars().next() {
        println!("First character safely: '{}'", first_char);
    }
    
    // Getting specific characters (2nd, 3rd, etc.)
    
    println!("\n--- Getting Specific Characters ---");
    
    // ✅ BEST: Using chars().nth() for specific positions
    if let Some(second_char) = german.chars().nth(1) {
        println!("Second character: '{}'", second_char);  // 'b'
    }
    
    if let Some(third_char) = german.chars().nth(2) {
        println!("Third character: '{}'", third_char);   // 'e'
    }
    
    if let Some(fourth_char) = german.chars().nth(3) {
        println!("Fourth character: '{}'", fourth_char); // 'r'
    }
    
    // ❌ INEFFICIENT: Chaining next() calls (don't do this!)
    // let mut chars = german.chars();
    // let first = chars.next();   // Skip first
    // let second = chars.next();  // Get second
    // let third = chars.next();   // Get third
    
    // ✅ EFFICIENT: Get multiple characters at once
    let chars_vec: Vec<char> = german.chars().collect();
    println!("All characters: {:?}", chars_vec);
    
    // Access by index (safe because we collected into Vec)
    if chars_vec.len() > 1 {
        println!("Second char from Vec: '{}'", chars_vec[1]);
    }
    
    // ✅ ALTERNATIVE: Using enumerate to get both index and character
    println!("\n--- All Characters with Index ---");
    for (i, c) in german.chars().enumerate() {
        println!("Character at index {}: '{}'", i, c);
    }
}
