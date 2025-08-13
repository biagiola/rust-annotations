// Lecture: join method

fn main() {
    let address_portions = [
        String::from("123 Elm Street"),
        String::from("Suburbia"),
        String::from("New Jersey"),
    ];

    // similar what we do in previous lecture but more consice
    println!("{}", address_portions.join(", "));

    // Option
    let address = address_portions
        .into_iter()
        .reduce(|mut accumulator, portion| {
            accumulator.push_str(", ");
            accumulator.push_str(&portion);
            accumulator
        });
    println!("{address:?}");
}