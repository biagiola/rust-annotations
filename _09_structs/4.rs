fn main() {
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool
    }

     // the mut affects all the fields
    let mut beverage: Coffee = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true
    };

    // Mocha String was deallocated by this new string
    beverage.name = String::from("Caramel Macchiato");
    println!("{}", beverage.name);
}