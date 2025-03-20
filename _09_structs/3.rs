fn main() {
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool
    }

    // An instance is the concrete value made from a type
    let mocha: Coffee = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true
    };

    // with heap type data, that does not implements the copy trait,
    // we move the ownership and the variable is the new owner 
    // not the name field of the struct.
    let favorite_coffee: String = mocha.name;
    println!("{0}", favorite_coffee);
    // println!("{0}", mocha.name); // the name field of the struct is not longer the owner
    println!("{0}", mocha.price); // but the price still do.

    // at the end of the day, a struct is the owner of its fiels, and
    // each name field is the owner of each corresponding value.    
}