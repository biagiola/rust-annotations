fn main() {
    let registrations: [bool; 3] = [true, false, true];

    let mut first: bool = registrations[0];
    println!("{first}");

    // first is a copy trait, so we can modify as we please
    first = false;
    println!("{first} and {registrations:?}")
}