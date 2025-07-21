fn is_item_in_stock(
    item_is_in_system: bool,
    item_is_in_stock: bool
) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_is_in_stock {
        Option::Some(false)
    } else {
        Option::None
    }
}

fn main() {
    let availability: Option<bool> = is_item_in_stock(false, false);

    match availability {
        Option::Some(value) => println!("Item is available: {value}"),
        Option::None => println!("Your item doesn't exists in our system")
        // Option::Some(true) => println!("Yes, the item is available"),
        // Option::Some(false) => println!("Item is not in stock")
        // we can also have an arm for an specific value of the Option.
        // Just make sure you are considering all possible options.
    }
}