use fake::{Fake, Faker};
use warehouse::{
    Item,
    ProductCategory,
    FLOOR_SPACE,
    INVENTORY_MANAGER,
    ORDERS_MANAGER
};

/// Primary entrypoint int oour warehouse program
fn main() {
    let fake_item: Item = Faker.fake();
    println!("Fake Item: {:?}", fake_item);

    println!("This is our primary program");
    

}