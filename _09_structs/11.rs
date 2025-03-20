struct Coffee {
    price: f64,
    name: String,
    is_hot: bool
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffe {
    Coffe {
        name, price, is_hot
    }
}

// 3. Third way is using a ref to the Coffee struct instead of its instance.
// So by default, our parameter it will be immutable.
fn drink_coffee(coffee: &Coffee) {
    println!("Drinking my delicious {}", coffee.name);
    // in this example ownership is never moved. Mocha
    // variable is still the owner of its values. So
    // after invocation of drink_coffee fn we can still
    // use mocha fields in the main fn.

    // In this way, rust access to the field from a reference.
    // This reference in the drink_coffee fn can be access its
    // value dereference it. In background, rust is putting an
    // asterisk to do the dereference, which mean, take the value
    // the reference is keeping on. Whatever we put a asteric
    // in front of a ref or not, we're doing a dereference
    // to get the real final value.
    println!("Drinking my delicious {}.", (*coffee).name);
    
    // For a beginner who is learning for the first time the language
    // it's better to make the dereference manually just to keeping in
    // mind the fundamentals behind the scenes.
}
fn main() {
    let mocha: Coffee = make_coffee(String::from("Latte"), 4.99, true);
    drink_coffee(&mocha);
}