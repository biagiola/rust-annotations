fn main() {
    let apples: i32 = 6; // a stack variable
    
    // stack variables are primitives, does not have
    // a pointer representation like heap-allocated data
    // so, we use & for print its reference
    // println!("{:p}", &apples);
    print_my_value(apples);
    println!("{apples} is still valid");
}

fn print_my_value(value: i32) {
    // println!("{:p}", &value);

    println!("Your value is {value}");
    // here, we implement copy trait. Value is a new reference
    // and it's deallocate after finish its scope
}