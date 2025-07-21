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

/*
unwrap()
What it does: If the value is Some(v) or Ok(v), it gives you v. If it's None or Err, it panics with a generic, default error message.
When to use it: Mostly in prototypes, quick tests, or examples where you are absolutely certain the operation won't fail. It's simple to type, but its generic panic message can make debugging harder.

expect()
What it does: Exactly the same as unwrap(), but if it panics, it uses a custom message that you provide.
When to use it: In production-ready code. When the program panics, the custom message tells you exactly which assumption was violated and why you expected a value to be there. This is incredibly helpful for debugging.

*/