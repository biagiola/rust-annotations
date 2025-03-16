fn main() {
    let person: String = String::from("David");

    // we can clone it to have both values on the heap
    let user: String = person.clone();

    // values
    println!("{person}");
    println!("{user}");

    // references
    println!("{:p}", &person);
    println!("{:p}", &user);
    
    // with the & we have its reference (memory address)
    // but for print it, we need to use the format specifier
    // for pointer {:p}, otherwise, it will print the value
    // at the address memory.
}