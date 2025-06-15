// Lecture: String iteration
// we dont know exactly who long could be each element of the string,
// remember, emojis could be twice or more longer than a normal character.

// So we have two main methos, char and bytes
// The chard method returns an iterator of the Unicode characters.
// The bytes method returns an iterator of the raw bytes.

fn main() {
    // this string will ocupy a total of ten bytes. Six bytes
    // for the first six English alphabetic characters and then
    // four bytes for the actual emoji
    let seafood: &str = "Oyster🦪";

    // Print row information
    for byte in seafood.bytes() {
        // this will give us the numeric representation of this string
        // and its bytes in memory
        print!("{byte}/"); // use a slash just for readability
    }

    // print each character
    for character in seafood.chars() {
        print!("{character}");
    }

    println!(); // one break like to separate it from the previous print
    println!("{}", seafood.bytes().len());
    println!("{}", seafood.chars().count());
}