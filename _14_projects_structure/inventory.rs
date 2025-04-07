pub const FLOOR_SPACE: i32 = 10_000;
pub const MANAGER: &str = "Ivan Iventory";

pub fn talk_to_manager() {
    println!("Hey, {MANAGER}, how's your coffee?")
}

pub mod products; // assume we created the products.rs file for the import

// 2.Option: inventory/products.rs
// 3.Option: inventory/products/mod.rs

// 1. Option one: inline
// submodule
// mod products {
    // #[derive(Debug)]
    // pub enum ProductCategory {
    //     Ladder,
    //     Hammer,
    // }
    
    // #[derive(Debug)]
    // pub struct Item {
    //     pub name: String,
    //     pub category: ProductCategory,
    //     pub quantity: u32,
    // }
// }

