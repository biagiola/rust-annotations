#[derive(Debug)]

struct Coffee {
    price: f64,
    name: String,
    is_hot: bool
}

fn main() {
    let mocha: Coffee = make_coffee(String::from("Latte"), 4.99, true);
    drink_coffee(mocha);
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee { name, price, is_hot }
}

// 2. Second way is for mutable parameters
fn drink_coffee(mut coffee: Coffee) {
    println!("Drinking my delicios {}", coffee.name);
    coffee.is_hot = false;
    // we're still moving ownership from outside mocha variable
    // to coffee fn parameter, but now it is a mutable variable
}


// Just as a side note: What happends if we want to use mocha
// variable again in the main fn?
// Actually, drink_coffee fn doesn't need ownership of mocha
// to print the value, but we also need make a print again in
// the fn function.
// So, continue with our site note question about ownership, the
// first way we can achieve this is using the clone method,
// that's give us a full copy or copy trait of the heap data,
// which is expensive option and another option will be use mutable reference.
// One example of that is this:

// In drink_coffee fn:
// fn drink_coffee(coffee: &mut Coffee) {
//     println!("Drinking my delicious {}", coffee.name);
//     coffee.is_hot = false; // Mutating the borrowed Coffee

// In main fn: 
// drink_coffee(&mut mocha); // pass a mutable reference:
// println!("{:?}", mocha);  // now this works!

// We have more info about the 'value borrowed here after move' error
// with this command: rustc --explain E0382
// and also we're talk more about these topics in third and fourth examples.