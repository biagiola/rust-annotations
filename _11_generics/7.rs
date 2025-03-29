// Generics and impl Blocks I

#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T
}

// impl only for TreasureChest with the type string
impl TreasureChest<String> {
    fn clean_treasure(&mut self) { // ownership is transfered
        self.treasure = self.treasure.trim().to_string()
    }
}

impl TreasureChest<[&str; 3]> { // only apply for three &str
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

fn main() {
    let mut silver_chest: TreasureChest<String> = TreasureChest {
        captain: String::from("firebread"),
        treasure: String::from("  Silver ")
    };
    silver_chest.clean_treasure();
    // println!("{:?}", silver_chest);

    let special_chest: TreasureChest<[&str; 3]> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: ["Gold", "Silver", "Platinum"]
    };
    // println!("{:?}", special_chest.amount_of_treasure());
    println!("{:#?}", special_chest);


}