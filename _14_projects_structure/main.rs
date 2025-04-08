mod inventory;
mod orders;

use fake::{Fake, Faker};

use inventory::{
    Item,
    ProductCategory,
    FLOOR_SPACE,
    MANAGER as INVENTORY_MANAGER
};
use orders::MANAGER as ORDERS_MANAGER;

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        INVENTORY_MANAGER,
        ORDERS_MANAGER,
        FLOOR_SPACE
    );

    let fake_item: Item = Faker.fake();
    println!("{:?}", fake_item);
}