// Lecture: Fn traits closures

fn main() {
    let multiplier = 5; // technically a u8 type

    // for this closure we're implementing the Fn trait
    // coz u8 implements the copy trait.
    let multiply_by = |value| value * multiplier;
    println!("{}", multiply_by(3 as u8));

    // the trait for multiply_by is Fn because just read values for the ouside environment,
    // in this case, it read a stack variable u8.

    // now same thing with heap data for Fn traits coz we're only reading variables. 
    let numbers = vec![4, 8, 15, 16, 23, 42];

    // it does not need to borrow an mutable reference, thus
    // an immutable reference is sufficient.
    let print_numbers = || println!("{:?}", numbers);
    print_numbers();

    // So we can invoke it again or print numbers by itself 
    print_numbers();
    println!("{:?}", numbers);

    // For closures that either borrow immutable references or
    // do not borrow anything at all, the compiler will infer the fn trait
}