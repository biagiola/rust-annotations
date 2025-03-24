// Nesting enums in enums
#[derive(Debug)]
enum Beans { Pinto, Black }

#[derive(Debug)]
enum Meat { Chicken, Steak }

#[derive(Debug)]
enum RestaurantItem {
    Burrito { meat: Meat, beans: Beans },
    Bowl { meat: Meat, beans: Beans },
    Vegan
}

// tuple style
fn main() {
    let lunch: RestaurantItem = RestaurantItem::Burrito {
        meat: Meat::Steak,
        beans: Beans::Pinto
    };
    let dinner: RestaurantItem = RestaurantItem::Bowl {
        meat: Meat::Chicken,
        beans: Beans::Black
    };
    let abandoned_meal: RestaurantItem = RestaurantItem::Vegan;
    println!("Launch: {lunch:#?}");
    println!("Dinner: {dinner:#?}");
    println!("Nobody ate: {abandoned_meal:#?}");
}