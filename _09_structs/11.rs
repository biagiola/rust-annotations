// 3. Third way is using a ref to the Coffee struct instead of its instance.
// So by default, our parameter will be immutable.

// Same as the first option in file 9.rs but with a ref instead of a direct value
// to not lose ownership of the struct in main fn.

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

fn drink_coffee(coffee: &Coffee) {
    // in this example ownership is never moved. Mocha
    // variable is still the owner of its values. So
    // after invocation of drink_coffee fn we can still
    // use mocha fields in the main fn.

    // In this way, rust accesses the field from a reference.
    // This reference in the drink_coffee fn can access its
    // value by dereferencing it.
    // In the background, rust is putting an asterisk to do the dereference,
    // which means, take the value the reference is pointing to.
    // Whether we put an asterisk in front of a ref or not, we're doing
    // a dereference to get the real final value.
    // println!("Drinking my delicious {}.", coffee.name);
    println!("Drinking my delicious {}.", (*coffee).name);
    
    // For a beginner who is learning the language for the first time,
    // it's better to make the dereference manually just to keep in
    // mind the fundamentals behind the scenes.
}

fn main() {
    let mocha: Coffee = make_coffee(
        String::from("Latte"),
        4.99,
        true
    );
    drink_coffee(&mocha);
    println!("{:#?}", mocha); // we borrowed the struct, so we can still use it in the main fn
}