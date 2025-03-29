// Generics and Impl II
#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T
}

impl TreasureChest<[&str; 2]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

impl<T> TreasureChest<T> { // we make sure that T is not coincidentally a concrete type with that name but rather a generic
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

// for example an enum that is called T
// enum T {
//     captain: String,
//     ship: &str
// }

fn main() {
    let gold_chest: TreasureChest<&str> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold"
    };
    // println("{}", gold_chest.capital_captain());

    let silver_chest: TreasureChest<[&str; 2]> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: ["Gold", "Silver"]
    };
    println!("{:#?}", silver_chest.capital_captain());
}