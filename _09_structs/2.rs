fn main() {
    struct Coffe {
        price: f64,
        name: String,
        is_hot: bool
    }

    let mocha: Coffee = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true
    }

    println!(
        "My {} this morning cost {} and it is {} that it was hot.",
        mocha.name,
        mocha.price,
        mocha.is_hot
    );
}