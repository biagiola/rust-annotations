use fake::{Fake, Faker};
use warehouse::{
    Item,
    ProductCategory,
    FLOOR_SPACE,
    INVENTORY_MANAGER,
    ORDERS_MANAGER
};

fn main() {
    println!(
        "Main: Our managers are {} and {}. We have {} square feet of floor space",
        INVENTORY_MANAGER,
        ORDERS_MANAGER,
        FLOOR_SPACE
    );

    let fake_item: Item = Faker.fake();
    println!("Fake Item: {:?}", fake_item);

    let random_category: ProductCategory = Faker.fake();
    println!("Random Category: {:?}", random_category);
}