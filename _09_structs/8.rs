struct Coffee {
    price: f64,
    name: i32,
    is_hot: bool
}

fn main() {
    // let mocha: Coffee = make_coffee(String::from("Latte"), 4.99, true);
    let mocha: Coffee = make_coffee(1, 4.99, true);

    // we move ownership from mocha to caramel_macchiato
    let caramel_macchiato = Coffee { ..mocha };
    println!("Name: {}", caramel_macchiato.name);
    println!("Name: {}", mocha.name);
    // error: borrow moved value if we use name as String as before
    // but i32 can implement copy trait as well as the other f64 and bool values

}

fn  make_coffee(name: i32, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot
    }
}
