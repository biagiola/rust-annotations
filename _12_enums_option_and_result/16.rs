fn main() {
    let musical_instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drum"),
        String::from("Bass")
    ];

    let bass: Option<&String> = musical_instruments.get(2); // copy without transfering ownership
    play(bass);

    let invalid_instrument: Option<&String> = musical_instruments.get(3);
    play(invalid_instrument);
}

fn play(instrument_option: Option<&String>) {
    match instrument_option {
    Option::Some(instrument) => println!("Palying the {instrument}"),
    Option::None => println!("Singing with my voice"),
}
}