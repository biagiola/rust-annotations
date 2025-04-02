fn main() {
    let musical_instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drum"),
        String::from("Bass")
    ];
    
    let bass: Option<&String> = musical_instruments.get(2); // copy without transfering ownership
    match bass {
        Option::Some(value) => println!("Playing the {value}"),
        Option::None => println!("Singing with my voice")
    }

    let invalid_instrument: Option<&String> = musical_instruments.get(3);
    match invalid_instrument {
        Option::Some(instrument) => println!("Palying the {instrument}"),
        Option::None => println!("Singing with my voice"),
    }
}