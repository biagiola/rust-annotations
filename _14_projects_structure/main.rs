mod inventory;
mod orders;

use inventory::products::{Item, ProductCategory};
// use inventory::products::Item;
use inventory::{FLOOR_SPACE, MANAGER, talk_to_manager};

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        MANAGER,
        orders::MANAGER,
        FLOOR_SPACE
    );

    talk_to_manager();

    let favorite_category = ProductCategory::Hammer;
    println!("My fav category of item is {favorite_category:?}");

    let tall_lader = Item {
        name: String::from("Ladder-o-matic 2000"),
        category: favorite_category,
        quantity: 100
    };
    println!("{:#?}", tall_lader);
}