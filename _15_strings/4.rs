// format! helps us to create a new string, without taking ownership of the original variables.

fn main() {
    let first_name: String = String::from("Sylvester");
    let last_name: String = String::from("Stallone");

    // let icon: String = format!("{} {}", first_name, last_name); 
    // You can use both String and &str with format!
    let icon: String = format!("{first_name} {last_name}"); // using String variables
    let icon2: String = format!("{} {}", "Sylvester", "Stallone"); // using &str literals
    let icon3: String = format!("{} {}", &first_name, &last_name); // using &String (which derefs to &str)
    // print our new heap allocated string
    println!("{icon}");
    println!("{icon2}");
    println!("{icon3}");

    // they don't lose ownership
    println!("{first_name}");
    println!("{last_name}");

    //TODO: what happens if we use data types like structs or enums?
}