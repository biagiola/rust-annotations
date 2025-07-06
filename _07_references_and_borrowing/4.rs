fn main() {
    let mut coffe: String = String::from("Mocha");
    let a: &mut String = &mut coffe; // we're borrowing here
    // println!("{:p} {0}", &coffe); // coffe is still the owner.
    
    // but if we try a print at the same time between a and coffe we'll get conflicts
    // coffee is temporarily inaccessible while borrowed as a rust borrowing rule.

    println!("{:p} {0}", &a);

    // Let's modify coffe value through a
    a.push_str(" and Cookies");
    println!("{:p} {0}", &a);

    // Let's move the mutable reference from a to b
    let b: &mut String = a;
    b.push_str(". So delicious!");
    println!("{:p} {b}", &b);

    // we can have just one mutable reference at the same time
}

// The Three Different Concepts:
//
// 1. Ownership Transfer:
// let coffe: String = String::from("Mocha");
// let a: String = coffe; // ✅ OWNERSHIP TRANSFER
// coffe is no longer valid, a now owns the String
//
// 2. Borrowing:
// let coffe: String = String::from("Mocha");
// let a: &mut String = &mut coffe; // ✅ BORROWING
// coffe still owns, a borrows mutably from coffe
//
//3. Moving a Reference
// let a: &mut String = &mut coffe; // First: borrow from coffe
// let b: &mut String = a;          // Then: MOVE the reference from a to b
// Now: a is invalid, b holds the mutable reference, coffe still owns
