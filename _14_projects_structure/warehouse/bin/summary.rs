use warehouse::{
    FLOOR_SPACE,
    INVENTORY_MANAGER,
    ORDERS_MANAGER
}

fn main() {
    println!(
        "Our manager {} and {}. We have {} square feet of space",
        INVENTORY_MANAGER,
        ORDERS_MANAGER,
        FLOOR_SPACE
    );
}