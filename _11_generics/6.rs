// Generics in strucs
#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T
}

fn main() {
    let gold_chest: TreasureChest<&str> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold"
    };
    println!("{:?}", gold_chest);

    let silver_chest: TreasureChest<String> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: String::from("Silver")
    };
    println!("{:?}", silver_chest);

    let special_chest: TreasureChest<[&str; 3]> = TreasureChest {
        captain: String::from("firebeard"),
        treasure: ["Gold", "Silver", "Platinum"]
    };
    println!("{:?}", special_chest);
}