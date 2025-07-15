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
    println!("{:#?}", card_suits[0]);

    // shadowing with a new type and new values
    let card_suits: (CardSuit, CardSuit) = (CardSuit::Hearts, CardSuit::Clubs);
    println!("{:#?}", card_suits.0);
}

// Btw, the shadowing technique can be use to any rust data structure.
// let data = User {
//     id: 101,
//     username: String::from("alice"),
// };
// let data = vec!["user_data", "transformed", "now_a_vector"];
// let data = 42;
// println("{data}");