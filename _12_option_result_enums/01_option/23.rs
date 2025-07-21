// The unwrap and expect methods

fn main() {
    let instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drum"),
        String::from("Bass")
    ];

    // we dont need to put the & simbol coz get method returns a reference
    let bass: Option<&String> = instruments.get(2); // copy without transfering ownership
    println!("{:?}", bass);

    let valid_instrument: &String = bass.unwrap(); // also return a reference
    println!("{}", valid_instrument)

    //
    let invalid_instrument: Option<&String> = instruments.get(3);
    println!("{:?}", invalid_instrument);
    println!("example: {}", invalid_instrument.unwrap()); // dont' print this line

    // more naive approach to get the real value from the Option Enum but now let's use
    // the unwrap method which attempts to extract the associated data out to the Some variant.
}