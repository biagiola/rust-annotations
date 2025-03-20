// now this struct is not only attached to the main scope
// but rather to the module or file scope
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool
}

fn main() {
    let coffee: Coffee = make_coffee(String::from("Latte"), 4.99, true);

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