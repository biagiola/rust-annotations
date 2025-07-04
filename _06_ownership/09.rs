fn main() {
    let person: String = String::from("David");

    // we can clone it to have both values on the heap
    // Note: clone() has a performance cost - it allocates new heap memory and copies the data
    let user: String = person.clone();

    // values
    println!("{person}");
    println!("{user}");

    // references - these show the stack addresses of the variables themselves
    println!("{:p}", &person);
    println!("{:p}", &user);
    
    // with the & we get the stack address where each variable is stored
    // {:p} format specifier prints these stack addresses as pointers
    // Note: this shows where the String structs are on the stack,
    // not where the actual string data is stored on the heap
}
