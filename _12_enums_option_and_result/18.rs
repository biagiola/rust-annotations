// Rust prelude: It's a collection of named constructs that are
// available automatically in every program.
fn main() {
    let availability: Option<bool> = item_is_available();

    match availability {
        // we can call Option direclty in our program becasue of rust prelude
        // also if we remove and just use Some or None directly
        Some(true) => println!("Yes, the item is available"),
        Some(false) => println!("Yes, the item is available"),
        None => println!("Your item doesn't exists in our system")
    }
}

fn item_is_available() -> Option<bool> {
    let item_exists_in_catalog: bool = true;
    let item_is_in_stock: bool = false;

    if item_exists_in_catalog && item_is_in_stock {
        Some(true) // Option::Some(true)
    } else if item_exists_in_catalog {
        Some(false) // Option::Some(false)
    } else {
        None // Option::None
    }
}