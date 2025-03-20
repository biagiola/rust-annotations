#[derive(Debug)]

struct Coffee {
    price: f64,
    name: String,
    is_hot: bool
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name, price, is_hot
    }
}

// 4. Fourth way: it's to use our parameter as a mutable ref to
// the Coffee instance o struct. Allowing to  modify the original
// variable mocha. Ownership is not moving but the coffee param is
// access to modify it.
fn drink_coffee(coffee: &mut Coffee) {
    println!("Drinking my delicious {}", coffee.name);
    coffee.is_hot = false; // it will affected in main fn
}

fn main() {
    let mut mocha: Coffee = make_coffee(String::from("Latte"), 4.99, true);
    drink_coffee(&mut mocha);
    // println!("{}", mocha.is_hot);
    println!("{:?}", mocha);
}
