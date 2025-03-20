struct Coffe {
    price: f64,
    name: String,
    is_hot: bool
}

// From here to the next lectures, we will see four ways to work around
// structs as function parameters 

// we can pass structs as parameters to a fn in four different ways
// 1. First way: passing a struct as immutable
fn main() {
    let mocha: Coffee = make_coffee(String::from("Latte"), 4.99, true);
    drink_coffee(mocha);

    // we cannot use mocha.name anymore
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name, price, is_hot
    }
}

// here ownership is moving from mocha variable to the coffee parameter
fn drink_coffee(coffee: Coffee) {
    println!("Drinking my delicious: {}", coffee.name)
}