#[derive(Debug)]

// An enum is a type that represents a set of possible values.
// each possible value is called a variant
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

// we can use that enum in a variable or struct etc
struct Card {
    rank: String,
    suit: CardSuit
}

fn main() {
    let first_card: CardSuit = CardSuit::Hearts;
    let mut second_card: CardSuit = CardSuit::Spades;
    second_card = CardSuit::Clubs;
    println!("{:#?}", second_card);

    let card_suits: [CardSuit; 2] = [CardSuit::Hearts, CardSuit::Spades];
    let card_suits: (CardSuit, CardSuit) = (CardSuit::Hearts, CardSuit::Clubs);
    println!("{:#?}", card_suits.0);
}