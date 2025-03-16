fn main() {
    // this is an advance feature in rust called lifetimes.
    // Lifetimes basically means that a ref can essentially
    // become invalid before the end of a give scope.
    // So, that means ref1, if we dont use it in a println!
    // its lifetimes ends after its declaration and 
    // not at the end of the scope.
    
    let mut car: String = String::from("Red");
    
    // same as previous example
    let ref1: &mut String = &mut car;
    
    // but now we make a mutation
    ref1.push_str(" and silver");
    
    // create again a second ref
    let ref2: &String = &car;
 
    // and its totally fine again, coz the lifetime
    // of ref1 that ends after its push_str in this case
    println!("{ref2}");
    println!("{car}");
    println!("{ref1}");

    // but we cannot use ref1 anymore. It was already borrowed as mutable by the f
    // println!("{ref1}");
    
    // it would be different if we declare ref2 before
    // the push_str of ref1 and the declaration of ref1.
 
    // and of course we can't have two mut ref strings at 
    // the same time and trying to use them in a println
}