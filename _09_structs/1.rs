fn main() {
    // A struct is a container for related pieces of data.
    // It's similar to objects from POO.
 
    // In tuples the data is saved in order, and that is a disadvantage
    // here, we can infer that maybe 5.99 is the price of the coffe
    // but is not well structure as a key, value pair for example.
    // also, we don't care about order while they maitained together
    // in a single type is fine, hence we can make use of structs.
    let coffee: (&str, f64, bool) = ("Caramel Macchiato");

    // we have three king of structs:
    // Named field, Tuple-like, Unit-like.
    // and the most used is Named field
    
    // in structs we use pascal case
    struct CoffeDrink {
        price: f64,
        name: String,
        is_hot: bool
    }
}