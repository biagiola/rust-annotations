// Nesteing enums in enums
#[derive(Debug)]
enum Meat {
    Chicken,
    Steak
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito(Meat),
    Bowl(Meat),
    VeganPlate
}

// tuple style
fn main() {
    let lunch: RestaurantItem = RestaurantItem::Burrito(Meat::Steak);
    let dinner: RestaurantItem = RestaurantItem::Bowl(Meat::Chicken);
    let abandoned_meal: RestaurantItem = RestaurantItem::VeganPlate;
    println!("Lunch was {lunch:?} and dinner was {dinner:?}");
    println!("Nobody ate {abandoned_meal:?}");
}
