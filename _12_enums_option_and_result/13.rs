fn main() {
    let musical_instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drum"),
        String::from("Bass")
    ];

    let bass: Option<&String> = musical_instruments.get(2); // copy without transfering ownership
    println!("{:?}", bass);

    // let valid_instrument: &String = bass.unwrap();
    let valid_instrument: &String = bass.expect("Unabler to retrieve element");
    // in the case where we don't have the vlaue to extract, we emit a message
    println!("{}", valid_instrument);
}