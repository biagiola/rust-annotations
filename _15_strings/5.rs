fn main() {
    let mut music_genres: &str = "  Rock, Metal, Country, Rap  ";
    println!("{}", music_genres.trim()); // trim creates a new string slice, doesn't modify original
    println!("{}", music_genres.trim_start());
    println!("{}", music_genres.trim_end());

    music_genres = music_genres.trim(); // reassigning variable to point to new trimmed slice
    println!("{}", music_genres.to_uppercase());
    println!("{}", music_genres.to_lowercase());

    println!("{}", music_genres.replace("Metal", "Funk"));

    // splits by delimiter
    let genres: Vec<&str> = music_genres.split(", ").collect();
    println!("{:#?}", genres);
    
    // String checking methods
    let test_string = "Hello, Rust World! 123";
    println!("\n--- String Analysis ---");
    println!("Contains 'Rust': {}", test_string.contains("Rust")); // case sensitive
    println!("Starts with 'Hello': {}", test_string.starts_with("Hello"));
    println!("Ends with '123': {}", test_string.ends_with("123"));
    println!("Is empty: {}", test_string.is_empty());
    println!("Length: {}", test_string.len()); // returns the number of bytes in the string, not the number of characters

    // String iteration
    println!("\n--- String Iteration ---");
    println!("Characters:");
    for (i, c) in "Rust".chars().enumerate() {
        println!("  {} -> '{}'", i, c);
    }
    
    // Character analysis
    println!("\n--- Character Analysis ---");
    println!("Is all ASCII: {}", test_string.is_ascii());
    println!("All alphabetic: {}", "HelloWorld".chars().all(|c| c.is_alphabetic())); // we're using all iterator method with closure.
    println!("All numeric: {}", "12345".chars().all(|c| c.is_numeric()));
    
    // Advanced splitting
    println!("\n--- Advanced Splitting ---");
    let data = "apple::banana::cherry";
    let fruits: Vec<&str> = data.split("::").collect(); // collect() is a method that takes an iterator and returns a collection, is something we'll see in the iterator section in detail.
    println!("Split by '::': {:?}", fruits);
    
    let sentence = "Hello world rust programming";
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("Split whitespace: {:?}", words);
    
    // String finding and indexing
    println!("\n--- Finding & Indexing ---");
    if let Some(pos) = test_string.find("Rust") {
        println!("'Rust' found at position: {}", pos); // returns the index of the first occurrence of the substring
    }
    
    // String manipulation with String type
    println!("\n--- String Manipulation ---");
    let mut mutable_string = String::from("Hello");
    mutable_string.push('!');
    mutable_string.push_str(" World");
    println!("After push operations: {}", mutable_string);
    
    // String formatting with padding
    println!("\n--- String Formatting ---");
    let name = "Rust";
    println!("Left aligned: '{:<10}'", name);
    println!("Right aligned: '{:>10}'", name);
    println!("Center aligned: '{:^10}'", name);
    println!("With padding: '{:*^10}'", name);
}
