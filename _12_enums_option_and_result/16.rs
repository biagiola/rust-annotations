fn main() {
    let musical_instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drum"),
        String::from("Bass")
    ];

    let bass: Option<&String> = musical_instruments.get(2);
    play(bass); // pass a reference

    println!("{bass:?}"); // still exists

    let invalid_instrument: Option<&String> = musical_instruments.get(3);
    play(invalid_instrument);
}

fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Option::Some(instrument) => println!("Palying the {instrument}"),
        Option::None => println!("Singing with my voice"),
    }
}