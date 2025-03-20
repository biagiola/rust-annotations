struct Coffee {
    price: f64,
    name: String,
    is_hot: bool
}

fn main() {
    let mocha: Coffee = make_coffee(String::from("Latte"), 4.99, true);

    let caramel_macchiato = Coffee {
        name: String::from("Caramel Macchiato"), // New heap allocation
        price: mocha.price, // copy trait
        is_hot: mocha.is_hot // copy trait
    };
    // with the two copy traits and a new String, we're also creating
    // a full copy of mocha.

    // another way to accomplish the same result as above,
    // if we have a lot of fields to copy, we can spread one
    // instance field to another with distruct update syntax
    let caramel_macchiato = Coffee {
        name: String::from("Caramel Macchiato"),
        ..mocha // copy only the rest values
    };

}

fn  make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot
    }
}

// for a full deep copy we use clone for the heap data we need it
// let caramel_macchiato = mocha.clone();