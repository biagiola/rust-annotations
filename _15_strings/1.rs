fn main() {
    let pirate: &str = "Bloodhook";

    // heap data
    let sailer: String = String::from(pirate);
    let bad_guy: String = pirate.to_string(); // same as above

    println!("{pirate} and {sailer} and {bad_guy}");

    // we cannot access to an specific characted as we treat it in arrays or vectors
    // niether &str nor String. 
    // For english alphabet, one english character occupies 1 byte in memory.
    // but it is not the same for emojis for examples or non english characters
    // so that's why we cannot use access like arrays.

    // We says we want to grab the first four letters and in this case is ok.
    // But if we dealing with german words for examples with umlaut vowels it
    // will maybe it would not grab the four first words. Each index represents
    // a byte not a word necessarily 
    let first_initial: &str = &pirate[0..4];
    println!("{first_initial}");
}