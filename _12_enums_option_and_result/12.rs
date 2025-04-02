fn main() {
    let musical_instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drum"),
        String::from("Bass")
    ];

    let bass: Option<&String> = musical_instruments.get(2); // copy without transfering ownership
    println!("{:?}", bass);

    let valid_instruments: &String = bass.unwrap();
    println!("{}", valid_instruments);

    // this will cause a panic at runtime
    // but we use options and everything is fine
    let invalid_instrument: Option<&String> = musical_instruments.get(3);
    println!("{:?}", invalid_instrument); // this prints Nones
    // println!("example: {}", invalid_instrument.unwrap()); // this line is not printed

    // more naive approach to get the real value from the Option enum is the get method but now let's use
    // the unwrap method which attempts to extract the associated data out of the same variant.
}