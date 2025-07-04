fn main() {
    // a heap variable
    let oranges: String = String::from("Orange");
    println!("{:p}", &oranges);

    print_my_value(oranges);
    // println!("{}", oranges); // we cannot use oranges anymore
}

fn print_my_value(value: String) {
    println!("{:p}", &value);

    println!("Your value is {value}");
    // When we call the function, ownership moves from 'oranges' to 'value'
    // 'value' deallocates the heap memory when this function finishes
    // and back in main, 'oranges' is no longer the owner of anything.
    
    // Compiler suggests to call the fn this way to keep oranges usable:
    // print_my_value(oranges.clone());
}