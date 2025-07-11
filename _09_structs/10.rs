// 2. Second way: passing a struct as mutable parameter directly losing ownership

#[derive(Debug)]
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee { name, price, is_hot }
}

// here ownership is moving from mocha variable to the coffee parameter
fn drink_coffee(mut coffee: Coffee) {
    println!("Drinking my delicious {}", coffee.name);
    coffee.is_hot = false;
    // we're still moving ownership from outside 'mocha' variable
    // to 'coffee' fn parameter, but now it's mutable so we can modify
    // the struct fields before leaving the fn and it will be reflected
    // in the main fn.
}

fn main() {
    let mocha: Coffee = make_coffee(
        String::from("Latte"),
        4.99,
        true
    );
    drink_coffee(mocha);

    // println!("{:?}", mocha); // again, after the fn call, mocha is not available
}

// What happens if we want to use 'mocha' variable again in the main fn?
// Actually, drink_coffee fn doesn't need ownership of 'mocha' to print the value,
// and let's assume we need to make a print again in the main function.
// So, first way we can achieve this is using the clone method, that gives us
// a full copy or copy trait of the heap data, which is an expensive option.

// Another option will be to use mutable reference. With that option we can
// modify the struct fields before leaving the fn and it will be reflected
// in the main fn. This we'll see in the fourth example in file 12.rs
