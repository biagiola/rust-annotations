fn main() {
    // a heap variable
    let oranges: String = String::from("Orange");
    print_my_value(oranges);
}

fn print_my_value(value: String) {
    // println!("{:p}", &value);

    println!("Your value is {value}");
    // 'oranges' will move its ownership to 'value' and
    // 'value' deallocates itself from the heap and after finish fn
    // and going back to main, oranges is not longer the owner of anything.
    
    // Compiler suggest to call the fn in this away:
    // print_my_value(oranges.clone());
}