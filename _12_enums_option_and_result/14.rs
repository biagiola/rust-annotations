fn main() {
    let musical_instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drum"),
        String::from("Bass")
    ];

    let bass: Option<&String> = musical_instruments.get(2); // copy without transfering ownership
    println!("{:?}", bass);

    // in the case where we dont have the value to extract, we emit this message
    let valid_instrument: &String = bass.expect("Unable to retrieve element");
    println!("{valid_instrument}");

    let invalid_instrument: Option<&String> = musical_instruments.get(3);
    println!("{:?}", invalid_instrument);
    println!("{}", invalid_instrument.expect("Unable to retrieve musical instrument"));
}