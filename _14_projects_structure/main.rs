mod inventory;
mod orders;

use inventory::products::{Item, ProductCategory};
use inventory::{FLOOR_SPACE, MANAGER as INVENTORY_MANAGER};
use orders::MANAGER as ORDERS_MANAGER;

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        INVENTORY_MANAGER,
        ORDERS_MANAGE,
        FLOOR_SPACE
    );

    let favorite_category = ProductCategory::Hammer;
    println!("My fav category of item is {favorite_category:?}");

    let tall_lader = Item::new(
        String::from("Ladder-o-matic 2000"),
        favorite_category,
        100
    );
    println!("{:#?}", tall_lader);
}