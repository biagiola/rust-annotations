fn main() {
    let first_name: String = String::from("Sylvester");
    let last_name: String = String::from("Stallone");

    // let icon: String = format!("{} {}", first_name, last_name); 
    let icon: String = format!("{first_name} {last_name}"); //  we're not taking ownership or either of the variables
    // print our new heap allocated string
    println!("{icon}");

    // they're not loose ownership
    println!("{first_name}");
    println!("{last_name}");

    //TODO: what happens if we use data types like structs or enums?
}