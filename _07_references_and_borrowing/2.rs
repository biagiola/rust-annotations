fn main() {
    let mut car: String = String::from("Red"); // 'car' is a mutable String

    let ref1: &mut String = &mut car;          // `ref1` is a mutable reference to `car`
    ref1.push_str(" and Blue");

    // create a new mutable reference
    // coming for the car again. ref2 will have
    // the previous modifications of ref1.
    let ref2: &mut String = &mut car;
    ref2.push_str("!!!!");

    // print the values
    println!("{ref2}");
    println!("{car}"); // still exists
    
    // but ref1 does not exists anymore, now ref2 is taking the place of ref1.
    // ref2 is immutable, so the mutable rule of ref1 and the value itsel is 'dropped'.
    // lifetime of ref1 is ended after ref2 mutable declaration and assignation.

    // In Rust, you cannot have multiple mutable references OR a mix of mutable and
    // immutable references at the same time.
}

// this is an advance feature in rust called lifetimes.
// Lifetimes basically means that a ref can essentially
// become invalid before the end of a give scope.
// So, that means ref1, if we dont use it in a println!
// its lifetimes ends after its declaration and 
// not at the end of the scope.

// you can either have many immutable references, or one mutable reference.

// Behind the scenes there is a line after the 'ref1.push_str(" and Blue");'
// Drop the mutable reference ref1 before creating ref2
// drop(ref1);