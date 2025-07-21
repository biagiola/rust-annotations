// The Option enum models a sceneario
// where a type could be a valid value
// or nothing at all.
 
// possible values
// Option::None represents an absent value.
// Option::Some(T) represents a present value.
 
fn main() {
    let musical_instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drum"),
        String::from("Bass")
    ];

    // let bass: Option<String> = musical_instruments[2]; // we cannot do this coz bass will be the onwer of that particualr value automatically when we make the assignation
    let bass: Option<String> = Some(musical_instruments[2].clone()); // but instead we can clone an give it to the Some method
    println!("{:?}", bass);

    // now also, we can do this using references, and print it as we normally do
    // an we still see the the value and not the reference due the way print works in rust
    // that derefence to see the actual value
    let bass: Option<&String> = Some(&musical_instruments[2]);
    println!("{:?}", bass);

    // also the same if we want to print only the string
    let bass: &String = &musical_instruments[2];
    println!("{bass}");

    // The best approach is using the get method
    let bass: Option<&String> = musical_instruments.get(2);
    println!("{:?}", bass);

    // Using .get() allows to us to handle better out of bounds panic
    // when this isn't a value at some index position, throwing us a Option::None instead.
}