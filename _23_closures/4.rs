fn main() {
    let multiplier = 5; // technically a u8 type

    // for this closure we're implementing the Fn trait
    // coz u8 implements the copy trait.
    let multiply_by = |value| value * multiplier;
    println!("{}", multiply_by(3 as u8));

    // now vectors do not implement the copy trait
    let numbers = vec![4, 8, 15, 16, 23, 42];
    // println!("{:?}", numbers);

    // but we only make a print, we don't change anything,
    // it does not need to borrow an mutable reference, thus
    // an immutable reference is sufficient
    let print_numbers = || println!("{:?}", numbers);
    print_numbers();

    // So we can invoke it again or print numbers by itself 
    print_numbers();
    println!("{:?}", numbers);

    // For closures that either borrow immutable references or
    // do not borrow anything at all, the compiler will infer the fn trait
}