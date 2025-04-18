use fake::Dummy;

/// A category of product that our business sells
#[derive(Debug, Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}

/// A concrete item in stock within our warehouse
#[derive(Debug, Dummy)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}

impl Item {
    /// Create a new Item
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        super::talk_to_manager(); 
        Self {
            name,
            category,
            quantity
        }
    }
}

// everything that is a child can reach up until its parrent level regarless of
// whether the pub keyword is present.