pub mod products;
pub use self::products::{Item, ProductCategory}; // export the Item and ProductCategory direclty from the Inventory module

pub const FLOOR_SPACE: i32 = 10_000;
pub const MANAGER: &str = "Ivan Iventory";

pub fn talk_to_manager() {
    println!(
        "Hey, {}, how's your coffee? What do you think of {:?}",
        MANAGER,
        products::ProductCategory::Ladder
    );
}



