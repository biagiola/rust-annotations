fn main() {
    let airline = "United";
    
    // ❌ WRONG: This doesn't compile!
    // let first_initial = &airline[0];
    
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
}
