// Real example of Option Enum (The get method on an array)

fn main() {
    let instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drum"),
        String::from("Bass")
    ];

    let bass: Option<&String> = instruments.get(2); // copy without transfering ownership
    println!("{:?}", bass);

    // this will cause a panic at runtime,
    // but we use options and everything is fine
    let invalid_instrument: Option<&String> = instruments.get(3);
    println!("{:?}", invalid_instrument);
}