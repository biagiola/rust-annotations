pub mod products;
pub const FLOOR_SPACE: i32 = 10_000;
pub const MANAGER: &str = "Ivan Iventory";

pub fn talk_to_manager() {
    println!("Hey, {}, how's your coffee?", MANAGER); // relative path
    // println!("Hey, {}, how's your coffee?", crate::inventory::MANAGER); // but just to know how to use variables using absolute path
    // println!("{:?}", products::ProductCategory::Ladder); // relative path
    // println!("{:?}", crate::inventory::products::ProductCategory::Ladder); // absolute path
}

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

