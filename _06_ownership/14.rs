fn main() {
    let apples: i32 = 6; // a stack variable
    
    // stack variables are primitives, do not have
    // a pointer representation like heap-allocated data
    // so, we use & to print its reference
    println!("{:p}", &apples);
    print_my_value(apples);
    println!("{apples} is still valid");
}

fn print_my_value(value: i32) {
    println!("{:p}", &value);

    println!("Your value is {value}");
    // here, the Copy trait is automatically used. 'value' is a new copy
    // of the original data, and it gets deallocated when this scope ends
}
