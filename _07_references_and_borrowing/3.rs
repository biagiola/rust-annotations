fn main() {
    // three of them have its own reference in the stack
    let coffe: String = String::from("Mocha");
    let a: &String = &coffe;
    let b: &String = a;

    println!("{:p} {}", &coffe, coffe);
    println!("{:p} {}", &a, a);
    println!("{:p} {}", &b, b);
}

// 'coffe' variable is saved on the stack as a pointer to the data.
// We only have one data on the heap that allocates the raw 'Mocha' string
// where 'coffe' variable points to that memory address or reference.
// Both a and b are references (borrowed pointer) to 'coffe'. So they are
// stored on the stack.

// a n b has thier own unique address on the stack, but both hold the same
// information, that is the address of coffe variable.