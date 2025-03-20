struct Coffee {
    price: f64,
    name: String,
    is_hot: bool
}

fn main() {
    // ownership for the name string variable is transfering,
    // in this case from the name variable to the name parameter
    // for make_coffee fn. And later, name parameter transafers
    // its owner to the name field of Coffe struct inside
    // make_coffee fn. And finally, the coffee.name becomes
    // the owner transfers.
    let name: String = String::from("Latte");
    let coffe: Coffee = make_coffee(name, 4.99, true);

    println!(
        "Name: {}, price: {}, is_hot: {}",
        coffee.name, coffee.price, coffee.is_hot
    );
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name: name,
        price: price,
        is_hot: is_hot
    }
}