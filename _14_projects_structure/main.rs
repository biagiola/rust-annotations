mod inventory;
mod orders;

// use inventory::products;
// use inventory::products{ProductCategory};
// use inventory::products::{self, ProductCategory}; // same as two line above

// use inventory::products::Item;
use inventory::products::{Item, ProductCategory};
use inventory::{FLOOR_SPACE};

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        inventory::MANAGER,
        orders::MANAGER,
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