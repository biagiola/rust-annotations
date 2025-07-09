fn main() {
    let airline = "United Airlines Flight";

    // this show an error about the type of the variable
    // let segments = airline.split(" ").collect();

    let segments: Vec<&str> = airline.split(" ").collect();
    println!("{:#?}", segments);

    // The answer "No" is correct because the collect method
    // requires a specified type to understand what kind of
    // collection to return. Without this type, the code will not compile.
}
